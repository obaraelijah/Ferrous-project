/*
 * ferrous
 *
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Generated by: https://openapi-generator.tech
 */


 use reqwest;

 use crate::apis::ResponseContent;
 use super::{Error, configuration};
 
 
 /// struct for typed errors of method [`export_workspace`]
 #[derive(Debug, Clone, Serialize, Deserialize)]
 #[serde(untagged)]
 pub enum ExportWorkspaceError {
     Status400(crate::models::ApiErrorResponse),
     Status500(crate::models::ApiErrorResponse),
     UnknownValue(serde_json::Value),
 }
 
 
 pub async fn export_workspace(configuration: &configuration::Configuration, uuid: &str) -> Result<crate::models::AggregatedWorkspace, Error<ExportWorkspaceError>> {
     let local_var_configuration = configuration;
 
     let local_var_client = &local_var_configuration.client;
 
     let local_var_uri_str = format!("{}/api/v1/export/workspace/{uuid}", local_var_configuration.base_path, uuid=crate::apis::urlencode(uuid));
     let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());
 
     if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
         local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
     }
 
     let local_var_req = local_var_req_builder.build()?;
     let local_var_resp = local_var_client.execute(local_var_req).await?;
 
     let local_var_status = local_var_resp.status();
     let local_var_content = local_var_resp.text().await?;
 
     if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
         serde_json::from_str(&local_var_content).map_err(Error::from)
     } else {
         let local_var_entity: Option<ExportWorkspaceError> = serde_json::from_str(&local_var_content).ok();
         let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
         Err(Error::ResponseError(local_var_error))
     }
 }
 