use std::str::FromStr;

use actix_web::http::Uri;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use rorm::{delete, insert, query, Database, Model};
use serde::{Deserialize, Serialize};

use crate::api::handler::{ApiError, ApiResult, PathId};
use crate::models::{Leech, LeechInsert};

#[derive(Deserialize)]
pub(crate) struct CreateLeechRequest {
    pub(crate) name: String,
    pub(crate) address: String,
}

#[derive(Serialize)]
pub(crate) struct CreateLeechResponse {
    pub(crate) id: i64,
}

pub(crate) async fn create_leech(
    req: Json<CreateLeechRequest>,
    db: Data<Database>,
) -> ApiResult<Json<CreateLeechResponse>> {
    let mut tx = db.start_transaction().await?;

    let address = Uri::from_str(req.address.as_str()).map_err(|_| ApiError::InvalidAddress)?;
    if let Some(scheme) = address.scheme_str() {
        if scheme != "https" && scheme != "http" {
            return Err(ApiError::InvalidAddress);
        }
    } else {
        return Err(ApiError::InvalidAddress);
    }

    if query!(&db, Leech)
        .transaction(&mut tx)
        .condition(Leech::F.address.equals(&address.to_string()))
        .optional()
        .await?
        .is_some()
    {
        return Err(ApiError::AddressAlreadyExists);
    }

    if query!(&db, Leech)
        .transaction(&mut tx)
        .condition(Leech::F.name.equals(&req.name))
        .optional()
        .await?
        .is_some()
    {
        return Err(ApiError::NameAlreadyExists);
    }

    let id = insert!(&db, LeechInsert)
        .transaction(&mut tx)
        .single(&LeechInsert {
            name: req.name.clone(),
            address: address.to_string(),
        })
        .await?;

    tx.commit().await?;

    Ok(Json(CreateLeechResponse { id }))
}

pub(crate) async fn delete_leech(
    path: Path<PathId>,
    db: Data<Database>,
) -> ApiResult<HttpResponse> {
    let mut tx = db.start_transaction().await?;

    query!(&db, (Leech::F.id,))
        .transaction(&mut tx)
        .condition(Leech::F.id.equals(path.id as i64))
        .optional()
        .await?
        .ok_or(ApiError::InvalidId)?;

    delete!(&db, Leech)
        .transaction(&mut tx)
        .condition(Leech::F.id.equals(path.id as i64))
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}