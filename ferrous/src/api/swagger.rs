//! This module holds the swagger definitions.
//!
//! They got created with [utoipa].

use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::api::handler;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("id"))),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        handler::ApiErrorResponse,
        handler::ApiStatusCode,
        handler::LoginRequest,
        handler::FinishRegisterRequest,
        handler::CreateLeechResponse,
        handler::CreateLeechRequest,
        handler::GetLeech,
        handler::GetLeechResponse,
        handler::UpdateLeechRequest,
        handler::CreateUserRequest,
        handler::CreateUserResponse,
        handler::GetUser,
        handler::GetUserResponse,
        handler::SetPasswordRequest,
        handler::CreateWorkspaceRequest,
        handler::CreateWorkspaceResponse,
        handler::GetWorkspace,
        handler::GetWorkspaceResponse,
    )),
    modifiers(&SecurityAddon),
)]
pub(crate) struct ApiDoc;