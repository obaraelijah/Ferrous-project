//! Certificate transparency can be used for subdomain enumerations
//!
//! For technical information, see [here](https://certificate.transparency.dev/)

pub mod crt_sh_db;
pub mod crt_sh_types;

/// Settings for a certificate transparency search request
pub struct CertificateTransparencySettings {
    /// The target domain to query
    pub target: String,
    /// Also include already expired certificates
    pub include_expired: bool,
}