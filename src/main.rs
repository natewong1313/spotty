use librespot::core::{Error, SpotifyId, spotify_id::SpotifyItemType};
use std::sync::Arc;

use crate::{client::client::SpotifyClient, gui::init_app};
mod client;
mod gui;

use eframe::egui;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_app();

    Ok(())
}
