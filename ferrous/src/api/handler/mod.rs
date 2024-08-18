use actix_toolbox::tb_middleware::actix_session;
use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use log::{debug, error, info, trace, warn};
use serde::Serialize;
use serde_repr::Serialize_repr;
use webauthn_rs::prelude::WebauthnError;

pub(crate) use crate::api::handler::auth::*;
pub(crate) use crate::api::handler::user::*;
use crate::modules::user::create::CreateUserError;

mod auth;
mod user;

pub(crate) type ApiResult<T> = Result<T, ApiError>;

#[derive(Serialize_repr)]
#[repr(u16)]
enum ApiStatusCode {
    LoginFailed = 1000,
    NotFound = 1001,
    InvalidContentType = 1002,
    InvalidJson = 1003,
    PayloadOverflow = 1004,
    Unauthenticated = 1005,
    Missing2fa = 1006,
    MissingPrivileges = 1007,
    NoSecurityKeyAvailable = 1008,
    UserAlreadyExists = 1009,
    InternalServerError = 2000,
    DatabaseError = 2001,
    SessionError = 2002,
    WebauthnError = 2003,
}

#[derive(Serialize)]
struct ApiErrorResponse {
    status_code: ApiStatusCode,
    message: String,
}

impl ApiErrorResponse {
    pub(crate) fn new(status_code: ApiStatusCode, message: String) -> Self {
        Self {
            status_code,
            message,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ApiError {
    LoginFailed,
    NotFound,
    InvalidContentType,
    InvalidJson(serde_json::Error),
    PayloadOverflow(String),
    InternalServerError,
    DatabaseError(rorm::Error),
    InvalidHash(argon2::password_hash::Error),
    SessionInsert(actix_session::SessionInsertError),
    SessionGet(actix_session::SessionGetError),
    Unauthenticated,
    Missing2FA,
    SessionCorrupt,
    MissingPrivileges,
    NoSecurityKeyAvailable,
    Webauthn(WebauthnError),
    UserAlreadyExists,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::LoginFailed => write!(f, "Login failed"),
            ApiError::DatabaseError(_) => write!(f, "Database error occurred"),
            ApiError::InvalidHash(_) | ApiError::InternalServerError => {
                write!(f, "Internal server error")
            }
            ApiError::SessionInsert(_) | ApiError::SessionGet(_) => {
                write!(f, "Session error occurred")
            }
            ApiError::NotFound => write!(f, "Not found"),
            ApiError::InvalidContentType => write!(f, "Content type error"),
            ApiError::InvalidJson(err) => write!(f, "Json error: {err}"),
            ApiError::PayloadOverflow(err) => write!(f, "{err}"),
            ApiError::Unauthenticated => write!(f, "Unauthenticated"),
            ApiError::Missing2FA => write!(f, "2FA is missing"),
            ApiError::SessionCorrupt => write!(f, "Corrupt session"),
            ApiError::MissingPrivileges => write!(f, "You are missing privileges"),
            ApiError::NoSecurityKeyAvailable => write!(f, "No security key available"),
            ApiError::Webauthn(_) => write!(f, "Webauthn error"),
            ApiError::UserAlreadyExists => write!(f, "User already exists"),
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ApiError::LoginFailed => {
                trace!("Login failed");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::LoginFailed,
                    self.to_string(),
                ))
            }
            ApiError::DatabaseError(err) => {
                error!("Database error occurred: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::DatabaseError,
                    self.to_string(),
                ))
            }
            ApiError::InvalidHash(err) => {
                error!("Got invalid password hash from db: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::InternalServerError,
                    self.to_string(),
                ))
            }
            ApiError::SessionInsert(err) => {
                error!("Session insert error: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::SessionError,
                    self.to_string(),
                ))
            }
            ApiError::SessionGet(err) => {
                error!("Session get error: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::SessionError,
                    self.to_string(),
                ))
            }
            ApiError::NotFound => HttpResponse::NotFound().json(ApiErrorResponse::new(
                ApiStatusCode::NotFound,
                self.to_string(),
            )),
            ApiError::InvalidContentType => HttpResponse::BadRequest().json(ApiErrorResponse::new(
                ApiStatusCode::InvalidContentType,
                self.to_string(),
            )),
            ApiError::InvalidJson(err) => {
                debug!("Received invalid json: {err}");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::InvalidJson,
                    self.to_string(),
                ))
            }
            ApiError::InternalServerError => HttpResponse::InternalServerError().json(
                ApiErrorResponse::new(ApiStatusCode::InternalServerError, self.to_string()),
            ),
            ApiError::PayloadOverflow(err) => {
                debug!("Payload overflow: {err}");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::PayloadOverflow,
                    self.to_string(),
                ))
            }
            ApiError::Unauthenticated => {
                trace!("Unauthenticated");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::Unauthenticated,
                    self.to_string(),
                ))
            }
            ApiError::Missing2FA => {
                trace!("Missing 2fa");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::Missing2fa,
                    self.to_string(),
                ))
            }
            ApiError::SessionCorrupt => {
                warn!("Corrupt session");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::SessionError,
                    self.to_string(),
                ))
            }
            ApiError::MissingPrivileges => {
                trace!("Missing privileges");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::MissingPrivileges,
                    self.to_string(),
                ))
            }
            ApiError::NoSecurityKeyAvailable => {
                debug!("Missing security key");

                HttpResponse::BadRequest().json(ApiErrorResponse::new(
                    ApiStatusCode::NoSecurityKeyAvailable,
                    self.to_string(),
                ))
            }
            ApiError::Webauthn(err) => {
                info!("Webauthn error: {err}");

                HttpResponse::InternalServerError().json(ApiErrorResponse::new(
                    ApiStatusCode::WebauthnError,
                    self.to_string(),
                ))
            }
            ApiError::UserAlreadyExists => HttpResponse::BadRequest().json(ApiErrorResponse::new(
                ApiStatusCode::UserAlreadyExists,
                self.to_string(),
            )),
        }
    }
}

impl From<rorm::Error> for ApiError {
    fn from(value: rorm::Error) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::InvalidHash(value)
    }
}

impl From<actix_session::SessionInsertError> for ApiError {
    fn from(value: actix_session::SessionInsertError) -> Self {
        Self::SessionInsert(value)
    }
}

impl From<actix_session::SessionGetError> for ApiError {
    fn from(value: actix_session::SessionGetError) -> Self {
        Self::SessionGet(value)
    }
}

impl From<CreateUserError> for ApiError {
    fn from(value: CreateUserError) -> Self {
        match value {
            CreateUserError::DatabaseError(err) => Self::DatabaseError(err),
            CreateUserError::UsernameAlreadyExists => Self::UserAlreadyExists,
            CreateUserError::HashError(err) => Self::InvalidHash(err),
        }
    }
}
