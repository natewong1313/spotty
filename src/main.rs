use librespot::core::{Error, SpotifyId, spotify_id::SpotifyItemType};

use crate::client::client::SpotifyClient;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let spotify_client = SpotifyClient::new().await?;
    let mut track = SpotifyId::from_base62("3orAdhaGP0RhjMN3f8B8Im").unwrap();
    track.item_type = SpotifyItemType::Track;

    spotify_client.data.test().await;

    spotify_client.streaming.play_track(track).await;
    Ok(())
}
