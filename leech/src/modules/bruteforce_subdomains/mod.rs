//! This module uses a wordlist to bruteforce subdomains of a target domain.
//!
//! It requests A and AAAA records of the constructed domain of a DNS server.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;


/// The settings to configure a subdomain bruteforce
#[derive(Debug)]
pub struct BruteforceSubdomainsSettings {
    /// The domain to use as base name. It shouldn't end in a . like DNS names.
    pub domain: String,
    /// Path to a wordlist that can be used for subdomain enumeration.
    ///
    /// The entries in the wordlist are assumed to be line seperated.
    pub wordlist_path: PathBuf,
}

/// Enumerates subdomains by brute forcing dns records with a wordlist.
///
/// **Parameter**:
/// - `settings`: [BruteforceSubdomainsSettings]
pub async fn bruteforce_subdomains(settings: BruteforceSubdomainsSettings) -> Result<(), String> {
    unimplemented!()
}
