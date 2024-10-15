
use once_cell::sync::Lazy;
use regex::bytes::Regex;
use log::debug;

pub struct AllProbes {
    pub empty_tcp_probes: [Vec<EmptyProbe>; 3],
    pub payload_tcp_probes: [Vec<PayloadProbe>; 3],
    pub empty_tls_probes: [Vec<EmptyProbe>; 3],
    pub payload_tls_probes: [Vec<PayloadProbe>; 3],
    pub udp_probes: [Vec<PayloadProbe>; 3],
}

/// A probe without payload
pub struct EmptyProbe {
    /// The name of the service detected by this probe
    pub service: &'static str,
    
    /// The base regex this probe is tested against
    pub regex: Regex,
    
    /// Secondary regexes to match against (if any)
    pub sub_regex: Vec<Regex>,
}

impl EmptyProbe {
    pub fn is_match(&self, data: &[u8]) -> bool {
        if self.regex.is_match(data) {
            if self.sub_regex.is_empty() {
                true
            } else {
                debug!(target: "regex", "Initial regex matched for service: {}", self.service);
                for sub in &self.sub_regex {
                    if sub.is_match(data) {
                        return true;
                    }
                }
                false
            }
        } else {
            false
        }
    }
}

/// A probe with payload
pub struct PayloadProbe {
    /// The name of the service detected by this probe
    pub service: &'static str,
    
    /// The payload to send upon connection
    pub payload: &'static [u8],
    
    /// The base regex this probe is tested against
    pub regex: Regex,
    
    /// Secondary regexes to match against (if any)
    pub sub_regex: Vec<Regex>,
}

impl PayloadProbe {
    pub fn is_match(&self, data: &[u8]) -> bool {
        if self.regex.is_match(data) {
            if self.sub_regex.is_empty() {
                true
            } else {
                debug!(target: "regex", "Initial regex matched for service: {}", self.service);
                for sub in &self.sub_regex {
                    if sub.is_match(data) {
                        return true;
                    }
                }
                false
            }
        } else {
            false
        }
    }
}

/// Lists of all probes
pub static PROBES: Lazy<AllProbes> = Lazy::new(|| AllProbes {
    empty_tcp_probes: [
        vec![EmptyProbe { service: "ssh", regex: Regex::new(r"^SSH-[\d.]+-[^ -]+(?: [^\r\n])?\r?\n").unwrap(), sub_regex: vec![] },EmptyProbe { service: "mariadb", regex: Regex::new(r"MariaDB").unwrap(), sub_regex: vec![Regex::new(r"^.\x00\x00\x00\x0a(5\.[-_~.+:\w]+MariaDB-[-_~.+:\w]+)\x00").unwrap(),Regex::new(r"^E\x00\x00\x00(?s-u:.)(?s-u:.)\x04Host .* is not allowed to connect to this MariaDB server").unwrap(),] },],
        vec![],
        vec![],
    ],
    payload_tcp_probes: [
        vec![PayloadProbe { service: "http2", regex: Regex::new(r"^(?s-u:.)(?s-u:.)(?s-u:.)\x04(?s-u:.)(?:\x00|\x80)\x00\x00\x00").unwrap(), sub_regex: vec![], payload: b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n\x00\x00\x00\x04\x00\x00\x00\x00\x00" },PayloadProbe { service: "http", regex: Regex::new(r"HTTP/1.[01] \d\d\d [^\r\n]+\r\n").unwrap(), sub_regex: vec![], payload: b"HEAD / HTTP/1.1\r\n\r\n" },],
        vec![],
        vec![],
    ],
    empty_tls_probes: [
        vec![],
        vec![],
        vec![],
    ],
    payload_tls_probes: [
        vec![PayloadProbe { service: "https", regex: Regex::new(r"HTTP/1.[01] \d\d\d [^\r\n]+\r\n").unwrap(), sub_regex: vec![], payload: b"HEAD / HTTP/1.1\r\n\r\n" },],
        vec![],
        vec![],
    ],
    udp_probes: [
        vec![],
        vec![],
        vec![],
    ],
});
