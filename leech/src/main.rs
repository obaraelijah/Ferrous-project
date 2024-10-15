//! # Leeches
//! Leeches are the workers of ferrous.
//!
//! They provide a gRPC server to receive requests from ferrous and respond with results.
//! If this connection is lost somehow, they will store the results in a local database
//! and will try to connect to the ferrous gRPC server to send the missing data.
//!
//! You can also use the leech as a cli utility without a ferrous attached for manual
//! execution and testing. See the subcommand `run` for further information.
#![warn(missing_docs)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]

use std::env;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use chrono::{Datelike, Timelike};
use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use dehashed_rs::SearchType;
use ipnet::IpNet;
use itertools::Itertools;
use log::{error, info, warn};
use prost_types::Timestamp;
use rorm::cli;
use tokio::sync::mpsc;
use tokio::task;
use tonic::transport::Endpoint;
use trust_dns_resolver::Name;
use uuid::Uuid;

use crate::config::get_config;
use crate::modules::bruteforce_subdomains::{
    bruteforce_subdomains, BruteforceSubdomainResult, BruteforceSubdomainsSettings,
};
use crate::modules::certificate_transparency::{query_ct_api, CertificateTransparencySettings};
use crate::modules::port_scanner::icmp_scan::{start_icmp_scan, IcmpScanSettings};
use crate::modules::port_scanner::tcp_con::{start_tcp_con_port_scan, TcpPortScannerSettings};
use crate::modules::service_detection::DetectServiceSettings;
use crate::modules::{dehashed, service_detection, whois};
use crate::rpc::rpc_attacks::attack_results_service_client::AttackResultsServiceClient;
use crate::rpc::rpc_attacks::shared::CertEntry;
use crate::rpc::rpc_attacks::{CertificateTransparencyResult, MetaAttackInfo};
use crate::rpc::start_rpc_server;
use crate::utils::input;

pub mod config;
pub mod logging;
pub mod modules;
pub mod rpc;
pub mod utils;

/// The technique to use for the port scan
#[derive(Debug, ValueEnum, Copy, Clone)]
pub enum PortScanTechnique {
    /// A tcp connect scan
    TcpCon,
    /// A icmp scan
    Icmp,
}

/// The execution commands
#[derive(Subcommand)]
pub enum RunCommand {
    /// Bruteforce subdomains via DNS
    BruteforceSubdomains {
        /// Valid domain name
        target: Name,
        /// Path to a wordlist that can be used for subdomain enumeration.
        ///
        /// The entries in the wordlist are assumed to be line seperated.
        #[clap(short = 'w', long = "wordlist")]
        wordlist_path: PathBuf,
        /// The concurrent task limit
        #[clap(long)]
        #[clap(default_value_t = NonZeroU32::new(50).unwrap())]
        concurrent_limit: NonZeroU32,
    },
    /// Retrieve domains through certificate transparency
    CertificateTransparency {
        /// Valid domain name
        target: String,
        /// Whether expired certificates should be included
        #[clap(long)]
        #[clap(default_value_t = false)]
        include_expired: bool,
        /// The number of times the connection should be retried if it failed.
        #[clap(long)]
        #[clap(default_value_t = 6)]
        max_retries: u32,
        /// The interval that should be wait between retries on a port.
        ///
        /// The interval is specified in milliseconds.
        #[clap(long)]
        #[clap(default_value_t = 100)]
        retry_interval: u16,
    },
    /// A simple port scanning utility
    PortScanner {
        /// Valid IPv4 or IPv6 addresses or networks in CIDR notation
        #[clap(required(true))]
        targets: Vec<String>,
        /// A single port, multiple, comma seperated ports or (inclusive) port ranges
        ///
        /// If no values are supplied, 1-65535 is used as default
        #[clap(short = 'p')]
        ports: Vec<String>,
        /// Valid IPv4 or IPv6 addresses or networks in CIDR notation
        #[clap(long)]
        exclude: Vec<String>,
        /// The technique to use for port scans
        #[clap(short = 't', long)]
        #[clap(default_value = "tcp-con")]
        technique: PortScanTechnique,
        /// The time to wait until a connection is considered failed.
        ///
        /// The timeout is specified in milliseconds.
        #[clap(long)]
        #[clap(default_value_t = 1000)]
        timeout: u16,
        /// The concurrent task limit
        #[clap(long)]
        #[clap(default_value_t = NonZeroU32::new(1000).unwrap())]
        concurrent_limit: NonZeroU32,
        /// The number of times the connection should be retried if it failed.
        #[clap(long)]
        #[clap(default_value_t = 6)]
        max_retries: u32,
        /// The interval that should be wait between retries on a port.
        ///
        /// The interval is specified in milliseconds.
        #[clap(long)]
        #[clap(default_value_t = 100)]
        retry_interval: u16,
        /// Skips the initial icmp check.
        ///
        /// All hosts are assumed to be reachable.
        #[clap(long)]
        #[clap(default_value_t = false)]
        skip_icmp_check: bool,
    },
    /// Query the dehashed API
    Dehashed {
        /// The query for the api
        query: String,
    },
    /// Query whois entries
    Whois {
        /// The ip to query information for
        query: IpAddr,
    },
    /// Detect the service running behind a port
    ServiceDetection {
        /// The ip address to connect to
        addr: IpAddr,

        /// The port to connect to
        port: u16,

        /// The interval that should be waited for a response after connecting and sending an optional payload.
        ///
        /// The interval is specified in milliseconds.
        #[clap(long)]
        #[clap(default_value_t = 1000)]
        wait_for_response: u64,

        /// Flag for debugging
        ///
        /// Normally the service detection would stop after the first successful match.
        /// When this flag is enabled it will always run all checks producing their logs before returning the first match.
        #[clap(long)]
        debug: bool,
    },
}

/// All available subcommands
#[derive(Subcommand)]
pub enum Command {
    /// Start the leech as a server
    Server,
    /// Execute a command via CLI
    Execute {
        /// Specifies the verbosity of the output
        #[clap(short = 'v', global = true, action = ArgAction::Count)]
        verbosity: u8,

        /// Push the results to a workspace in ferrous
        #[clap(long)]
        push: Option<Uuid>,

        /// Api key to authenticate when pushing
        #[clap(long)]
        api_key: Option<String>,

        /// the subcommand to execute
        #[clap(subcommand)]
        command: RunCommand,
    },
    /// Apply migrations to the database
    Migrate {
        /// The directory where the migration files are located
        migration_dir: String,
    },
}

/// The main CLI parser
#[derive(Parser)]
pub struct Cli {
    /// Specify an alternative path to the config file
    #[clap(long = "config-path")]
    #[clap(default_value_t = String::from("/etc/leech/config.toml"))]
    config_path: String,

    /// Subcommands
    #[clap(subcommand)]
    commands: Command,
}

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.commands {
        Command::Migrate { migration_dir } => migrate(&cli.config_path, migration_dir).await?,
        Command::Server => {
            let config = get_config(&cli.config_path)?;
            logging::setup_logging(&config.logging)?;
            start_rpc_server(&config).await?;
        }
        Command::Execute {
            command,
            verbosity,
            push,
            api_key,
        } => {
            if env::var("RUST_LOG").is_err() {
                match verbosity {
                    0 => env::set_var("RUST_LOG", "leech=info"),
                    1 => env::set_var("RUST_LOG", "leech=debug"),
                    _ => env::set_var("RUST_LOG", "leech=trace"),
                }
            }
            env_logger::init();

            if let Some(workspace) = push {
                let config = get_config(&cli.config_path).map_err(|e| {
                    format!("Couldn't retrieve necessary config for pushing to ferrous: {e}")
                })?;

                let api_key = if let Some(api_key) = api_key {
                    api_key
                } else {
                    print!("Please enter your api key: ");
                    std::io::stdout().flush().unwrap();
                    input()
                        .await
                        .map_err(|err| err.to_string())?
                        .ok_or_else(|| "Can't push to ferrous without api key".to_string())?
                };

                match command {
                    RunCommand::CertificateTransparency {
                        target,
                        include_expired,
                        max_retries,
                        retry_interval,
                    } => {
                        let ct = CertificateTransparencySettings {
                            target,
                            include_expired,
                            max_retries,
                            retry_interval: Duration::from_millis(retry_interval as u64),
                        };

                        let entries = query_ct_api(ct).await?;

                        for x in entries
                            .iter()
                            .flat_map(|e| {
                                let mut name_value = e.name_value.clone();

                                name_value.push(e.common_name.clone());
                                name_value
                            })
                            .sorted()
                            .dedup()
                        {
                            info!("{x}");
                        }

                        info!("Sending results to ferrous");

                        let endpoint =
                            Endpoint::from_str(config.ferrous.ferrous_uri.as_ref()).unwrap();
                        let chan = endpoint.connect().await.unwrap();

                        let mut client = AttackResultsServiceClient::new(chan);
                        client
                            .certificate_transparency(CertificateTransparencyResult {
                                entries: entries
                                    .into_iter()
                                    .map(|x| CertEntry {
                                        value_names: x.name_value,
                                        common_name: x.common_name,
                                        serial_number: x.serial_number,
                                        not_after: x.not_after.map(|ts| {
                                            Timestamp::date_time_nanos(
                                                ts.year() as i64,
                                                ts.month() as u8,
                                                ts.day() as u8,
                                                ts.hour() as u8,
                                                ts.minute() as u8,
                                                ts.second() as u8,
                                                ts.nanosecond(),
                                            )
                                            .unwrap()
                                        }),
                                        not_before: x.not_before.map(|ts| {
                                            Timestamp::date_time_nanos(
                                                ts.year() as i64,
                                                ts.month() as u8,
                                                ts.day() as u8,
                                                ts.hour() as u8,
                                                ts.minute() as u8,
                                                ts.second() as u8,
                                                ts.nanosecond(),
                                            )
                                            .unwrap()
                                        }),
                                        issuer_name: x.issuer_name,
                                    })
                                    .collect(),
                                attack_info: Some(MetaAttackInfo {
                                    workspace_uuid: workspace.to_string(),
                                    api_key,
                                }),
                            })
                            .await
                            .unwrap();

                        info!("Finished sending results to ferrous")
                    }
                    _ => todo!("Not supported right now for pushing to ferrous"),
                }
            } else {
                match command {
                    RunCommand::BruteforceSubdomains {
                        target,
                        wordlist_path,
                        concurrent_limit,
                    } => {
                        let (tx, mut rx) = mpsc::channel(128);

                        task::spawn(async move {
                            while let Some(res) = rx.recv().await {
                                match res {
                                    BruteforceSubdomainResult::A { source, target } => {
                                        info!("Found a record for {source}: {target}");
                                    }
                                    BruteforceSubdomainResult::Aaaa { source, target } => {
                                        info!("Found aaaa record for {source}: {target}");
                                    }
                                    BruteforceSubdomainResult::Cname { source, target } => {
                                        info!("Found cname record for {source}: {target}");
                                    }
                                };
                            }
                        });

                        let settings = BruteforceSubdomainsSettings {
                            domain: target.to_string(),
                            wordlist_path,
                            concurrent_limit: u32::from(concurrent_limit),
                        };
                        if let Err(err) = bruteforce_subdomains(settings, tx).await {
                            error!("{err}");
                        }
                    }
                    RunCommand::CertificateTransparency {
                        target,
                        include_expired,
                        max_retries,
                        retry_interval,
                    } => {
                        let ct = CertificateTransparencySettings {
                            target,
                            include_expired,
                            max_retries,
                            retry_interval: Duration::from_millis(retry_interval as u64),
                        };

                        let entries = query_ct_api(ct).await?;
                        for x in entries
                            .into_iter()
                            .flat_map(|mut e| {
                                e.name_value.push(e.common_name);
                                e.name_value
                            })
                            .sorted()
                            .dedup()
                        {
                            info!("{x}");
                        }
                    }
                    RunCommand::PortScanner {
                        targets,
                        exclude,
                        technique,
                        ports,
                        timeout,
                        concurrent_limit,
                        max_retries,
                        retry_interval,
                        skip_icmp_check,
                    } => {
                        let mut addresses = vec![];
                        for target in targets {
                            if let Ok(addr) = IpAddr::from_str(&target) {
                                addresses.push(addr);
                            } else if let Ok(net) = IpNet::from_str(&target) {
                                addresses.extend(net.hosts());
                            } else {
                                return Err(format!("{target} isn't valid ip address or ip net"));
                            }
                        }

                        let mut exclude_addresses = vec![];
                        for ex in exclude {
                            if let Ok(addr) = IpAddr::from_str(&ex) {
                                exclude_addresses.push(addr);
                            } else if let Ok(net) = IpNet::from_str(&ex) {
                                exclude_addresses.extend(net.hosts());
                            } else {
                                return Err(format!("{ex} isn't valid ip address or ip net"));
                            }
                        }

                        let addresses: Vec<IpAddr> = addresses
                            .into_iter()
                            .filter(|addr| !exclude_addresses.contains(addr))
                            .sorted()
                            .dedup()
                            .collect();

                        let mut port_range = vec![];

                        if ports.is_empty() {
                            port_range.extend(1..=u16::MAX);
                        } else {
                            utils::parse_ports(&ports, &mut port_range)?;
                        }

                        match technique {
                            PortScanTechnique::TcpCon => {
                                let settings = TcpPortScannerSettings {
                                    addresses,
                                    port_range,
                                    timeout: Duration::from_millis(timeout as u64),
                                    skip_icmp_check,
                                    max_retries,
                                    retry_interval: Duration::from_millis(retry_interval as u64),
                                    concurrent_limit: u32::from(concurrent_limit),
                                };

                                let (tx, mut rx) = mpsc::channel(1);

                                task::spawn(async move {
                                    while let Some(addr) = rx.recv().await {
                                        info!("Open port found: {addr}");
                                    }
                                });

                                if let Err(err) = start_tcp_con_port_scan(settings, tx).await {
                                    error!("{err}");
                                }
                            }
                            PortScanTechnique::Icmp => {
                                let settings = IcmpScanSettings {
                                    addresses,
                                    timeout: Duration::from_millis(timeout as u64),
                                };
                                let (tx, mut rx) = mpsc::channel(1);

                                task::spawn(async move {
                                    while let Some(addr) = rx.recv().await {
                                        info!("Host up: {addr}");
                                    }
                                });

                                if let Err(err) = start_icmp_scan(settings, tx).await {
                                    error!("{err}");
                                }
                            }
                        }
                    }
                    RunCommand::Dehashed { query } => {
                        let email = match env::var("DEHASHED_EMAIL") {
                            Ok(x) => x,
                            Err(_) => {
                                error!("Missing environment variable DEHASHED_EMAIL");
                                return Err(
                                    "Missing environment variable DEHASHED_EMAIL".to_string()
                                );
                            }
                        };
                        let api_key = match env::var("DEHASHED_API_KEY") {
                            Ok(x) => x,
                            Err(_) => {
                                error!("Missing environment variable DEHASHED_API_KEY");
                                return Err(
                                    "Missing environment variable DEHASHED_API_KEY".to_string()
                                );
                            }
                        };

                        match dehashed::query(
                            email,
                            api_key,
                            dehashed_rs::Query::Domain(SearchType::Simple(query)),
                        )
                        .await
                        {
                            Ok(x) => {
                                for entry in x.entries {
                                    info!("{entry:?}");
                                }
                            }
                            Err(err) => error!("{err}"),
                        }
                    }
                    RunCommand::Whois { query } => match whois::query_whois(query).await {
                        Ok(x) => {
                            info!("Found result\n{x:#?}");
                        }
                        Err(err) => error!("{err}"),
                    },
                    RunCommand::ServiceDetection {
                        addr,
                        port,
                        wait_for_response,
                        debug,
                    } => {
                        let result = service_detection::detect_service(DetectServiceSettings {
                            socket: SocketAddr::new(addr, port),
                            wait_for_response: Duration::from_millis(wait_for_response),
                            always_run_everything: debug,
                        })
                        .await;
                        println!("{result:?}");
                    }
                }
            }
        }
    }

    Ok(())
}

async fn migrate(config_path: &str, migration_dir: String) -> Result<(), String> {
    let config = get_config(config_path)?;
    cli::migrate::run_migrate_custom(
        cli::config::DatabaseConfig {
            last_migration_table_name: None,
            driver: cli::config::DatabaseDriver::Postgres {
                host: config.database.host,
                port: config.database.port,
                name: config.database.name,
                user: config.database.user,
                password: config.database.password,
            },
        },
        migration_dir,
        false,
        None,
    )
    .await
    .map_err(|e| e.to_string())
}
