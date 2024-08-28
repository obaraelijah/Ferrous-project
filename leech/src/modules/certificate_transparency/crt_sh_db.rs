//! This module contains the db queries for the crt.sh database

/// Returns a query for the database of crt.sh
///
/// **Parameter**:
/// - `target`: [&str]: Domain to search
/// - `include_expired`: [bool]: If true, expired certificates are included in the results

pub fn get_query(target: &str, include_expired: bool) -> String {
    unimplemented!()
}