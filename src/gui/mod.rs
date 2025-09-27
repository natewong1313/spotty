use eframe::Error;
use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};
use hello_egui::material_icons;

use crate::gui::app::App;

pub mod app;
pub mod fonts;

pub fn init_app() -> Result<(), Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Demo",
        options,
        Box::new(|cc| {
            material_icons::initialize(&cc.egui_ctx);
            fonts::setup(&cc.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
}
