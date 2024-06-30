use std::fs;

use crate::conversation::Conversation;
use crate::GptUi;

pub fn load_all(ui_data: &mut GptUi) {
    let path = ui_data.user_settings.conv_save_location.clone();

    if let Ok(exists) = path.try_exists() {
        if !exists {
            return;
        }
    }

    let dir_contents = fs::read_dir(path.clone()).unwrap();

    dir_contents.for_each(|file_result| {
        if let Ok(file) = file_result {
            let conv = Conversation::load_from_file(file.path(), false);

            ui_data.conversations.push(conv);
        }
    });
}
