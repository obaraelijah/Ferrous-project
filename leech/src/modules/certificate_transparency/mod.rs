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

/// Query the crt.sh certificate transparency api.
///
/// **Parameters**:
/// - `settings`: [CertificateTransparencySettings]
pub async fn query_ct_db(settings: CertificateTransparencySettings) {
    let mut db_config = DatabaseConfiguration::new(DatabaseDriver::Postgres {
        name: "certwatch".to_string(),
        host: "crt.sh".to_string(),
        port: 5432,
        user: "guest".to_string(),
        password: "".to_string(),
    });
    db_config.disable_logging = Some(true);
    let db = Database::connect(db_config).await.unwrap();

    let rows = match db
        .raw_sql(
            get_query(&settings.target, settings.include_expired).as_str(),
            None,
            None,
        )
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            println!("Error querying data: {err}");
            return;
        }
    };

    let entries: Vec<String> = rows
        .into_iter()
        .map(|row| row.get(2).unwrap())
        .sorted()
        .dedup()
        .collect();

    println!("{entries:#?}");
}