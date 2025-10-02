use flume::{Receiver, Sender};

use crate::client::client::SpotifyClient;
use crate::shared::message::{BackendMessage, GuiMessage};
use std::thread;

// I don't know a better name for this for now
pub struct Backend {
    from_gui: Receiver<BackendMessage>,
    to_gui: Sender<GuiMessage>,
    spotify_client: SpotifyClient,
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
            spotify_client,
        }
    }

    pub async fn run(self) {
        while let Ok(msg) = self.from_gui.recv_async().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&self, msg: BackendMessage) {
        match msg {
            BackendMessage::RequestLoadUserProfile => {
                println!("do sum here");
                let _ = self.to_gui.send(GuiMessage::UserProfileLoaded);
            }
        }
    }
}

pub async fn init_backend(
    from_gui: Receiver<BackendMessage>,
    to_gui: Sender<GuiMessage>,
) -> Result<(), librespot::core::Error> {
    println!("getting spotify client");

    let spotify_client = SpotifyClient::new().await?;
    let backend = Backend::new(from_gui, to_gui, spotify_client);
    tokio::spawn(async move {
        backend.run().await;
    });
    Ok(())
}
