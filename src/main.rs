use std::sync::mpsc;

use crate::{
    backend::backend::init_backend,
    gui::init_gui,
    shared::message::{BackendMessage, GuiMessage},
};
mod backend;
mod client;
mod gui;
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (to_backend, from_gui) = flume::unbounded::<BackendMessage>();
    let (to_gui, from_backend) = flume::unbounded::<GuiMessage>();

    init_backend(from_gui, to_gui).await?;
    init_gui(from_backend, to_backend);

    Ok(())
}
