use crate::gui::app::App;
use crate::gui::fonts::get_fonts;
use eframe::{Error, NativeOptions};

pub mod app;
pub mod fonts;

pub fn init_app() -> Result<(), Error> {
    dioxus_devtools::connect_subsecond();

    subsecond::call(|| {
        eframe::run_native(
            "egui Demo",
            NativeOptions::default(),
            Box::new(move |cc| {
                cc.egui_ctx.set_fonts(get_fonts());
                Ok(Box::new(App::new(cc)))
            }),
        )
    })
}
