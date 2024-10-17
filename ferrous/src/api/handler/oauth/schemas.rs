//! Different request and response types as defined in [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749)

use std::time::Duration;

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

/// The client constructs the request URI by adding the following
/// parameters to the query component of the authorization endpoint URI
/// using the "application/x-www-form-urlencoded" format
#[derive(Deserialize, IntoParams)]
pub(crate) struct AuthRequest {
    /// Value MUST be set to "code".
    pub response_type: String,

    /// The client identifier as described in [Section 2.2](https://www.rfc-editor.org/rfc/rfc6749#section-2.2).
    pub client_id: Uuid,

    /// As described in [Section 3.1.2](https://www.rfc-editor.org/rfc/rfc6749#section-3.1.2).
    pub redirect_uri: Option<String>,

    /// The scope of the access request as described by [Section 3.3](https://www.rfc-editor.org/rfc/rfc6749#section-3.3).
    pub scope: Option<String>,

    /// An opaque value used by the client to maintain
    /// state between the request and callback.  The authorization
    /// server includes this value when redirecting the user-agent back
    /// to the client.  The parameter SHOULD be used for preventing
    /// cross-site request forgery as described in [Section 10.12](https://www.rfc-editor.org/rfc/rfc6749#section-10.12).
    pub state: Option<String>,

    /// Pkce Code challenge.
    pub code_challenge: Option<String>,

    /// Code verifier transformation method is "S256" or "plain".
    /// It defaults to "plain" if not present in the request.
    #[serde(default)]
    pub code_challenge_method: CodeChallengeMethod,
}

#[derive(Deserialize, Default, Copy, Clone, Debug, ToSchema)]
pub(crate) enum CodeChallengeMethod {
    #[default]
    #[serde(rename = "S256")]
    Sha256,
    #[serde(rename = "plain")]
    Plain,
}

#[derive(Serialize, Debug)]
pub(crate) struct AuthError {
    pub error: AuthErrorType,
    pub state: Option<String>,
    pub error_description: Option<&'static str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AuthErrorType {
    /// The request is missing a required parameter, includes an
    /// invalid parameter value, includes a parameter more than
    /// once, or is otherwise malformed.
    InvalidRequest,

    /// The client is not authorized to request an
    /// authorization code using this method.
    UnauthorizedClient,

    /// The resource owner or authorization server denied the request.
    AccessDenied,

    /// The authorization server does not support obtaining an
    /// authorization code using this method.
    UnsupportedResponseType,

    /// The requested scope is invalid, unknown, or malformed.
    InvalidScope,

    /// The authorization server encountered an unexpected
    /// condition that prevented it from fulfilling the request.
    /// (This error code is needed because a 500
    /// Internal Server Error HTTP status code cannot be returned
    /// to the client via an HTTP redirect.)
    ServerError,

    /// The authorization server is currently unable to handle
    /// the request due to a temporary overloading or maintenance
    /// of the server.  (This error code is needed because a 503
    /// Service Unavailable HTTP status code cannot be returned
    /// to the client via an HTTP redirect.)
    TemporarilyUnavailable,
}

#[derive(Deserialize, ToSchema)]
pub(crate) struct TokenRequest {
    /// Must be "authorization_code"
    pub grant_type: GrantType,
    pub code: Uuid,
    pub redirect_uri: String,
    pub client_id: Uuid,
    pub client_secret: String,
    /// Code verifier
    pub code_verifier: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GrantType {
    AuthorizationCode,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct TokenResponse {
    /// Always `"access_token"`
    #[schema(example = "access_token")]
    pub token_type: TokenType,

    pub access_token: String,

    /// Duration in seconds the token is valid for
    #[schema(value_type = u64)]
    #[serde(serialize_with = "duration_seconds")]
    pub expires_in: Duration,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TokenType {
    AccessToken,
}

/// Possible error response when requesting an access token.
#[derive(Debug, Serialize, ToSchema)]
pub struct TokenError {
    /// A single ASCII \[[USASCII](https://www.rfc-editor.org/rfc/rfc6749#ref-USASCII)\] error code
    pub error: TokenErrorType,

    /// Human-readable ASCII \[[USASCII](https://www.rfc-editor.org/rfc/rfc6749#ref-USASCII)\] text providing
    /// understanding the error that occurred.
    pub error_description: Option<&'static str>,
}

/// Possible error types of a [`TokenError`]
#[allow(dead_code)]
#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TokenErrorType {
    /// The request is missing a required parameter, includes an
    /// unsupported parameter value (other than grant type),
    /// repeats a parameter, includes multiple credentials,
    /// utilizes more than one mechanism for authenticating the
    /// client, or is otherwise malformed.
    InvalidRequest,

    /// Client authentication failed (e.g., unknown client, no
    /// client authentication included, or unsupported
    /// authentication method).  The authorization server MAY
    /// return an HTTP 401 (Unauthorized) status code to indicate
    /// which HTTP authentication schemes are supported.  If the
    /// client attempted to authenticate via the "Authorization"
    /// request header field, the authorization server MUST
    /// respond with an HTTP 401 (Unauthorized) status code and
    /// include the "WWW-Authenticate" response header field
    /// matching the authentication scheme used by the client.
    InvalidClient,

    /// The provided authorization grant (e.g., authorization
    /// code, resource owner credentials) or refresh token is
    /// invalid, expired, revoked, does not match the redirection
    /// URI used in the authorization request, or was issued to
    /// another client.
    InvalidGrant,

    /// The authenticated client is not authorized to use this
    /// authorization grant type.
    UnauthorizedClient,

    /// The authorization grant type is not supported by the
    /// authorization server.
    UnsupportedGrantType,

    /// The requested scope is invalid, unknown, malformed, or
    /// exceeds the scope granted by the resource owner.
    InvalidScope,

    /// The authorization server encountered an unexpected
    /// condition that prevented it from fulfilling the request.
    ///
    /// This type is not in the rfc
    ServerError,
}

fn duration_seconds<S: serde::ser::Serializer>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_u64(duration.as_secs())
}