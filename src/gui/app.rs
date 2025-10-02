use std::sync::Arc;

use eframe::{
    CreationContext,
    egui::{
        self, Button, CentralPanel, Color32, Context, FontFamily, Margin, RichText, SidePanel, Ui,
        Vec2, vec2,
    },
};
use egui_phosphor::regular::{
    HOUSE_LINE, LIST_MAGNIFYING_GLASS, MICROPHONE, PLAYLIST, USERS_THREE, VINYL_RECORD,
};
use flume::{Receiver, Sender};

use crate::client::client::SpotifyClient;
use crate::shared::message::{BackendMessage, GuiMessage};

pub struct App {
    search_text: String,
    selected_nav: String,
    from_backend: Receiver<GuiMessage>,
    to_backend: Sender<BackendMessage>,
    count: u32,
}

impl App {
    pub fn new(
        cc: &CreationContext,
        from_backend: Receiver<GuiMessage>,
        to_backend: Sender<BackendMessage>,
    ) -> Self {
        {
            let ctx = cc.egui_ctx.clone();
            subsecond::register_handler(Arc::new(move || ctx.request_repaint()));
        }

        Self {
            selected_nav: "Home".to_string(),
            search_text: "".to_string(),
            from_backend,
            to_backend,
            count: 0,
        }
    }
}

impl App {
    fn render(&mut self, ctx: &Context) {
        subsecond::call(|| {
            while let Ok(msg) = self.from_backend.try_recv() {
                self.handle_backend_message(msg);
            }

            SidePanel::left("sidebar")
                .resizable(false)
                .frame(egui::containers::Frame {
                    inner_margin: Margin::symmetric(10, 10),
                    fill: Color32::from_rgb(18, 18, 18),
                    ..Default::default()
                })
                .show(ctx, |ui| {
                    self.ui_sidebar(ui);
                });

            CentralPanel::default()
                .frame(egui::containers::Frame::default().fill(Color32::from_rgb(18, 18, 18)))
                .show(ctx, |ui| {
                    // Main content here
                    ui.label(format!("The value is {}!", self.search_text));
                    ui.label(format!("The value is {}!", self.count));
                });
        })
    }

    fn ui_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.style_mut().spacing.button_padding = vec2(10.0, 6.0);

        // Logo
        //
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new("Spotty!")
                    .size(22.0)
                    .strong()
                    .color(Color32::from_rgb(29, 185, 84))
                    .family(FontFamily::Name("ComicSans".into())),
            );
        });
        ui.add_space(8.0);
        // Nav items
        let nav_items = vec![
            (HOUSE_LINE, "Home"),
            (PLAYLIST, "Playlists"),
            (VINYL_RECORD, "Albums"),
            (MICROPHONE, "Podcasts"),
            (USERS_THREE, "Artists"),
            (LIST_MAGNIFYING_GLASS, "Discover"),
        ];
        for (icon, name) in nav_items {
            self.ui_nav_item(ui, icon, name);
        }
    }

    fn ui_nav_item(&mut self, ui: &mut Ui, icon: &str, name: &str) {
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
                        .family(FontFamily::Name("phosphor-fill".into()).to_owned()),
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

    fn handle_backend_message(&mut self, msg: GuiMessage) {
        match msg {
            GuiMessage::UserProfileLoaded => {
                println!("profile loaded");
                self.count += 1;
            }
        }
    }

    pub fn send_to_backend(&self, msg: BackendMessage) {
        let _ = self.to_backend.send(msg);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        subsecond::call(|| {
            self.render(ctx);
        });
    }
}
