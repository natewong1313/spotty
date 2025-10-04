use std::{error::Error, sync::Arc};

use futures::{StreamExt, TryStreamExt};
use rspotify::{
    AuthCodeSpotify,
    model::{PlayHistory, PrivateUser, SimplifiedPlaylist},
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
        DataClient { session }
    }
    pub async fn get_user_name(&self) -> anyhow::Result<Option<String>> {
        let user = self.get_user().await?;

        Ok(user.display_name)
    }
    pub async fn get_user_profile_img(&self) -> anyhow::Result<String> {
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

    async fn get_user(&self) -> anyhow::Result<PrivateUser> {
        let token = self.session.get_rspotify_token().await?;
        let spotify = AuthCodeSpotify::from_token(token);
        let user = spotify.me().await?;
        Ok(user)
    }

    pub async fn get_playlists(&self) -> anyhow::Result<Vec<SimplifiedPlaylist>> {
        let token = self.session.get_rspotify_token().await?;
        let spotify = AuthCodeSpotify::from_token(token);
        let raw_playlists = spotify.current_user_playlists();

        let playlists = raw_playlists.try_collect::<Vec<_>>().await?;
        Ok(playlists)
    }

    pub async fn get_recently_played(&self) -> anyhow::Result<Vec<PlayHistory>> {
        let token = self.session.get_rspotify_token().await?;
        let spotify = AuthCodeSpotify::from_token(token);
        let recently_played = spotify
            .current_user_recently_played(None, None)
            .await?
            .items;
        Ok(recently_played)
    }
}
