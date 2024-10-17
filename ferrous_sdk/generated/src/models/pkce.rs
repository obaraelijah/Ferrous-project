/*
 * ferrous
 *
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Generated by: https://openapi-generator.tech
 */

 /// Pkce : The client sends the code challenge as part of the OAuth 2.0 Authorization Request ([Section 4.1.1 of \\[RFC6749\\]](https://www.rfc-editor.org/rfc/rfc6749#section-4.1.1)) using the following additional parameters:

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pkce {
    /// Code challenge.
    #[serde(rename = "code_challenge")]
    pub code_challenge: String,
    #[serde(rename = "code_challenge_method", skip_serializing_if = "Option::is_none")]
    pub code_challenge_method: Option<crate::models::CodeChallengeMethod>,
}

impl Pkce {
    /// The client sends the code challenge as part of the OAuth 2.0 Authorization Request ([Section 4.1.1 of \\[RFC6749\\]](https://www.rfc-editor.org/rfc/rfc6749#section-4.1.1)) using the following additional parameters:
    pub fn new(code_challenge: String) -> Pkce {
        Pkce {
            code_challenge,
            code_challenge_method: None,
        }
    }
}