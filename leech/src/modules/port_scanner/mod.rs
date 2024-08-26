//! This module holds a port scanning utility.

use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

/// The settings of a tcp connection port scan
#[derive(Clone, Debug)]
pub struct TcpPortScannerSettings {
    /// The addresses to scan
    pub addresses: Vec<IpAddr>,
    /// The port range to scan
    pub port_range: Vec<u16>,
    /// The duration to wait for a response
    pub timeout: Duration,
    /// Defines how many times a connection should be retried if it failed the last time
    pub max_retries: u8,
    /// The interval to wait in between the retries
    pub retry_interval: Duration,
    /// Maximum of concurrent tasks that should be spawned
    ///
    /// 0 means, that there should be no limit.
    pub concurrent_limit: usize,
    /// If set to true, there won't be an initial ping check.
    ///
    /// All hosts are assumed to be reachable.
    pub skip_ping_check: bool,
}

pub async fn start_tcp_con_port_scan(settings: TcpPortScannerSettings) {
    unimplemented!()
}
