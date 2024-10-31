//! This module implements all attacks as tasks to be spawned with `tokio::spawn`
//!
//! To start any attack create an [`AttackContext`] ([give it a leech](AttackContext::leech))
//! and call your desired attack method.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use chrono::{NaiveDateTime, TimeZone, Utc};
use dehashed_rs::{DehashedError, Query, ScheduledRequest, SearchResult};
use futures::StreamExt;
use ipnetwork::IpNetwork;
use log::{debug, error, warn};
use rorm::prelude::*;
use rorm::{and, insert, query, update, Database};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tonic::transport::Channel;
use uuid::Uuid;

#[cfg(doc)]
use crate::api::handler;
use crate::chan::{CertificateTransparencyEntry, WsManagerChan, WsManagerMessage, WsMessage};
use crate::models::{
    Attack, BruteforceSubdomainsResult, BruteforceSubdomainsResultInsert,
    CertificateTransparencyResultInsert, CertificateTransparencyValueNameInsert,
    DehashedQueryResultInsert, DnsRecordType, Host, HostInsert, OsType, Port, PortInsert,
    PortProtocol, TcpPortScanResult, TcpPortScanResultInsert,
};
use crate::rpc::rpc_definitions;
use crate::rpc::rpc_definitions::req_attack_service_client::ReqAttackServiceClient;
use crate::rpc::rpc_definitions::shared::dns_record::Record;

/// Common data required to start any attack
#[derive(Clone)]
pub struct AttackContext {
    /// Handle to the database to insert results into
    pub db: Database,

    /// Handle to send status updates over websocket
    pub ws_manager: WsManagerChan,

    /// The user starting the attack
    pub user_uuid: Uuid,

    /// The workspace the attack is started in
    pub workspace_uuid: Uuid,

    /// The attack's uuid
    pub attack_uuid: Uuid,
}

impl AttackContext {
    /// Add a leech to the context
    pub fn leech(self, leech: ReqAttackServiceClient<Channel>) -> LeechAttackContext {
        LeechAttackContext {
            common: self,
            leech,
        }
    }

    /// Query the [dehashed](https://dehashed.com/) API.
    ///
    /// See [`handler::attacks::query_dehashed`] for more information.
    pub async fn query_dehashed(self, sender: mpsc::Sender<ScheduledRequest>, query: Query) {
        let (tx, rx) = oneshot::channel::<Result<SearchResult, DehashedError>>();

        if let Err(err) = sender.send(ScheduledRequest::new(query, tx)).await {
            error!("Couldn't send to dehashed scheduler: {err}");
            return;
        }

        let res = match rx.await {
            Err(err) => {
                error!("Error waiting for result: {err}");
                return;
            }
            Ok(Err(err)) => {
                error!("Error while using dehashed: {err}");
                return;
            }
            Ok(Ok(res)) => res,
        };

        let entries: Vec<_> = res
            .entries
            .into_iter()
            .map(|x| DehashedQueryResultInsert {
                uuid: Uuid::new_v4(),
                dehashed_id: x.id as i64,
                username: x.username,
                name: x.name,
                email: x.email,
                password: x.password,
                hashed_password: x.hashed_password,
                database_name: x.database_name,
                address: x.address,
                phone: x.phone,
                vin: x.vin,
                ip_address: x.ip_address.map(IpNetwork::from),
                attack: ForeignModelByField::Key(self.attack_uuid),
            })
            .collect();

        if let Err(err) = insert!(&self.db, DehashedQueryResultInsert)
            .bulk(&entries)
            .await
        {
            error!("Database error: {err}");
            return;
        }

        self.send_finished(true).await;
    }
}

/// Common data required to start attacks on a leech
#[derive(Clone)]
pub struct LeechAttackContext {
    /// Common data required to start any attack
    pub common: AttackContext,

    /// Client for talking with the leech
    pub leech: ReqAttackServiceClient<Channel>,
}

impl LeechAttackContext {
    /// Bruteforce subdomains through a DNS wordlist attack
    ///
    /// See [`handler::attacks::bruteforce_subdomains`] for more information.
    pub async fn bruteforce_subdomains(mut self, req: rpc_definitions::BruteforceSubdomainRequest) {
        match self.leech.bruteforce_subdomains(req).await {
            Ok(v) => {
                let mut stream = v.into_inner();

                while let Some(res) = stream.next().await {
                    match res {
                        Ok(v) => {
                            let Some(record) = v.record else {
                                warn!("Missing field record in grpc response of bruteforce subdomains");
                                continue;
                            };
                            let Some(record) = record.record else {
                                warn!("Missing field record.record in grpc response of bruteforce subdomains");
                                continue;
                            };

                            let source;
                            let destination;
                            let dns_record_type;
                            match record {
                                Record::A(a_rec) => {
                                    let Some(to) = a_rec.to else {
                                        warn!("Missing field record.record.a.to in grpc response of bruteforce subdomains");
                                        continue;
                                    };
                                    source = a_rec.source;
                                    destination = Ipv4Addr::from(to).to_string();
                                    dns_record_type = DnsRecordType::A;
                                }
                                Record::Aaaa(aaaa_rec) => {
                                    let Some(to) = aaaa_rec.to else {
                                        warn!("Missing field record.record.aaaa.to in grpc response of bruteforce subdomains");
                                        continue;
                                    };
                                    source = aaaa_rec.source;
                                    destination = Ipv6Addr::from(to).to_string();
                                    dns_record_type = DnsRecordType::Aaaa;
                                }
                                Record::Cname(cname_rec) => {
                                    source = cname_rec.source;
                                    destination = cname_rec.to;
                                    dns_record_type = DnsRecordType::Cname;
                                }
                            };

                            let Ok(None) = query!(&self.db, BruteforceSubdomainsResult)
                                .condition(and!(
                                    BruteforceSubdomainsResult::F
                                        .attack
                                        .equals(self.attack_uuid),
                                    BruteforceSubdomainsResult::F
                                        .dns_record_type
                                        .equals(dns_record_type.clone()),
                                    BruteforceSubdomainsResult::F.source.equals(&source),
                                    BruteforceSubdomainsResult::F
                                        .destination
                                        .equals(&destination)
                                ))
                                .optional()
                                .await
                            else {
                                debug!("entry already exists");
                                continue;
                            };

                            if let Err(err) = insert!(&self.db, BruteforceSubdomainsResult)
                                .single(&BruteforceSubdomainsResultInsert {
                                    uuid: Uuid::new_v4(),
                                    attack: ForeignModelByField::Key(self.attack_uuid),
                                    dns_record_type,
                                    source: source.clone(),
                                    destination: destination.clone(),
                                })
                                .await
                            {
                                error!("Could not insert data in db: {err}");
                                return;
                            };

                            self.send_ws(WsMessage::BruteforceSubdomainsResult {
                                attack_uuid: self.attack_uuid,
                                source,
                                destination,
                            })
                            .await;
                        }
                        Err(err) => {
                            error!("Error while reading from stream: {err}");
                            self.send_finished(false).await;
                            return;
                        }
                    }
                }
            }
            Err(err) => {
                error!("Error while reading from stream: {err}");
                self.send_finished(false).await;
                return;
            }
        };

        if let Err(err) = update!(&self.db, Attack)
            .condition(Attack::F.uuid.equals(self.attack_uuid))
            .set(Attack::F.finished_at, Some(Utc::now()))
            .exec()
            .await
        {
            error!("Database error: {err}");
        }

        self.send_finished(true).await;
    }

    /// Start a tcp port scan
    ///
    /// See [`handler::attacks::scan_tcp_ports`] for more information.
    pub async fn tcp_port_scan(mut self, req: rpc_definitions::TcpPortScanRequest) {
        async fn insert_result(
            db: &Database,
            workspace_uuid: Uuid,
            attack_uuid: Uuid,
            ip_addr: IpNetwork,
            port_num: u16,
        ) -> Result<(), rorm::Error> {
            insert!(db, TcpPortScanResult)
                .return_nothing()
                .single(&TcpPortScanResultInsert {
                    uuid: Uuid::new_v4(),
                    attack: ForeignModelByField::Key(attack_uuid),
                    address: ip_addr,
                    port: port_num as i32,
                })
                .await?;

            let mut tx = db.start_transaction().await?;
            let host = query!(&mut tx, (Host::F.uuid,))
                .condition(and![
                    Host::F.ip_addr.equals(ip_addr),
                    Host::F.workspace.equals(workspace_uuid)
                ])
                .optional()
                .await?;

            let host_uuid = if let Some((uuid,)) = host {
                uuid
            } else {
                insert!(&mut tx, HostInsert)
                    .return_primary_key()
                    .single(&HostInsert {
                        uuid: Uuid::new_v4(),
                        ip_addr,
                        os_type: OsType::Unknown,
                        response_time: None,
                        comment: String::new(),
                        workspace: ForeignModelByField::Key(workspace_uuid),
                    })
                    .await?
            };

            let port = query!(&mut tx, Port)
                .condition(and![
                    Port::F
                        .port
                        .equals(i16::from_ne_bytes(port_num.to_ne_bytes())),
                    Port::F.protocol.equals(PortProtocol::Tcp),
                    Port::F.host.equals(host_uuid),
                    Port::F.workspace.equals(workspace_uuid),
                ])
                .optional()
                .await?;
            if port.is_none() {
                insert!(&mut tx, PortInsert)
                    .return_nothing()
                    .single(&PortInsert {
                        uuid: Uuid::new_v4(),
                        port: i16::from_ne_bytes(port_num.to_ne_bytes()),
                        protocol: PortProtocol::Tcp,
                        host: ForeignModelByField::Key(host_uuid),
                        comment: String::new(),
                        workspace: ForeignModelByField::Key(workspace_uuid),
                    })
                    .await?;
            }
            tx.commit().await?;

            Ok(())
        }

        match self.leech.run_tcp_port_scan(req).await {
            Ok(v) => {
                let mut stream = v.into_inner();

                while let Some(res) = stream.next().await {
                    match res {
                        Ok(v) => {
                            let Some(addr) = v.address else {
                                warn!("Missing field address in grpc response of scan tcp ports");
                                continue;
                            };

                            let Some(addr) = addr.address else {
                                warn!("Missing field address.address in grpc response of scan tcp ports");
                                continue;
                            };

                            let address = match addr {
                                rpc_definitions::shared::address::Address::Ipv4(addr) => {
                                    IpAddr::V4(addr.into())
                                }

                                rpc_definitions::shared::address::Address::Ipv6(addr) => {
                                    IpAddr::V6(addr.into())
                                }
                            };

                            if let Err(err) = insert_result(
                                &self.db,
                                self.workspace_uuid,
                                self.attack_uuid,
                                IpNetwork::from(address),
                                v.port as u16,
                            )
                            .await
                            {
                                error!("Database error: {err}");
                            }

                            self.send_ws(WsMessage::ScanTcpPortsResult {
                                attack_uuid: self.attack_uuid,
                                address: address.to_string(),
                                port: v.port as u16,
                            })
                            .await;
                        }
                        Err(err) => {
                            error!("Error while reading from stream: {err}");
                            self.send_finished(false).await;
                            return;
                        }
                    }
                }
            }
            Err(err) => {
                error!("Error while reading from stream: {err}");
                self.send_finished(false).await;
                return;
            }
        };

        if let Err(err) = update!(&self.db, Attack)
            .condition(Attack::F.uuid.equals(self.attack_uuid))
            .set(Attack::F.finished_at, Some(Utc::now()))
            .exec()
            .await
        {
            error!("Database error: {err}");
        }

        self.send_finished(true).await;
    }

    /// Query a certificate transparency log collector.
    ///
    /// See [`handler::attacks::query_certificate_transparency`] for more information.
    pub async fn query_certificate_transparency(
        mut self,
        req: rpc_definitions::CertificateTransparencyRequest,
    ) {
        match self.leech.query_certificate_transparency(req).await {
            Ok(res) => {
                let res = res.into_inner();

                let mut tx = self.db.start_transaction().await.unwrap();

                for cert_entry in &res.entries {
                    let cert_uuid = insert!(&mut tx, CertificateTransparencyResultInsert)
                        .return_primary_key()
                        .single(&CertificateTransparencyResultInsert {
                            uuid: Uuid::new_v4(),
                            created_at: Utc::now(),
                            attack: ForeignModelByField::Key(self.attack_uuid),
                            issuer_name: cert_entry.issuer_name.clone(),
                            serial_number: cert_entry.serial_number.clone(),
                            common_name: cert_entry.common_name.clone(),
                            not_before: cert_entry.not_before.clone().map(|x| {
                                Utc.from_utc_datetime(
                                    &NaiveDateTime::from_timestamp_millis(x.seconds * 1000)
                                        .unwrap(),
                                )
                            }),
                            not_after: cert_entry.not_after.clone().map(|x| {
                                Utc.from_utc_datetime(
                                    &NaiveDateTime::from_timestamp_millis(x.seconds * 1000)
                                        .unwrap(),
                                )
                            }),
                        })
                        .await
                        .unwrap();

                    let value_names: Vec<_> = cert_entry
                        .value_names
                        .clone()
                        .into_iter()
                        .map(|x| CertificateTransparencyValueNameInsert {
                            uuid: Uuid::new_v4(),
                            value_name: x,
                            ct_result: ForeignModelByField::Key(cert_uuid),
                        })
                        .collect();

                    insert!(&mut tx, CertificateTransparencyValueNameInsert)
                        .return_nothing()
                        .bulk(&value_names)
                        .await
                        .unwrap();
                }

                tx.commit().await.unwrap();

                self.send_ws(WsMessage::CertificateTransparencyResult {
                    attack_uuid: self.attack_uuid,
                    entries: res
                        .entries
                        .into_iter()
                        .map(|e| CertificateTransparencyEntry {
                            serial_number: e.serial_number,
                            issuer_name: e.issuer_name,
                            common_name: e.common_name,
                            value_names: e.value_names,
                            not_before: e.not_before.map(|ts| {
                                Utc.from_utc_datetime(
                                    &NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32)
                                        .unwrap(),
                                )
                            }),
                            not_after: e.not_after.map(|ts| {
                                Utc.from_utc_datetime(
                                    &NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32)
                                        .unwrap(),
                                )
                            }),
                        })
                        .collect(),
                })
                .await;
            }
            Err(err) => {
                error!("Error while reading from stream: {err}");
                self.send_finished(false).await;
                return;
            }
        }

        if let Err(err) = update!(&self.db, Attack)
            .condition(Attack::F.uuid.equals(self.attack_uuid))
            .set(Attack::F.finished_at, Some(Utc::now()))
            .exec()
            .await
        {
            error!("Database error: {err}");
        }

        self.send_finished(true).await;
    }
}

/* Some utility methods and impls */
impl AttackContext {
    async fn send_ws(&self, message: WsMessage) {
        if self
            .ws_manager
            .send(WsManagerMessage::Message(self.user_uuid, message))
            .await
            .is_err()
        {
            error!("Couldn't send websocket message, the websocket manager died!");
        }
    }

    async fn send_finished(&self, finished_successful: bool) {
        self.send_ws(WsMessage::AttackFinished {
            attack_uuid: self.attack_uuid,
            finished_successful,
        })
        .await;
    }
}
impl std::ops::Deref for LeechAttackContext {
    type Target = AttackContext;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}
impl std::ops::DerefMut for LeechAttackContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}