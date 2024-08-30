use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use actix_web::web::Data;
use log::{debug, warn};
use rorm::{query, Database, Model};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tonic::transport::{Channel, Endpoint};

use crate::models::Leech;
use crate::rpc::rpc_attacks::req_attack_service_client::ReqAttackServiceClient;

pub(crate) type RpcManagerChannel = Sender<RpcManagerEvent>;
pub(crate) type RpcClients = Data<RwLock<HashMap<i64, ReqAttackServiceClient<Channel>>>>;

/**
Events for the RpcManager.
Make sure to commit any pending database state regarding the event
as the RpcManager must be able to retrieve the new state.
*/

/**
Starts the rpc connection to a leech.
**Parameter**:
- `leech`: [Leech]: Instance of a leech
- `rpc_clients`: [RpcClients]
*/

pub(crate) async fn rpc_client_loop(leech: Leech, rpc_clients: RpcClients) {
    unimplemented!()
}

pub(crate) enum RpcManagerEvent {
    /// Leech got deleted.
    Deleted(i64),
    /// Leech got created.
    Created(i64),
    /// Leech got updated.
    Updated(i64),
}