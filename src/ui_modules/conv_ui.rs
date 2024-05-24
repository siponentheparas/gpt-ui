use crate::GptUi;

use egui::{Align, Color32, Layout, Vec2};

use super::delete_conv_ui::show_conv_delete;
use super::rename_conv_ui::show_rename_conv;

/// conv UI
pub fn show_conversation(ctx: &egui::Context, ui: &mut egui::Ui, ui_data: &mut GptUi) {
    if ui_data.conversations.len() > 0 && ui_data.conversation_selected {
        ui.vertical_centered_justified(|ui| {
            let conv_index = ui_data.selected_conversation_index;
            let conv = &mut ui_data.conversations[conv_index];

            if conv.show_delete_conv_ui {
                show_conv_delete(&ctx, conv);
            }

            let sender: String;
            let send_button_text: String;

            if conv.pre_query_entered {
                if let Some(pre_query_msg) = conv.get_pre_query_message() {
                    ui.label(format!("Pre-Query Message: \"{}\"", pre_query_msg.content));
                } else {
                    ui.label("No Pre-Query Message");
                }
            }

            egui::ScrollArea::vertical()
                // The height of the bottom panel has to be reduced from the availalbe height. 
                // Otherwise the scrollarea will go under the bottom panel.
                .max_height(ui.available_height() - 60.0)
                .max_width(ui.available_width())
                .show(ui, |ui| {
                    for msg in &conv.messages {
                        if msg.role == "system" {
                            continue;
                        }

                        // message sender and left/right aling
                        let ui_name: String;
                        let message_layout: Layout;

                        if msg.role == "assistant" {
                            ui_name = "AI".to_owned();
                            message_layout = Layout::left_to_right(Align::TOP);
                        } else {
                            ui_name = "you".to_owned();
                            message_layout = Layout::right_to_left(Align::TOP);
                        }

                        // dark/white theme to message background.
                        let msg_bgr: Color32;

                        if ui_data.user_settings.dark_theme {
                            msg_bgr = Color32::BLACK;
                        } else {
                            msg_bgr = Color32::LIGHT_GRAY;
                        }

                        ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 30.0 };

                        ui.with_layout(message_layout, move |ui| {
                            egui::Frame::none().fill(msg_bgr).show(ui, |ui| {
                                ui.set_min_size(Vec2 { x: 750.0, y: 50.0 });
                                ui.set_max_size(Vec2 { x: 750.0, y: 50.0 });
                                ui.wrap_text();

                                ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 10.0 };

                                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                                    ui.heading(ui_name);
                                    ui.label(&msg.content);
                                });
                            });
                        });
                    }
                });

            if !conv.pre_query_entered {
                ui.label("Input Pre-Query Message");
                sender = "system".to_owned();

                // The send button text.
                // If currently inputing the pre-query message and the text field is empty, then set button text as "Skip". Otherwise the button text is "Send".
                if conv.editor_text.is_empty() {
                    send_button_text = "Skip".to_owned();
                } else {
                    send_button_text = "Send".to_owned();
                }
            } else {
                sender = "user".to_owned();
                send_button_text = "Send".to_owned();
            }

            let send_button_enabled: bool;

            let mut send_error_message: String = "".to_owned();

            if conv.pre_query_entered
                && !conv.generating
                && (conv.messages.len() > 0 && conv.messages[conv.messages.len() - 1].role == "user")
            {
                send_button_enabled = true;
                send_error_message =
                    "Failed to send message. Check server connection and try again.".to_owned();
            } else if (conv.generating || conv.editor_text.trim().is_empty())
                && conv.pre_query_entered
            {
                send_button_enabled = false;
            } else {
                send_button_enabled = true;
            }

            if conv.generating {
                ctx.request_repaint();
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            egui::TopBottomPanel::bottom("input field panel")
                .min_height(60.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            // TODO: figure out how to make code editor more visible in light mode
                            ui.add_sized(
                                [400.0, 50.0],
                                egui::TextEdit::multiline(&mut conv.editor_text),
                            );

                            ui.horizontal(|ui| {
                                if ui
                                    .add_enabled(
                                        send_button_enabled,
                                        egui::Button::new(send_button_text),
                                    )
                                    .clicked()
                                {
                                    conv.add_message(sender, conv.editor_text.to_owned());
                                    // println!("{:?}", &conv);
                                    conv.editor_text = "".to_owned();
                                    conv.send(
                                        ui_data.tx.clone(),
                                        ui_data.selected_conversation_index,
                                        ui_data.user_settings.server_address.clone(),
                                    );
                                }
                                ui.label(
                                    egui::RichText::new(send_error_message).color(Color32::RED),
                                );
                            });
                        });

                        ui.vertical(|ui| {
                            ui.style_mut().spacing.item_spacing = Vec2 { x: 0.0, y: 10.0 };

                            if ui.button("Rename").clicked() {
                                conv.show_rename_conv_ui = true;
                            }

                            if conv.show_rename_conv_ui {
                                show_rename_conv(&ctx, ui, conv);
                            }

                            if ui.button("Delete").clicked() {
                                conv.show_delete_conv_ui = true;
                            }

                            if ui.button("Export To File").clicked() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("json", &["json"])
                                    .set_title("Export conv To File")
                                    .set_file_name(conv.title.clone())
                                    .save_file()
                                {
                                    conv.clone().save_to_file(path.clone());
                                }
                            }
                        });
                    })
                });

            // A story:
            // Past problem: When the user asks the AI a question in a conversation and then selects another conversation.
            // The AI response of the first (correct) conversation would go to the second (wrong) conversation.
            // Fixed this by sending the conversation index as an argument into the conversation's send method.
            // The send method's thread will then return a tuple (Message, usize) where the usize variable is the conversation index to put the response to.
            // Probably better ways to do this, but this was the first solution that came to mind.
            if let Ok((message, conv_index)) = ui_data.rx.try_recv() {
                if message.role == "error" {
                    eprintln!("Failed to send message to AI:\n{}", message.content);

                    let conv = &mut ui_data.conversations[conv_index];
                    conv.generating = false;
                } else {
                    let conv = &mut ui_data.conversations[conv_index];
                    conv.add_message(message.role, message.content);
                    println!("conv:\n{:?}", conv);
                    conv.generating = false;
                }
            }
        });
    } else {
        ui.heading("Select or create a new conv from the left");
    }
}
