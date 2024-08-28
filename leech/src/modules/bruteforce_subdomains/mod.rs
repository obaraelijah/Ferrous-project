//! This module uses a wordlist to bruteforce subdomains of a target domain.
//!
//! It requests A and AAAA records of the constructed domain of a DNS server.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub struct BruteforceSubdomainsSettings {
    pub domain: String,
    pub wordlist_path: PathBuf,
}
