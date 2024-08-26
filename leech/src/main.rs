//! # Leeches
//! Leeches are the workers of Ferrous.
//!
//! They provide a gRPC server to receive requests from Ferrous and respond with results.
//! If this connection is lost somehow, they will store the results in a local database
//! and will try to connect to the Ferrous gRPC server to send the missing data.
//!
//! You can also use the leech as a cli utility without a Ferrous attached for manual
//! execution and testing. See the subcommand `run` for further information.
#![warn(missing_docs)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]

pub mod modules;

#[rorm::rorm_main]
#[tokio::main]
async fn main() {
    println!("Ferrous, project!");
}
