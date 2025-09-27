use std::sync::Arc;

use eframe::egui::{
    self, Button, CentralPanel, Color32, FontFamily, Frame, Margin, RichText, SidePanel, Stroke,
    TextEdit, Vec2, Widget, vec2,
};
use egui_phosphor::regular::{
    HOUSE_LINE, LIST_MAGNIFYING_GLASS, MAGNIFYING_GLASS, MICROPHONE, PLAYLIST, USERS_THREE,
    VINYL_RECORD,
};

pub struct App {
    search_text: String,
    selected_nav: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_text: "".to_string(),
            selected_nav: "Home".to_string(),
        }
    }
}

impl App {
    fn sidebar(&mut self, ui: &mut egui::Ui) {
        let fill_font = FontFamily::Name("phosphor-fill".into());

        ui.style_mut().spacing.button_padding = vec2(10.0, 6.0);

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new("Spotty")
                    .size(22.0)
                    .strong()
                    .color(Color32::from_rgb(29, 185, 84))
                    .family(FontFamily::Name("ComicSans".into())),
            );
        });

        ui.add_space(16.0);

        // Frame::default()
        //     .stroke(Stroke::new(1.0, Color32::from_rgb(83, 83, 83)))
        //     .corner_radius(4.0)
        //     .inner_margin(Margin::symmetric(8, 4))
        //     .show(ui, |ui| {
        //         ui.horizontal(|ui| {
        //             ui.label(RichText::new(MAGNIFYING_GLASS).family(regular_font));
        //             ui.add(
        //                 TextEdit::singleline(&mut self.search_text)
        //                     .hint_text("Search")
        //                     .frame(false),
        //             );
        //         });
        //     });
        //
        // ui.add_space(10.0);

        let nav_items = vec![
            (HOUSE_LINE, "Home"),
            (PLAYLIST, "Playlists"),
            (VINYL_RECORD, "Albums"),
            (MICROPHONE, "Podcasts"),
            (USERS_THREE, "Artists"),
            (LIST_MAGNIFYING_GLASS, "Discover"),
        ];

        for (icon, name) in nav_items {
            let is_selected = self.selected_nav == name;
            if ui
                .add(
                    Button::new(
                        RichText::new(format!("{}  {}", icon, name))
                            .size(16.0)
                            .color(if is_selected {
                                Color32::WHITE
                            } else {
                                Color32::from_rgb(83, 83, 83)
                            })
                            .family(fill_font.to_owned()),
                    )
                    .fill(if is_selected {
                        Color32::from_rgb(33, 33, 33)
                    } else {
                        Color32::TRANSPARENT
                    })
                    .min_size(Vec2::new(200.0, 32.0)),
                )
                .clicked()
            {
                self.selected_nav = name.to_string();
            }
        }

        // ui.add_space(10.0);
        //
        // ui.separator();
        // ui.add_space(10.0);
        //
        // ui.horizontal(|ui| {
        //     ui.add_space(10.0);
        //     ui.label(
        //         egui::RichText::new("Pinned")
        //             .color(Color32::from_rgb(83, 83, 83))
        //             .size(12.0),
        //     );
        // });
        //
        // ui.add_space(10.0);
        //
        // ui.separator();
        // ui.add_space(10.0);
        //
        // ui.horizontal(|ui| {
        //     ui.add_space(10.0);
        //     ui.label(
        //         egui::RichText::new("My Library")
        //             .color(Color32::from_rgb(83, 83, 83))
        //             .size(12.0),
        //     );
        // });
        //
        // ui.add_space(10.0);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .frame(egui::containers::Frame {
                inner_margin: Margin::symmetric(10, 10),
                fill: Color32::from_rgb(18, 18, 18),
                ..Default::default()
            })
            .show(ctx, |ui| {
                self.sidebar(ui);
            });

        CentralPanel::default()
            .frame(egui::containers::Frame::default().fill(Color32::from_rgb(18, 18, 18)))
            .show(ctx, |ui| {
                // Main content here
                ui.label(format!("The value is {}", self.search_text));
            });
    }
}
