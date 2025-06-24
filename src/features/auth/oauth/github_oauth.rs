use crate::{
    config::oauth::OAuthConfig, core::errors::OAuthError, features::auth::model::GithubUser,
};
use oauth2::{
    AuthorizationCode, Client, CsrfToken, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RevocationErrorResponseType, Scope, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
    TokenResponse,
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
};
use reqwest::Client as HttpClient;

pub struct GithubOAuth {
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

impl GithubOAuth {
    pub fn new(config: &OAuthConfig) -> Self {
        let oauth_client = BasicClient::new(config.github_client_id.to_owned())
            .set_auth_uri(config.github_auth_url.to_owned())
            .set_token_uri(config.github_token_url.to_owned())
            .set_client_secret(config.github_client_secret.to_owned())
            .set_redirect_uri(config.github_redirect_url.to_owned());

        Self { oauth_client }
    }

    pub fn generate_auth_url(&self) -> (String, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .add_scope(Scope::new("read:user".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url.to_string(), csrf_token, pkce_verifier)
    }

    pub async fn exchange_code(
        &self,
        code: String,
        pkce_verifier: PkceCodeVerifier,
        http_client: &HttpClient,
    ) -> Result<GithubUser, OAuthError> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(http_client)
            .await
            .map_err(|e| OAuthError::OAuth(e.to_string()))?;

        let resp = http_client
            .get("https://api.github.com/user")
            .bearer_auth(token.access_token().secret())
            .header("User-Agent", "e-commerce_rust") // GitHub requires a User-Agent header
            .send()
            .await
            .map_err(OAuthError::Http)?;

        let text = resp.text().await.map_err(OAuthError::Http)?;
        println!("GitHub userinfo response: {}", text);
        let user_info: GithubUser = serde_json::from_str(&text).map_err(|e| OAuthError::OAuth(e.to_string()))?;
        
        Ok(user_info)
    }
}
