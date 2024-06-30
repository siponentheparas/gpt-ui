use crate::GptUi;

pub fn show_settings(ctx: &egui::Context, _ui: &mut egui::Ui, ui_data: &mut GptUi) {
    egui::Window::new("Settings")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_width(300.0);
            ui.set_height(200.0);

            if ui.button("Toggle Theme").clicked() {
                ui_data.user_settings.dark_theme = !ui_data.user_settings.dark_theme;
            }
            ui.add_space(15.0);

            let save_location_label = ui.label("Conversation Save Location");
            if ui
                .button("Change")
                .labelled_by(save_location_label.id)
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Change Save Location")
                    .set_directory(ui_data.user_settings.conv_save_location.clone())
                    .pick_folder()
                {
                    ui_data.user_settings.conv_save_location = path;
                }
            }

            ui.add_space(5.0);

            ui.label(ui_data.user_settings.conv_save_location.to_string_lossy());

            ui.add_space(15.0);

            ui.vertical_centered(|ui| {
                ui.label("Server Connection Settings");
            });

            let ip_label = ui.label("IP Address: ");
            ui.text_edit_singleline(&mut ui_data.user_settings.server_address.ip)
                .labelled_by(ip_label.id);

            let port_label = ui.label("Port: ");
            ui.text_edit_singleline(&mut ui_data.user_settings.server_address.port)
                .labelled_by(port_label.id);

            ui.add_space(5.0);

            ui.label(format!(
                "http://{}:{}/v1/chat/completions",
                ui_data.user_settings.server_address.ip, ui_data.user_settings.server_address.port
            ));

            ui.add_space(10.0);

            egui::TopBottomPanel::bottom("Settings bottom panel").show_inside(ui, |ui| {
                if ui.button("Close").clicked() {
                    ui_data.user_settings.save();
                    ui_data.show_settings_ui = false;
                }
            });
        });
}
