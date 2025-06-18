use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType}, AuthorizationCode, Client, CsrfToken, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RevocationErrorResponseType, Scope, StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse, TokenResponse,
};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use crate::{config::oauth::OAuthConfig, core::errors::OAuthError};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
}

pub struct GoogleOAuth {
    oauth_client: Client<
        StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
}

impl GoogleOAuth {
    pub fn new(config: &OAuthConfig) -> Self {
        let oauth_client = BasicClient::new(config.google_client_id.to_owned())
            .set_auth_uri(config.google_auth_url.to_owned())
            .set_token_uri(config.google_token_url.to_owned())
            .set_client_secret(config.google_client_secret.to_owned())
            .set_redirect_uri(config.google_redirect_url.to_owned());

        Self { oauth_client }
    }

    pub fn generate_auth_url(&self) -> (String, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url.to_string(), csrf_token, pkce_verifier)
    }

    pub async fn exchange_code(
        &self,
        code: String,
        pkce_verifier: PkceCodeVerifier,
        http_client: &HttpClient,
    ) -> Result<GoogleUser, OAuthError> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(http_client)
            .await
            .map_err(|e| OAuthError::OAuth(e.to_string()))?;

        // Use the access token to get the user info from Google
        let user_info: GoogleUser = http_client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token.access_token().secret())
            .send()
            .await
            .map_err(OAuthError::Http)?
            .json()
            .await
            .map_err(OAuthError::Http)?;

        Ok(user_info)
    }
}
