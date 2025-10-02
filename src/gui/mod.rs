use crate::gui::app::App;
use crate::gui::fonts::get_fonts;
use crate::shared::message::{BackendMessage, GuiMessage};
use eframe::{Error, NativeOptions};
use flume::{Receiver, Sender};

pub mod app;
pub mod fonts;

pub fn init_gui(
    from_backend: Receiver<GuiMessage>,
    to_backend: Sender<BackendMessage>,
) -> Result<(), Error> {
    dioxus_devtools::connect_subsecond();

    subsecond::call(move || {
        let from_backend = from_backend.clone();
        let to_backend = to_backend.clone();
        eframe::run_native(
            "Spotty",
            NativeOptions::default(),
            Box::new(move |cc| {
                cc.egui_ctx.set_fonts(get_fonts());
                Ok(Box::new(App::new(cc, from_backend, to_backend)))
            }),
        )
    })
}
