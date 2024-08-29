#![warn(missing_docs)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]

use std::env;
use std::net::IpAddr;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use clap::{ArgAction, Parser, Subcommand};
use ipnet::IpNet;
use itertools::Itertools;
use log::{error, info};
use tokio::sync::mpsc;
use tokio::task;
use trust_dns_resolver::Name;

use crate::config::get_config;
use crate::modules::bruteforce_subdomains::{
    bruteforce_subdomains, BruteforceSubdomainResult, BruteforceSubdomainsSettings,
};
use crate::modules::certificate_transparency::{
    query_ct_api, query_ct_db, CertificateTransparencySettings,
};
use crate::modules::port_scanner::{start_tcp_con_port_scan, TcpPortScannerSettings};

pub mod config;
pub mod modules;
pub mod utils;

// ... [Rest of the code remains the same until the main function] ...

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.commands {
        Command::Server => {
            let _config = get_config(&cli.config_path).map_err(|e| e.to_string())?;
        }
        Command::Execute { command, verbosity } => {
            if env::var("RUST_LOG").is_err() {
                match verbosity {
                    0 => env::set_var("RUST_LOG", "leech=info"),
                    1 => env::set_var("RUST_LOG", "leech=debug"),
                    _ => env::set_var("RUST_LOG", "leech=trace"),
                }
            }
            env_logger::init();

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
                        concurrent_limit: usize::from(concurrent_limit),
                    };
                    bruteforce_subdomains(settings, tx).await?
                }
                RunCommand::CertificateTransparency {
                    target,
                    include_expired,
                    db,
                } => {
                    let ct = CertificateTransparencySettings {
                        target,
                        include_expired,
                    };
                    if db {
                        query_ct_db(ct).await;
                    } else {
                        query_ct_api(ct).await;
                    }
                }
                RunCommand::PortScanner {
                    targets,
                    exclude,
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

                    let settings = TcpPortScannerSettings {
                        addresses,
                        port_range,
                        timeout: Duration::from_millis(timeout as u64),
                        skip_icmp_check,
                        max_retries,
                        retry_interval: Duration::from_millis(retry_interval as u64),
                        concurrent_limit: usize::from(concurrent_limit),
                    };

                    let (tx, mut rx) = mpsc::channel(128);

                    task::spawn(async move {
                        while let Some(addr) = rx.recv().await {
                            info!("Open port found: {addr}");
                        }
                    });

                    if let Err(err) = start_tcp_con_port_scan(settings, tx).await {
                        error!("{err}");
                    }
                }
            }
        }
    }

    Ok(())
}
