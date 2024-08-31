use actix_toolbox::tb_middleware::Session;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, HttpResponse};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use log::error;
use rand::thread_rng;
use rorm::{query, update, Database, Model};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use webauthn_rs::prelude::Uuid;

use crate::api::handler::{ApiError, ApiResult};
use crate::api::middleware::{AdminRequired, AuthenticationRequired};
use crate::chan::{WsManagerChan, WsManagerMessage};
use crate::models::User;
use crate::modules::user::create::create_user_transaction;
use crate::modules::user::delete::delete_user_transaction;

#[derive(Deserialize, ToSchema)]
pub(crate) struct CreateUserRequest {
    #[schema(example = "user123")]
    pub(crate) username: String,
    #[schema(example = "Anon")]
    pub(crate) display_name: String,
    #[schema(example = "super-secure-password")]
    pub(crate) password: String,
    #[schema(example = true)]
    pub(crate) admin: bool,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct CreateUserResponse {
    #[param(example = "user123")]
    pub(crate) uuid: String,
}

/// Create a user
#[utoipa::path(
    tag = "User Admin Management",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "User got created", body = CreateUserResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    request_body = inline(CreateUserRequest),
    security(("api_key" = []))
)]
#[post("/users")]
pub(crate) async fn create_user(
    req: Json<CreateUserRequest>,
    db: Data<Database>,
) -> ApiResult<Json<CreateUserResponse>> {
    let uuid = create_user_transaction(
        req.username.clone(),
        req.display_name.clone(),
        req.password.clone(),
        req.admin,
        &db,
    )
    .await?;

    Ok(Json(CreateUserResponse {
        uuid: uuid.to_string(),
    }))
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct DeleteUserRequest {
    #[param(example = "user123")]
    pub(crate) username: String,
}

/// Delete a user by its username
#[utoipa::path(
    tag = "User Admin Management",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "User got deleted"),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    params(DeleteUserRequest),
    security(("api_key" = []))
)]
#[delete("/users/{username}")]
pub(crate) async fn delete_user(
    req: Path<DeleteUserRequest>,
    db: Data<Database>,
) -> ApiResult<HttpResponse> {
    delete_user_transaction(req.username.clone(), &db).await?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct GetUserRequest {
    pub(crate) username: String,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct GetUser {
    pub(crate) uuid: Uuid,
    #[schema(example = "user123")]
    pub(crate) username: String,
    #[schema(example = "Anon")]
    pub(crate) display_name: String,
    #[schema(example = true)]
    pub(crate) admin: bool,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) last_login: Option<DateTime<Utc>>,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct GetUserResponse {
    pub(crate) users: Vec<GetUser>,
}

/// Retrieve a user by its username
#[utoipa::path(
    tag = "User Admin Management",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "Returns the user", body = GetUser),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    params(GetUserRequest),
    security(("api_key" = []))
)]
#[get("/users/{username}")]
pub(crate) async fn get_user(
    req: Path<GetUserRequest>,
    db: Data<Database>,
) -> ApiResult<Json<GetUser>> {
    let user = query!(&db, User)
        .condition(User::F.username.equals(&req.username))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUsername)?;

    Ok(Json(GetUser {
        uuid: Uuid::from_slice(user.uuid.as_slice()).unwrap(),
        username: user.username,
        display_name: user.display_name,
        admin: user.admin,
        created_at: DateTime::from_utc(user.created_at, Utc),
        last_login: user.last_login.map(|dt| DateTime::from_utc(dt, Utc)),
    }))
}

/// Retrieve all users
#[utoipa::path(
    tag = "User Admin Management",
    context_path = "/api/v1/admin",
    responses(
        (status = 200, description = "Returns all users", body = GetUserResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    security(("api_key" = []))
)]
#[get("/users")]
pub(crate) async fn get_all_users(db: Data<Database>) -> ApiResult<Json<GetUserResponse>> {
    let users = query!(&db, User).all().await?;

    Ok(Json(GetUserResponse {
        users: users
            .into_iter()
            .map(|u| GetUser {
                uuid: Uuid::from_slice(u.uuid.as_slice()).unwrap(),
                username: u.username,
                display_name: u.display_name,
                admin: u.admin,
                created_at: DateTime::from_utc(u.created_at, Utc),
                last_login: u.last_login.map(|dt| DateTime::from_utc(dt, Utc)),
            })
            .collect(),
    }))
}

/// Retrieve the own user
#[utoipa::path(
    tag = "User Management",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Returns the own user", body = GetUser),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    security(("api_key" = []))
)]
#[get("/users/me")]
pub(crate) async fn get_me(session: Session, db: Data<Database>) -> ApiResult<Json<GetUser>> {
    let uuid: Vec<u8> = session.get("uuid")?.ok_or(ApiError::SessionCorrupt)?;

    let user = query!(&db, User)
        .condition(User::F.uuid.equals(&uuid))
        .optional()
        .await?
        .ok_or(ApiError::SessionCorrupt)?;

    Ok(Json(GetUser {
        uuid: Uuid::from_slice(&user.uuid).unwrap(),
        username: user.username,
        display_name: user.display_name,
        admin: user.admin,
        created_at: DateTime::from_utc(user.created_at, Utc),
        last_login: user.last_login.map(|dt| DateTime::from_utc(dt, Utc)),
    }))
}

#[derive(Deserialize, ToSchema)]
pub(crate) struct SetPasswordRequest {
    #[schema(example = "super-secure-password")]
    current_password: String,
    #[schema(example = "ultra-secure-password!1!1!")]
    new_password: String,
}

/// Set a new password
#[utoipa::path(
    tag = "User Management",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Password was updated"),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    request_body = SetPasswordRequest,
    security(("api_key" = []))
)]
#[post("/users/setPassword")]
pub(crate) async fn set_password(
    req: Json<SetPasswordRequest>,
    session: Session,
    db: Data<Database>,
    ws_manager_chan: Data<WsManagerChan>,
) -> ApiResult<HttpResponse> {
    let uuid: Vec<u8> = session.get("uuid")?.ok_or(ApiError::SessionCorrupt)?;

    let mut tx = db.start_transaction().await?;

    let user = query!(&db, User)
        .transaction(&mut tx)
        .condition(User::F.uuid.equals(&uuid))
        .optional()
        .await?
        .ok_or(ApiError::SessionCorrupt)?;

    if Argon2::default()
        .verify_password(
            req.current_password.as_bytes(),
            &PasswordHash::try_from(user.password_hash.as_str())?,
        )
        .is_err()
    {
        return Err(ApiError::InvalidPassword);
    }

    let salt = SaltString::generate(&mut thread_rng());
    let password_hash = Argon2::default()
        .hash_password(req.new_password.as_bytes(), &salt)?
        .to_string();

    update!(&db, User)
        .transaction(&mut tx)
        .set(User::F.password_hash, &password_hash)
        .exec()
        .await?;

    tx.commit().await?;

    session.purge();

    if let Err(err) = ws_manager_chan
        .send(WsManagerMessage::CloseSocket(uuid))
        .await
    {
        error!("Error sending to websocket manager: {err}");
        return Err(ApiError::InternalServerError);
    }

    Ok(HttpResponse::Ok().finish())
}
