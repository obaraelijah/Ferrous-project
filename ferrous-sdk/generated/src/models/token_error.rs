/*
 * ferrous
 *
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Generated by: https://openapi-generator.tech
 */

/// TokenError : Possible error response when requesting an access token.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenError {
    #[serde(rename = "error")]
    pub error: crate::models::TokenErrorType,
    /// Human-readable ASCII \\[[USASCII](https://www.rfc-editor.org/rfc/rfc6749#ref-USASCII)\\] text providing understanding the error that occurred.
    #[serde(
        rename = "error_description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_description: Option<Option<String>>,
}

impl TokenError {
    /// Possible error response when requesting an access token.
    pub fn new(error: crate::models::TokenErrorType) -> TokenError {
        TokenError {
            error,
            error_description: None,
        }
    }
}
