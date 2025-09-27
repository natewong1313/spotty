use std::{collections::HashSet, sync::Arc};

use librespot::{
    core::{Error, Session, SessionConfig, cache::Cache},
    discovery::Credentials,
    oauth::OAuthClientBuilder,
};

const SPOTIFY_REDIRECT_URI: &str = "http://127.0.0.1:8898/login";
const SPOTIFY_SCOPES: &[&str] = &["streaming", "playlist-read-private"];
const CACHE_PATH: &str = ".spotty_cache";

#[derive(Clone)]
pub struct AuthenticatedSpotifySession {
    session: Session,
    credentials: Credentials,
}

// todo: better error handling
impl AuthenticatedSpotifySession {
    pub async fn new() -> Result<Self, Error> {
        let libre_cache = Cache::new(Some(CACHE_PATH), None, None, None)?;
        let credentials = Self::load_credentials(&libre_cache).await?;
        let session = Self::new_session(libre_cache);

        // session.connect(credentials.clone(), true).await?;
        // println!("signed in to user {}", session.username());

        Ok(AuthenticatedSpotifySession {
            session,
            credentials,
        })
    }

    pub async fn get_rspotify_token(&self) -> Result<rspotify::Token, Error> {
        // todo: remove unwrap
        let auth_tokens = self.session.login5().auth_token().await.unwrap();
        let expires_in = chrono::Duration::from_std(auth_tokens.expires_in).unwrap();
        let expires_at = chrono::Utc::now() + expires_in;

        let token = rspotify::Token {
            access_token: auth_tokens.access_token,
            expires_in,
            expires_at: Some(expires_at),
            scopes: HashSet::new(),
            refresh_token: None,
        };

        Ok(token)
    }

    pub fn get_auth_session(&self) -> Session {
        return self.session.clone();
    }

    pub fn get_credentials(&self) -> Credentials {
        return self.credentials.clone();
    }

    // Either get cached credentials or create a new set of credentials
    async fn load_credentials(cache: &Cache) -> Result<Credentials, Error> {
        let credentials = match cache.credentials() {
            Some(credentials) => credentials,
            None => {
                let new_credentials = Self::new_credentials().await?;
                new_credentials
            }
        };

        Ok(credentials)
    }
    // Goes through oauth flow and opens a new browser link
    async fn new_credentials() -> Result<Credentials, Error> {
        let session_config = SessionConfig::default();
        let oauth_client = OAuthClientBuilder::new(
            &session_config.client_id,
            SPOTIFY_REDIRECT_URI,
            SPOTIFY_SCOPES.to_vec(),
        )
        .open_in_browser()
        .build()?;

        let token = oauth_client
            .get_access_token_async()
            .await
            .map(|t| Credentials::with_access_token(t.access_token))?;
        Ok(token)
    }

    fn new_session(cache: Cache) -> Session {
        let session_config = SessionConfig::default();
        Session::new(session_config, Some(cache))
    }
}
