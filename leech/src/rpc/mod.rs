//! The gRPC part of the leech.
//!
//! In server mode, the leech has a grpc server running to receive requests from ferrous.
//! If the connection drops or the leech can't send the data, it will be saved in the local
//! database and pushing the data to the server is tried regularly.
//!
//! In cli mode, the leech can push the results to ferrous if desired.

use tonic::transport::Server;

use crate::config::Config;

/// Starts the gRPC server
///
/// **Parameter**:
/// - `config`: Reference to [Config]
pub async fn start_rpc_server(config: &Config) -> Result<(), String> {
    Ok(())
}