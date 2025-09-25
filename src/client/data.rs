use std::sync::Arc;

use rspotify::{AuthCodeSpotify, prelude::OAuthClient};

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
    pub async fn test(&self) {
        let token = self.session.get_rspotify_token().await.unwrap();
        let spotify = AuthCodeSpotify::from_token(token);
        let me = spotify.me().await.unwrap();
        println!("{}", me.id);
    }
}
