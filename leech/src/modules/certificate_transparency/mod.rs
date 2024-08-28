//! Certificate transparency can be used for subdomain enumerations
//!
//! For technical information, see [here](https://certificate.transparency.dev/)

pub mod crt_sh_db;
pub mod crt_sh_types;

pub struct CertificateTransparencySettings {
    pub target: String,
    pub include_expired: bool,
}
