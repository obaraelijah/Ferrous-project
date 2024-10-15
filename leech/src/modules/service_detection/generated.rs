
use once_cell::sync::Lazy;
use probe_config::generated::*;
use regex::bytes::Regex;

/// Lists of all probes
pub static PROBES: Lazy<AllProbes> = Lazy::new(|| AllProbes {
    empty_tcp_probes: [
        vec![BaseProbe { service: "ssh", regex: Regex::new(r"^SSH-[\d.]+-[^ -]+(?: [^\r\n])?\r?\n").unwrap(), sub_regex: vec![] },BaseProbe { service: "mariadb", regex: Regex::new(r"MariaDB").unwrap(), sub_regex: vec![Regex::new(r"^.\x00\x00\x00\x0a(5\.[-_~.+:\w]+MariaDB-[-_~.+:\w]+)\x00").unwrap(),Regex::new(r"^E\x00\x00\x00(?s-u:.)(?s-u:.)\x04Host .* is not allowed to connect to this MariaDB server").unwrap(),] },],
        vec![],
        vec![],
    ],
    payload_tcp_probes: [
        vec![PayloadProbe { base: BaseProbe { service: "http2", regex: Regex::new(r"^(?s-u:.)(?s-u:.)(?s-u:.)\x04(?s-u:.)(?:\x00|\x80)\x00\x00\x00").unwrap(), sub_regex: vec![] }, payload: b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n\x00\x00\x00\x04\x00\x00\x00\x00\x00" },PayloadProbe { base: BaseProbe { service: "http", regex: Regex::new(r"HTTP/1.[01] \d\d\d [^\r\n]+\r\n").unwrap(), sub_regex: vec![] }, payload: b"HEAD / HTTP/1.1\r\n\r\n" },],
        vec![],
        vec![],
    ],
    empty_tls_probes: [
        vec![],
        vec![],
        vec![],
    ],
    payload_tls_probes: [
        vec![TlsProbe { base: BaseProbe { service: "https2", regex: Regex::new(r"^(?s-u:.)(?s-u:.)(?s-u:.)\x04(?s-u:.)(?:\x00|\x80)\x00\x00\x00").unwrap(), sub_regex: vec![] }, payload: b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n\x00\x00\x00\x04\x00\x00\x00\x00\x00", alpn: Some("h2") },TlsProbe { base: BaseProbe { service: "https", regex: Regex::new(r"HTTP/1.[01] \d\d\d [^\r\n]+\r\n").unwrap(), sub_regex: vec![] }, payload: b"HEAD / HTTP/1.1\r\n\r\n", alpn: None },],
        vec![],
        vec![],
    ],
    udp_probes: [
        vec![],
        vec![],
        vec![],
    ],
});
