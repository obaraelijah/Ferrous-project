//! Certificate transparency can be used for subdomain enumerations
//!
//! For technical information, see [here](https://certificate.transparency.dev/)

pub struct CertificateTransparencySettings {
    pub target: String,
    pub include_expired: bool,
}
