use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::env;

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub google_client_id: ClientId,
    pub google_client_secret: ClientSecret,
    pub google_redirect_url: RedirectUrl,
    pub google_auth_url: AuthUrl,
    pub google_token_url: TokenUrl,
}

impl OAuthConfig {
    pub fn from_env() -> Self {
        Self {
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
        }
    }
}
