use std::{error::Error, sync::Arc};

use rspotify::{
    AuthCodeSpotify,
    model::{Image, PrivateUser},
    prelude::{BaseClient, OAuthClient},
};

use crate::client::auth::AuthenticatedSpotifySession;

// TODO: probably rename this to something better
// This file is a rspotify wrapper to make authenticated calls
pub struct DataClient {
    session: Arc<AuthenticatedSpotifySession>,
}

impl DataClient {
    pub fn new(session: Arc<AuthenticatedSpotifySession>) -> Self {
        // test if this works
        DataClient { session }
    }
    pub async fn get_user_name(&self) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        let user = self.get_user().await?;

        Ok(user.display_name)
    }
    pub async fn get_user_profile_img(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let user = self.get_user().await?;
        let image = match user.images {
            Some(images) if images.len() > 0 => images[0].url.to_owned(),
            Some(_) | None => {
                // default profile pic i got from google
                "https://i.scdn.co/image/ab67616100005174757d9a0af822e6400aa3e180".to_string()
            }
        };
        Ok(image)
    }

    async fn get_user(&self) -> Result<PrivateUser, Box<dyn Error + Send + Sync>> {
        let token = self.session.get_rspotify_token().await?;
        let spotify = AuthCodeSpotify::from_token(token);
        let user = spotify.me().await?;
        Ok(user)
    }
}
