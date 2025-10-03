use flume::{Receiver, Sender};

use crate::client::client::SpotifyClient;
use crate::shared::message::{BackendMessage, GuiMessage, UserProfile};
use std::sync::Arc;
use std::thread::{self, sleep};
use std::time::Duration;

// I don't know a better name for this for now
pub struct Backend {
    from_gui: Receiver<BackendMessage>,
    to_gui: Sender<GuiMessage>,
    spotify_client: Arc<SpotifyClient>,
}

impl Backend {
    pub fn new(
        from_gui: Receiver<BackendMessage>,
        to_gui: Sender<GuiMessage>,
        spotify_client: SpotifyClient,
    ) -> Self {
        Self {
            from_gui,
            to_gui,
            spotify_client: Arc::new(spotify_client),
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        self.fetch_startup_data().await?;
        while let Ok(msg) = self.from_gui.recv_async().await {
            self.handle_message(msg);
        }

        Ok(())
    }

    // load stuff like user profile on start
    async fn fetch_startup_data(&self) -> anyhow::Result<()> {
        let name = self
            .spotify_client
            .data
            .get_user_name()
            .await?
            .unwrap_or("Unknown User".to_string());
        let profile_img = self.spotify_client.data.get_user_profile_img().await?;
        let user_profile = UserProfile { name, profile_img };
        self.to_gui
            .send(GuiMessage::UserProfileLoaded(user_profile))?;

        Ok(())
    }

    fn handle_message(&self, msg: BackendMessage) {
        match msg {
            BackendMessage::RequestLoadUserProfile => {
                println!("do sum here");
                // let _ = self.to_gui.send(GuiMessage::UserProfileLoaded);
            }
        }
    }
}

pub async fn init_backend(
    from_gui: Receiver<BackendMessage>,
    to_gui: Sender<GuiMessage>,
) -> anyhow::Result<()> {
    println!("getting spotify client");

    let spotify_client = SpotifyClient::new().await?;
    let backend = Backend::new(from_gui, to_gui, spotify_client);
    tokio::spawn(async move {
        backend.run().await;
    });
    Ok(())
}
