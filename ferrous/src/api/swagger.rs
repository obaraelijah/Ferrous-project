//! This module holds the swagger definitions.
//!
//! They got created with [utoipa].

use utoipa::openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::api::handler;
use crate::api::handler::{
    api_keys, attacks, auth, data_export, domains, global_tags, hosts, leeches, oauth,
    oauth_applications, oauth_decisions, ports, services, settings, users, websocket, wordlist,
    workspace_tags, workspaces,
};
use crate::models;
use crate::modules::oauth::schemas as oauth_schemas;

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
        auth::test,
        auth::login,
        auth::logout,
        auth::start_auth,
        auth::finish_auth,
        auth::start_register,
        auth::finish_register,
        leeches::create_leech,
        leeches::delete_leech,
        leeches::get_all_leeches,
        leeches::get_leech,
        leeches::update_leech,
        websocket::websocket,
        users::create_user,
        users::delete_user,
        users::get_user,
        users::get_all_users_admin,
        users::get_me,
        users::update_me,
        users::set_password,
        users::get_all_users,
        workspaces::create_workspace,
        workspaces::delete_workspace,
        workspaces::get_workspace,
        workspaces::get_all_workspaces,
        workspaces::update_workspace,
        workspaces::get_workspace_admin,
        workspaces::get_all_workspaces_admin,
        attacks::bruteforce_subdomains,
        attacks::scan_tcp_ports,
        attacks::query_certificate_transparency,
        attacks::delete_attack,
        attacks::get_attack,
        attacks::get_tcp_port_scan_results,
        attacks::query_dehashed,
        attacks::hosts_alive_check,
        attacks::service_detection,
        attacks::dns_resolution,
        oauth_applications::create_oauth_app,
        oauth_applications::get_all_oauth_apps,
        oauth_applications::get_oauth_app,
        oauth_applications::update_oauth_app,
        oauth_applications::delete_oauth_app,
        oauth::info,
        oauth::accept,
        oauth::deny,
        oauth_decisions::get_decisions,
        oauth_decisions::revoke_decision,
        settings::get_settings,
        settings::update_settings,
        api_keys::create_api_key,
        api_keys::delete_api_key,
        api_keys::get_api_keys,
        api_keys::update_api_key,
        hosts::get_all_hosts,
        hosts::get_host,
        hosts::update_host,
        global_tags::create_global_tag,
        global_tags::get_all_global_tags,
        global_tags::update_global_tag,
        global_tags::delete_global_tag,
        workspace_tags::create_workspace_tag,
        workspace_tags::get_all_workspace_tags,
        workspace_tags::update_workspace_tag,
        workspace_tags::delete_workspace_tag,
        ports::get_all_ports,
        services::get_all_services,
        domains::get_all_domains,
        wordlist::get_all_wordlists,
        wordlist::create_wordlist_admin,
        wordlist::get_all_wordlists_admin,
        wordlist::update_wordlist_admin,
        wordlist::delete_wordlist_admin,
    ),
    components(schemas(
        handler::ApiErrorResponse,
        handler::ApiStatusCode,
        handler::UuidResponse,
        handler::PageParams,
        handler::SimpleTag,
        handler::TagType,
        auth::LoginRequest,
        auth::FinishRegisterRequest,
        leeches::CreateLeechRequest,
        leeches::SimpleLeech,
        leeches::GetAllLeechesResponse,
        leeches::UpdateLeechRequest,
        users::CreateUserRequest,
        users::GetUser,
        users::GetUserResponse,
        users::UserResponse,
        users::UpdateMeRequest,
        users::SetPasswordRequest,
        users::SimpleUser,
        users::GetAllUsersResponse,
        workspaces::CreateWorkspaceRequest,
        workspaces::SimpleWorkspace,
        workspaces::FullWorkspace,
        workspaces::GetAllWorkspacesResponse,
        workspaces::UpdateWorkspaceRequest,
        attacks::SimpleAttack,
        attacks::BruteforceSubdomainsRequest,
        attacks::HostsAliveRequest,
        attacks::ScanTcpPortsRequest,
        attacks::QueryCertificateTransparencyRequest,
        attacks::PortOrRange,
        attacks::ServiceDetectionRequest,
        attacks::DnsResolutionRequest,
        handler::TcpPortScanResultsPage,
        attacks::SimpleTcpPortScanResult,
        dehashed_rs::Query,
        dehashed_rs::SearchType,
        attacks::QueryDehashedRequest,
        models::AttackType,
        oauth_applications::CreateAppRequest,
        oauth_applications::SimpleOauthClient,
        oauth_applications::FullOauthClient,
        oauth_applications::GetAppsResponse,
        oauth_applications::UpdateAppRequest,
        oauth::OpenRequestInfo,
        oauth_decisions::GetMyDecisionsResponse,
        oauth_decisions::FullDecision,
        settings::SettingsFull,
        settings::UpdateSettingsRequest,
        api_keys::FullApiKey,
        api_keys::CreateApiKeyRequest,
        api_keys::GetApiKeysResponse,
        api_keys::UpdateApiKeyRequest,
        handler::HostResultsPage,
        hosts::SimpleHost,
        hosts::FullHost,
        hosts::UpdateHostRequest,
        models::OsType,
        global_tags::CreateGlobalTagRequest,
        models::Color,
        global_tags::FullGlobalTag,
        global_tags::GetGlobalTagsResponse,
        global_tags::UpdateGlobalTag,
        models::PortProtocol,
        workspace_tags::FullWorkspaceTag,
        workspace_tags::GetWorkspaceTagsResponse,
        workspace_tags::UpdateWorkspaceTag,
        workspace_tags::CreateWorkspaceTagRequest,
        ports::SimplePort,
        handler::PortResultsPage,
        services::SimpleService,
        handler::ServiceResultsPage,
        domains::SimpleDomain,
        handler::DomainResultsPage,
        wordlist::GetAllWordlistsResponse,
        wordlist::SimpleWordlist,
        wordlist::CreateWordlistRequest,
        wordlist::GetAllWordlistsAdminResponse,
        wordlist::FullWordlist,
        wordlist::UpdateWordlistRequest,
    )),
    modifiers(&SecurityAddon, &SecurityAddon2)
)]
pub(crate) struct FrontendApi;

#[derive(OpenApi)]
#[openapi(
    paths(oauth::auth, oauth::token, data_export::export_workspace),
    components(schemas(
        models::OsType,
        models::PortProtocol,
        handler::ApiErrorResponse,
        handler::ApiStatusCode,
        oauth_schemas::TokenRequest,
        oauth_schemas::TokenResponse,
        oauth_schemas::TokenError,
        oauth_schemas::TokenErrorType,
        data_export::AggregatedWorkspace,
        data_export::AggregatedHost,
        data_export::AggregatedPort,
        data_export::AggregatedService,
        data_export::AggregatedDomain,
        data_export::AggregatedTags,
    )),
    modifiers(&SecurityAddon2)
)]
pub(crate) struct ExternalApi;
