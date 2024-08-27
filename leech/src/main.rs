//! # Leeches
//! Leeches are the workers of Ferrous.
//!
//! They provide a gRPC server to receive requests from Ferrous and respond with results.
//! If this connection is lost somehow, they will store the results in a local database
//! and will try to connect to the Ferrous gRPC server to send the missing data.
//!
//! You can also use the leech as a cli utility without a Ferrous attached for manual
//! execution and testing. See the subcommand `run` for further information.
#![warn(missing_docs)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]

use std::net::IpAddr;
use std::num::{NonZeroU16, NonZeroUsize};
use std::str::FromStr;
use std::time::Duration;

use clap::{Parser, Subcommand};
use ipnet::IpNet;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::config::get_config;
use crate::modules::port_scanner::{start_tcp_con_port_scan, TcpPortScannerSettings};

pub mod config;
pub mod modules;

pub(crate) struct Regexes {
    pub(crate) ports: Regex,
}

static RE: Lazy<Regexes> = Lazy::new(|| Regexes {
    ports: Regex::new(r#"^(?P<range>\d*-\d*)$|^(?P<single>\d+)$|^$"#).unwrap(),
});

#[derive(Subcommand)]
enum RunCommand {
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
        /// The time to wait until a connection is considered failed.
        ///
        /// The timeout is specified in milliseconds.
        #[clap(long)]
        #[clap(default_value_t = 1000)]
        timeout: u16,
        /// The concurrent task limit
        #[clap(long)]
        #[clap(default_value_t = NonZeroUsize::new(1000).unwrap())]
        concurrent_limit: NonZeroUsize,
        /// The number of times the connection should be retried if it failed.
        #[clap(long)]
        #[clap(default_value_t = 6)]
        max_retries: u8,
        /// The interval that should be wait between retries on a port.
        ///
        /// The interval is specified in milliseconds.
        #[clap(long)]
        #[clap(default_value_t = 100)]
        retry_interval: u16,
        /// Skips the initial ping check.
        ///
        /// All hosts are assumed to be reachable.
        #[clap(long)]
        #[clap(default_value_t = false)]
        skip_ping_check: bool,
    },
}

#[derive(Subcommand)]
enum Command {
    Server,
    Execute {
        #[clap(subcommand)]
        command: RunCommand,
    },
}

#[derive(Parser)]
struct Cli {
    #[clap(long = "config-path")]
    #[clap(help = "Specify an alternative path to the config file")]
    #[clap(default_value_t = String::from("/etc/leech/config.toml"))]
    config_path: String,

    #[clap(subcommand)]
    commands: Command,
}

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let _config = get_config(&cli.config_path).map_err(|e| e.to_string())?;

    match cli.commands {
        Command::Server => {}
        Command::Execute { command } => todo!(),
    }

    Ok(())
}
