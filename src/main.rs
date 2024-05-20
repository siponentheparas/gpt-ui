#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod conversation;
mod ui_modules;
mod user_settings;
mod save_load;

use std::sync::mpsc::{Receiver, Sender};

use conversation::{Conversation, Message};
use eframe::egui;
use ui_modules::{chat_hist_ui, conv_ui};
use user_settings::UserSettings;
use save_load::save::save_all;
use save_load::load::load_all;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        centered: true,
        ..Default::default()
    };
    eframe::run_native("GPT-UI", options, Box::new(|_cc| Box::<GptUi>::default()))
}

struct GptUi {
    conversations: Vec<Conversation>,
    tried_loading_convs: bool,
    conversation_selected: bool,
    selected_conversation_index: usize,
    user_settings: UserSettings,
    show_settings_ui: bool,
    tx: Sender<(Message, usize)>,
    rx: Receiver<(Message, usize)>,
}

impl Default for GptUi {
    fn default() -> Self {
        let user_settings: UserSettings;

        if let Some(settings) = UserSettings::load() {
            user_settings = settings;
        } else {
            user_settings = UserSettings::default();
        }

        let (tx, rx) = std::sync::mpsc::channel::<(Message, usize)>();
        GptUi {
            conversations: vec![],
            tried_loading_convs: false,
            conversation_selected: false,
            selected_conversation_index: 0,
            user_settings,
            show_settings_ui: false,
            tx,
            rx,
        }
    }
}

impl eframe::App for GptUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for (i, conv) in self.conversations.clone().iter().enumerate() {
            if conv.delete {
                if let Some(path) = &conv.file_path {
                    std::fs::remove_file(path).unwrap();
                    println!("Removed conversation: {}", path.to_string_lossy());
                }

                self.conversations.remove(i);
            }
        }

        if !self.tried_loading_convs {
            load_all(self);
            self.tried_loading_convs = true;
            println!("Loaded all saved conversations");
        }

        // Save conversations when close has been requested
        if ctx.input(|i| i.viewport().close_requested()) {
            save_all(self);
            println!("Saved all conversations, can exit.")
        }

        // Theme
        if self.user_settings.dark_theme {
            ctx.style_mut(|style| {
                style.visuals = egui::Visuals::dark();
            });
        } else {
            ctx.style_mut(|style| {
                style.visuals = egui::Visuals::light();
            });
        }

        // The left sidepanel
        egui::SidePanel::left("Chat History")
            .min_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                chat_hist_ui::show_chat_history(ctx, ui, self);
            });

        // Conversation contents
        egui::CentralPanel::default().show(ctx, |ui| {
            conv_ui::show_conversation(ctx, ui, self);
        });
    }
}
