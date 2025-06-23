use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::env;

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    // Google
    pub google_client_id: ClientId,
    pub google_client_secret: ClientSecret,
    pub google_redirect_url: RedirectUrl,
    pub google_auth_url: AuthUrl,
    pub google_token_url: TokenUrl,
    // GitHub
    pub github_client_id: ClientId,
    pub github_client_secret: ClientSecret,
    pub github_auth_url: AuthUrl,
    pub github_token_url: TokenUrl,
    pub github_redirect_url: RedirectUrl,
}

impl OAuthConfig {
    pub fn from_env() -> Self {
        Self {
            // Google
            google_client_id: ClientId::new(
                env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
            ),
            google_client_secret: ClientSecret::new(
                env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
            ),
            google_redirect_url: RedirectUrl::new(
                env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL must be set"),
            )
            .expect("Invalid redirect URL"),
            google_auth_url: AuthUrl::new(
                "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            )
            .expect("Invalid auth URL"),
            google_token_url: TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .expect("Invalid token URL"),

            // Github
            github_client_id: ClientId::new(
                env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
            ),
            github_client_secret: ClientSecret::new(
                env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set"),
            ),
            github_redirect_url: RedirectUrl::new(
                env::var("GITHUB_REDIRECT_URL").expect("GITHUB_REDIRECT_URL must be set"),
            )
            .expect("Invalid github redirect URL"),
            github_auth_url: AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Invalid github auth URL"),
            github_token_url: TokenUrl::new(
                "https://github.com/login/oauth/access_token".to_string(),
            )
            .expect("Invalid github token URL"),
        }
    }
}
