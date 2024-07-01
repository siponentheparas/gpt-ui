use std::{fs, path::PathBuf};

use crate::conversation::Conversation;
use crate::list::List;
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
            if file.file_name() != "gpt-ui_lists.json" {
                let conv = Conversation::load_from_file(file.path(), false);

                ui_data.conversations.push(conv);
            }
        }
    });
}

pub fn load_lists(path: &PathBuf) -> Option<Vec<List>> {
    let file_path = path.join("gpt-ui_lists.json");

    if file_path.exists() {
        match fs::read_to_string(file_path) {
            Ok(file) => {
                let lists: Vec<List> = serde_json::from_str(&file).unwrap();

                Some(lists)
            },

            Err(e) => {
                eprintln!("Failed to read lists file: {}", e);
                None
            }
        }

    } else {
        println!("lists file does not exist");
        None
    }
}
