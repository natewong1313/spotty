use std::sync::Arc;

use eframe::Frame;
use eframe::egui::SidePanel;
use eframe::{
    CreationContext,
    egui::{
        self, Align, Button, CentralPanel, Color32, Context, FontFamily, FontId, Image, Layout,
        Margin, RichText, Stroke, TextEdit, Ui, Vec2, vec2,
    },
};
use egui_phosphor::regular::{
    HOUSE_LINE, LIST_MAGNIFYING_GLASS, MAGNIFYING_GLASS, MICROPHONE, PLAYLIST, USERS_THREE,
    VINYL_RECORD,
};
use flume::{Receiver, Sender};
use hello_egui::flex::{Flex, FlexAlignContent, FlexItem, item};
use poll_promise::Promise;
use rspotify::model::PlayHistory;

use crate::shared::message::{BackendMessage, GuiMessage};
use crate::{client::client::SpotifyClient, shared::message::UserProfile};

pub struct App {
    search_text: String,
    selected_nav: String,
    from_backend: Receiver<GuiMessage>,
    to_backend: Sender<BackendMessage>,
    user_profile: Option<UserProfile>,
    recently_played: Option<Vec<PlayHistory>>,
}

impl App {
    pub fn new(
        cc: &CreationContext,
        from_backend: Receiver<GuiMessage>,
        to_backend: Sender<BackendMessage>,
    ) -> Self {
        {
            let ctx = cc.egui_ctx.clone();
            egui_extras::install_image_loaders(&ctx);
            subsecond::register_handler(Arc::new(move || ctx.request_repaint()));
        }

        Self {
            selected_nav: "Home".to_string(),
            search_text: "".to_string(),
            from_backend,
            to_backend,
            user_profile: None,
            recently_played: None,
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
                .frame(egui::containers::Frame {
                    inner_margin: Margin::symmetric(8, 8),
                    fill: Color32::from_rgb(18, 18, 18),
                    ..Default::default()
                })
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        // Top bar
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            if let Some(profile) = &self.user_profile {
                                ui.add(
                                    Image::new(profile.profile_img.clone())
                                        .corner_radius(100)
                                        .fit_to_fraction(Vec2::new(100.0, 100.0))
                                        .max_size(Vec2::new(36.0, 36.0)),
                                );
                            }

                            let available_size = ui.available_size();
                            // Annoying search input
                            ui.allocate_ui(available_size, |ui| {
                                egui::Frame::group(ui.style())
                                    .stroke(Stroke::NONE)
                                    .fill(Color32::from_rgb(33, 33, 33))
                                    .inner_margin(Margin::symmetric(8, 8))
                                    .corner_radius(6)
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.with_layout(
                                                Layout::left_to_right(Align::Center),
                                                |ui| {
                                                    ui.label(
                                                        RichText::new(MAGNIFYING_GLASS)
                                                            .size(20.0)
                                                            .family(FontFamily::Name(
                                                                "phosphor".into(),
                                                            )),
                                                    );
                                                    ui.add_sized(
                                                        ui.available_size(),
                                                        egui::TextEdit::singleline(
                                                            &mut self.search_text,
                                                        )
                                                        .font(FontId::new(
                                                            16.0,
                                                            FontFamily::Proportional,
                                                        ))
                                                        .hint_text("Search")
                                                        .frame(false),
                                                    );
                                                },
                                            );
                                        });
                                    });
                            });
                        });
                        // Content
                        if let Some(recently_played) = &self.recently_played {
                            for played in recently_played.iter() {
                                // ui.label(played.track.name.clone());
                            }
                        }
                    });
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
            GuiMessage::UserProfileLoaded(profile) => {
                println!("profile loaded");
                println!("name {}", profile.name);
                println!("image {}", profile.profile_img);
                self.user_profile = Some(profile);
            }
            GuiMessage::UserPlaylistsLoaded(simplified_playlists) => {
                for playlist in &simplified_playlists {
                    // println!("{}", playlist.name);
                }
            }
            GuiMessage::UserRecentlyPlayed(items) => {
                for playlist in &items {
                    println!("{}", playlist.track.name);
                }
                self.recently_played = Some(items)
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
