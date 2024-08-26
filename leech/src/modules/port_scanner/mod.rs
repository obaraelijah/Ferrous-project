//! This module holds a port scanning utility.

use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

pub struct TcpPortScannerSettings {
    pub addresses: Vec<IpAddr>,
    pub timeout: Duration,
}
