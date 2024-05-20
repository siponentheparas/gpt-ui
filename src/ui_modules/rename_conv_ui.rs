use crate::conversation::Conversation;

pub fn show_rename_conv(ctx: &egui::Context, _ui: &mut egui::Ui, conv: &mut Conversation) {
    egui::Window::new("Rename Conversation")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui: &mut egui::Ui| {
                ui.text_edit_singleline(&mut conv.rename_text);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        conv.rename_text = "".to_owned();
                        conv.show_rename_conv_ui = false;
                    }

                    if ui.button("Rename").clicked() {
                        if conv.rename_text.is_empty() {
                            conv.rename_text = "".to_owned();
                            conv.show_rename_conv_ui = false;
                        } else {
                            conv.title = conv.rename_text.clone();
                            conv.rename_text = "".to_owned();
                            conv.show_rename_conv_ui = false;
                        }
                    }
                });
            });
        });
}
