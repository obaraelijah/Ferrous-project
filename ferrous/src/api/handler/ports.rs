use actix_toolbox::tb_middleware::Session;
use actix_web::get;
use actix_web::web::{Data, Json, Path};
use futures::TryStreamExt;
use rorm::{query, Database, FieldAccess, Model};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::handler::{workspaces, ApiError, ApiResult, PathUuid};
use crate::models::{Port, PortProtocol};

/// The simple representation of a port
#[derive(Serialize, ToSchema)]
pub struct SimplePort {
    /// Uuid of the port
    pub uuid: Uuid,
    /// Port number
    pub port: i16,
    /// Port protocol
    pub protocol: PortProtocol,
    /// The host this port is assigned to
    pub host: Uuid,
    /// A comment to the port
    pub comment: String,
}

/// All ports of a workspace
#[derive(Serialize, ToSchema)]
pub struct GetAllPortsResponse {
    ports: Vec<SimplePort>,
}

#[get("/workspaces/{uuid}/ports")]
pub async fn get_all_ports(
    path: Path<PathUuid>,
    db: Data<Database>,
    session: Session,
) -> ApiResult<Json<GetAllPortsResponse>> {
    let user_uuid: Uuid = session.get("uuid")?.ok_or(ApiError::SessionCorrupt)?;
    let path = path.into_inner();

    let mut tx = db.start_transaction().await?;

    if !workspaces::is_user_member_or_owner(&mut tx, user_uuid, path.uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    let ports: Vec<_> = query!(&mut tx, Port)
        .condition(Port::F.workspace.equals(path.uuid))
        .stream()
        .map_ok(|x| SimplePort {
            uuid: x.uuid,
            port: x.port,
            protocol: x.protocol,
            comment: x.comment,
            host: *x.host.key(),
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(Json(GetAllPortsResponse { ports }))
}