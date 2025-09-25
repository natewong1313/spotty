use std::{process::exit, sync::Arc};

use librespot::{
    core::SpotifyId,
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::Player,
    },
};

use crate::client::auth::AuthenticatedSpotifySession;

// Wrapper around librespot so we can call librespot methods with auth
pub struct StreamingClient {
    player: Arc<Player>,
}

impl StreamingClient {
    // TODO: prob shouldnt pass in session as an arg
    pub fn new(session: Arc<AuthenticatedSpotifySession>) -> Self {
        let config = PlayerConfig::default();
        let audio_format = AudioFormat::default();
        let backend = match audio_backend::find(None) {
            Some(backend) => backend,
            None => {
                // todo: improve this
                println!("failed to get audio_backend");
                exit(1);
            }
        };

        let player = Player::new(
            config,
            session.get_auth_session(),
            Box::new(NoOpVolume),
            move || backend(None, audio_format),
        );

        StreamingClient { player }
    }

    pub async fn play_track(&self, track_id: SpotifyId) {
        self.player.load(track_id, true, 0);
        self.player.await_end_of_track().await;
    }
}
