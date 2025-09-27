use eframe::egui::{
    self, Button, CentralPanel, Color32, FontFamily, Frame, Margin, RichText, SidePanel, Stroke,
    TextEdit, Vec2,
};
use hello_egui::material_icons::icons::{ICON_HOME, ICON_LIBRARY_MUSIC, ICON_SEARCH};

#[derive(Default)]
pub struct App {
    search_text: String,
    selected_nav: String,
}

impl App {
    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new("Spotty")
                    .size(18.0)
                    .strong()
                    .color(Color32::from_rgb(29, 185, 84))
                    .family(FontFamily::Name("ComicSans".into())),
            );
        });

        ui.add_space(20.0);

        Frame::default()
            .stroke(Stroke::new(1.0, Color32::from_rgb(83, 83, 83)))
            .corner_radius(4.0)
            .inner_margin(Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(ICON_SEARCH);
                    ui.add(
                        TextEdit::singleline(&mut self.search_text)
                            .hint_text("Search")
                            .frame(false),
                    );
                });
            });

        ui.add_space(10.0);

        // Navigation items
        let nav_items = vec![(ICON_HOME, "Home"), (ICON_LIBRARY_MUSIC, "Playlists")];

        for (icon, name) in nav_items {
            let is_selected = self.selected_nav == name;
            let text_color = if is_selected {
                Color32::WHITE
            } else {
                Color32::GRAY
            };

            if ui
                .add(
                    egui::Button::new(format!("{} {}", icon, name))
                        .fill(if is_selected {
                            Color32::from_rgb(60, 60, 60)
                        } else {
                            Color32::TRANSPARENT
                        })
                        .rounding(6.0)
                        .min_size(Vec2::new(180.0, 32.0)),
                )
                .clicked()
            {
                self.selected_nav = name.to_string();
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(200.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                self.sidebar(ui);
            });

        CentralPanel::default().show(ctx, |ui| {
            // Main content here
            ui.label(format!("The value is {}", self.search_text));
        });
    }
}
