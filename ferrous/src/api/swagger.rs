//! This module holds the swagger definitions.
//!
//! They got created with [utoipa].

use utoipa::openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::api::handler;
use crate::api::handler::{api_keys, global_tags, hosts, oauth, workspace_tags};
use crate::models;

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

struct SecurityAddon2;

impl Modify for SecurityAddon2 {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_token",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::test,
        handler::login,
        handler::logout,
        handler::start_auth,
        handler::finish_auth,
        handler::start_register,
        handler::finish_register,
        handler::create_leech,
        handler::delete_leech,
        handler::get_all_leeches,
        handler::get_leech,
        handler::update_leech,
        handler::websocket,
        handler::create_user,
        handler::delete_user,
        handler::get_user,
        handler::get_all_users,
        handler::get_me,
        handler::update_me,
        handler::set_password,
        handler::create_workspace,
        handler::delete_workspace,
        handler::get_workspace,
        handler::get_all_workspaces,
        handler::update_workspace,
        handler::get_workspace_admin,
        handler::get_all_workspaces_admin,
        handler::bruteforce_subdomains,
        handler::scan_tcp_ports,
        handler::query_certificate_transparency,
        handler::delete_attack,
        handler::get_attack,
        handler::get_tcp_port_scan_results,
        oauth::create_oauth_app,
        oauth::get_all_oauth_apps,
        oauth::get_oauth_app,
        oauth::update_oauth_app,
        oauth::delete_oauth_app,
        oauth::info,
        oauth::auth,
        oauth::accept,
        oauth::deny,
        oauth::token,
        handler::get_settings,
        handler::update_settings,
        api_keys::create_api_key,
        api_keys::delete_api_key,
        api_keys::get_api_keys,
        api_keys::update_api_key,
        hosts::get_all_hosts,
        hosts::get_host,
        global_tags::create_global_tag,
        global_tags::get_all_global_tags,
        global_tags::update_global_tag,
        global_tags::delete_global_tag,
        workspace_tags::create_workspace_tag,
        workspace_tags::get_all_workspace_tags,
        workspace_tags::update_workspace_tag,
        workspace_tags::delete_workspace_tag,
    ),
    components(schemas(
        handler::ApiErrorResponse,
        handler::ApiStatusCode,
        handler::LoginRequest,
        handler::FinishRegisterRequest,
        handler::CreateLeechRequest,
        handler::GetLeech,
        handler::GetLeechResponse,
        handler::UpdateLeechRequest,
        handler::CreateUserRequest,
        handler::CreateUserResponse,
        handler::GetUser,
        handler::GetUserResponse,
        handler::UserResponse,
        handler::UpdateMeRequest,
        handler::SetPasswordRequest,
        handler::CreateWorkspaceRequest,
        handler::SimpleWorkspace,
        handler::FullWorkspace,
        handler::SimpleAttack,
        handler::GetWorkspaceResponse,
        handler::UpdateWorkspaceRequest,
        handler::BruteforceSubdomainsRequest,
        handler::ScanTcpPortsRequest,
        handler::QueryCertificateTransparencyRequest,
        handler::PortOrRange,
        handler::PageParams,
        handler::TcpPortScanResultsPage,
        handler::SimpleTcpPortScanResult,
        handler::UuidResponse,
        models::AttackType,
        oauth::CreateAppRequest,
        oauth::SimpleOauthClient,
        oauth::FullOauthClient,
        oauth::GetAppsResponse,
        oauth::UpdateAppRequest,
        oauth::OpenRequestInfo,
        oauth::TokenResponse,
        oauth::TokenErrorResponse,
        oauth::Pkce,
        oauth::TokenType,
        oauth::GrantType,
        oauth::TokenRequest,
        oauth::CodeChallengeMethod,
        handler::SettingsFull,
        handler::UpdateSettingsRequest,
        api_keys::SimpleApiKey,
        api_keys::CreateApiKeyRequest,
        api_keys::GetApiKeysResponse,
        api_keys::UpdateApiKeyRequest,
        hosts::GetAllHostsResponse,
        hosts::SimpleHost,
        models::OsType,
        global_tags::CreateGlobalTagRequest,
        handler::Color,
        global_tags::FullGlobalTag,
        global_tags::GetGlobalTagsResponse,
        global_tags::UpdateGlobalTag,
        models::PortProtocol,
        workspace_tags::FullWorkspaceTag,
        workspace_tags::GetWorkspaceTagsResponse,
        workspace_tags::UpdateWorkspaceTag,
    )),
    modifiers(&SecurityAddon, &SecurityAddon2),
)]
pub(crate) struct ApiDoc;