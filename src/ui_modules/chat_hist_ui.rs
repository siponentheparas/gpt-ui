use crate::list::List;
use crate::ui_modules::settings_ui::show_settings;
use crate::Conversation;
use crate::GptUi;

use egui::{Button, Vec2};

/// Chat history UI
pub fn show_chat_history(ctx: &egui::Context, ui: &mut egui::Ui, ui_data: &mut GptUi) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if ui.button("New Chat").clicked() {
                ui_data.conversations.push(Conversation::default());
                ui_data.selected_conversation_index = ui_data.conversations.len() - 1;
                ui_data.conversation_selected = true;
            }

            if ui.button("Load From File").clicked() {
                if let Some(paths) = rfd::FileDialog::new()
                    .add_filter("json", &["json"])
                    .set_title("Load Conversation From File")
                    .pick_files()
                {
                    for path in paths {
                        let conv = Conversation::load_from_file(path.clone(), true);
                        ui_data.conversations.push(conv);
                        ui_data.selected_conversation_index = ui_data.conversations.len() - 1;
                        ui_data.conversation_selected = true;
                    }
                }
            }

            if ui.button("New List").clicked() {
                ui_data.lists.push(List::new("New List".to_owned()));

                println!("new list created!");
            }
        });

        // Chat history scroll area
        ui.label("Chat History");
        egui::ScrollArea::vertical()
            .drag_to_scroll(true)
            .auto_shrink(false)
            // The height of the bottom panel has to be reduced from the availalbe height.
            // Otherwise the scrollarea will go under the bottom panel.
            .max_height(ui.available_height() - 20.0)
            .show(ui, |ui| {

                // List buttons
                
                let list_size = Vec2 {
                    x: ui.available_width(),
                    y: 20.0,
                };
            
                for list in &ui_data.lists {
                    if ui.add_sized(list_size, Button::new("List button")).clicked() {
                        println!("List clicked. Name: {}", list.list_name);
                    }
                }

                // Conversation buttons
                // Counter for conversation index.
                let mut conv_index = 0;

                let conv_button_size = Vec2 {
                    x: ui.available_width(),
                    y: 40.0,
                };

                #[allow(clippy::explicit_counter_loop)]
                for conv in &ui_data.conversations {
                    let button_text = if conv.title == "unnamed" {
                        format!("{}{}", conv.title, conv_index)
                    } else {
                        conv.title.to_owned()
                    };

                    if ui
                        .add_sized(conv_button_size, Button::new(button_text))
                        .clicked()
                    {
                        if ui_data.conversation_selected
                            && ui_data.selected_conversation_index == conv_index
                        {
                            ui_data.conversation_selected = false;
                        } else {
                            ui_data.selected_conversation_index = conv_index;
                            ui_data.conversation_selected = true;
                        }
                    }

                    conv_index += 1;
                }
            });

        // Quick settings for quickly setting settings, like theme
        egui::TopBottomPanel::bottom("quick_settings")
            .min_height(20.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.button("Settings").clicked() {
                        ui_data.show_settings_ui = true;
                    }
                });
            });
    });

    if ui_data.show_settings_ui {
        show_settings(ctx, ui, ui_data);
    }
}
