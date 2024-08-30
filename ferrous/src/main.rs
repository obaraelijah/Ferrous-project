//! A scalable pen-testing platform.
//!
//! # Ferrous
//! The core of the ferrous project.
//!
//! It provides an API for accessing and retrieving data and events for the user
//! as well as an gRPC client and server interface for the leeches.
//!
//! ## Leeches
//! Leeches are the workers of ferrous.
//! ferrous for it self, does not collect any data.
#![warn(missing_docs)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]

use std::fs::read_to_string;

use actix_toolbox::logging::setup_logging;
use actix_web::cookie::Key;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use clap::{Command, Parser, Subcommand};
use rorm::{Database, DatabaseConfiguration, DatabaseDriver};

use crate::api::server;
use crate::config::Config;

mod api;
pub(crate) mod chan;
pub mod config;
pub mod models;
pub mod modules;
pub(crate) mod rpc;

#[derive(Parser)]
#[clap(version, about = "The ferrous core")]
struct Cli {
    #[clap(long = "config-path")]
    #[clap(help = "Specify an alternative path to the config file")]
    #[clap(default_value_t = String::from("/etc/ferrous/config.toml"))]
    config_path: String,

    #[clap(subcommand)]
    command: Command,
}

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let config_content =
        read_to_string(&cli.config_path).map_err(|e| format!("Error reading config file: {e}"))?;
    let config: Config =
        toml::from_str(&config_content).map_err(|e| format!("Error parsing config file: {e}"))?;

    setup_logging(&config.logging)?;

    match cli.command {
        Command::Start => {
            let db = get_db(&config).await?;
            let (rpc_manager_chan, rpc_clients) = chan::start_rpc_manager(db.clone()).await?;
            let ws_manager_chan = chan::start_ws_manager().await?;

            server::start_server(db, &config, rpc_manager_chan, rpc_clients, ws_manager_chan)
                .await?;
        }
        Command::Keygen => {
            let key = Key::generate();
            println!("{}", BASE64_STANDARD.encode(key.master()));
        }
    }

    Ok(())
}

/// Opens a connection to the database using the provided config
///
/// **Parameter**:
/// - `config`: Reference to [Config]
async fn get_db(config: &Config) -> Result<Database, String> {
    let db_config = DatabaseConfiguration {
        driver: DatabaseDriver::Postgres {
            host: config.database.host.clone(),
            port: config.database.port,
            user: config.database.user.clone(),
            password: config.database.password.clone(),
            name: config.database.name.clone(),
        },
        min_connections: 2,
        max_connections: 20,
        disable_logging: Some(true),
        statement_log_level: None,
        slow_statement_log_level: None,
    };

    Database::connect(db_config)
        .await
        .map_err(|e| format!("Error connecting to the database: {e}"))
}
