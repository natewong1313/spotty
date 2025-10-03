use std::sync::Arc;

use crate::client::{
    auth::AuthenticatedSpotifySession, data::DataClient, streaming::StreamingClient,
};

pub struct SpotifyClient {
    // session: Arc<AuthenticatedSpotifySession>,
    pub streaming: StreamingClient,
    pub data: DataClient,
}

impl SpotifyClient {
    pub async fn new() -> anyhow::Result<Self> {
        let session = Arc::new(AuthenticatedSpotifySession::new().await?);
        let streaming = StreamingClient::new(session.clone()).await?;
        let data = DataClient::new(session.clone());
        Ok(SpotifyClient {
            // session,
            streaming,
            data,
        })
    }
}
