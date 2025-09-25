use std::sync::Arc;

use crate::client::{
    auth::AuthenticatedSpotifySession, data::DataClient, streaming::StreamingClient,
};

pub struct SpotifyClient {
    pub session: Arc<AuthenticatedSpotifySession>,
    pub streaming: StreamingClient,
    pub data: DataClient,
}

impl SpotifyClient {
    pub async fn new() -> Result<Self, librespot::core::Error> {
        let session = Arc::new(AuthenticatedSpotifySession::new().await?);
        let streaming = StreamingClient::new(session.clone());
        let data = DataClient::new(session.clone());
        Ok(SpotifyClient {
            session,
            streaming,
            data,
        })
    }
}
