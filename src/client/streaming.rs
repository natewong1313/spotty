use std::{
    process::exit,
    sync::Arc,
    time::Duration,
};

use librespot::{
    connect::{ConnectConfig, LoadRequest, LoadRequestOptions, Spirc},
    core::{Error, SpotifyId},
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::{self, MixerConfig},
        player::Player,
    },
};
use tokio::time::sleep;

use crate::client::auth::AuthenticatedSpotifySession;

// Wrapper around librespot so we can call librespot methods with auth
pub struct StreamingClient {
    player: Arc<Player>,
    spirc: Spirc,
    username: String,
}

impl StreamingClient {
    // TODO: prob shouldnt pass in session as an arg
    pub async fn new(session: Arc<AuthenticatedSpotifySession>) -> Result<Self, Error> {
        let player_config = PlayerConfig::default();
        let mixer_config = MixerConfig::default();
        let mut connect_config = ConnectConfig::default();
        connect_config.name = "spotty".to_string();

        let audio_format = AudioFormat::default();

        let backend = match audio_backend::find(None) {
            Some(backend) => backend,
            None => {
                // todo: improve this
                println!("failed to get audio_backend");
                exit(1);
            }
        };

        let mixer_builder = mixer::find(None).unwrap();
        let mixer = mixer_builder(mixer_config)?;
        let auth_session = session.get_auth_session();

        let credentials = session.get_credentials();
        let username = credentials.clone().username.unwrap_or_default();

        println!("logged in as {}", username);

        // auth_session.connect(credentials.clone(), true).await?;

        let player = Player::new(
            player_config,
            auth_session.clone(),
            mixer.get_soft_volume(),
            move || backend(None, audio_format),
        );

        let event_channel = player.get_player_event_channel();

        let (spirc, spirc_task) = Spirc::new(
            connect_config,
            auth_session.clone(),
            credentials,
            player.clone(),
            mixer,
        )
        .await?;
        tokio::task::spawn(async move {
            tokio::select! {
                () = spirc_task => {},
            }
        });

        Ok(StreamingClient {
            player,
            spirc,
            username,
        })
    }

    pub async fn play_track(&self, _track_id: SpotifyId) -> Result<(), Error> {
        println!("playing track");
        let request_options = LoadRequestOptions::default();

        self.spirc.activate()?;
        self.spirc.load(LoadRequest::from_context_uri(
            format!("spotify:user:{}:collection", self.username),
            request_options,
        ))?;
        self.spirc.play()?;

        sleep(Duration::from_millis(100000)).await;
        // self.player.load(track_id, true, 0);
        // self.player.await_end_of_track().await;
        //
        Ok(())
    }
}
