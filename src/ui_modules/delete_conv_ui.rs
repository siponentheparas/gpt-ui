use crate::conversation::Conversation;

pub fn show_conv_delete(ctx: &egui::Context, conv: &mut Conversation) {
    egui::Window::new("Delete Conversation")
    .resizable(false)
    .collapsible(false)
    .show(ctx, |ui| {
        ui.set_width(280.0);
        ui.set_height(50.0);

        ui.label("Are you sure you want to delete this conversation?");

        ui.horizontal_centered(|ui| {
            if ui.button("No").clicked() {
                conv.show_delete_conv_ui = false;
            }

            if ui.button("Yes").clicked() {
                conv.delete = true;
                conv.show_delete_conv_ui = false;
            }
        });
    });
}