/*
 * ferrous
 *
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`auth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthError {
    Status400(crate::models::ApiErrorResponse),
    Status500(crate::models::ApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenError {
    Status400(crate::models::TokenError),
    Status500(crate::models::TokenError),
    UnknownValue(serde_json::Value),
}

/// Initial endpoint an application redirects the user to.  It requires both the `state` parameter against CSRF, as well as a pkce challenge. The only supported pkce `code_challenge_method` is `S256`.
pub async fn auth(
    configuration: &configuration::Configuration,
    response_type: &str,
    client_id: &str,
    scope: &str,
    state: &str,
    code_challenge: &str,
    redirect_uri: Option<&str>,
    code_challenge_method: Option<&str>,
) -> Result<(), Error<AuthError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/oauth/auth", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder =
        local_var_req_builder.query(&[("response_type", &response_type.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("client_id", &client_id.to_string())]);
    if let Some(ref local_var_str) = redirect_uri {
        local_var_req_builder =
            local_var_req_builder.query(&[("redirect_uri", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("scope", &scope.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("state", &state.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("code_challenge", &code_challenge.to_string())]);
    if let Some(ref local_var_str) = code_challenge_method {
        local_var_req_builder =
            local_var_req_builder.query(&[("code_challenge_method", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AuthError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Endpoint an application calls itself after the user accepted and was redirected back to it.
pub async fn token(
    configuration: &configuration::Configuration,
    token_request: crate::models::TokenRequest,
) -> Result<(), Error<TokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/oauth-server/token",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&token_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TokenError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
