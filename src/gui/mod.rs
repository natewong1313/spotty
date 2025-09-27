use std::sync::Arc;

use eframe::Error;
use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};
use hello_egui::material_icons;

use crate::gui::app::App;
use crate::gui::fonts::get_fonts;

pub mod app;
pub mod fonts;

pub fn init_app() -> Result<(), Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(get_fonts());
            Ok(Box::new(App::default()))
        }),
    )
}
