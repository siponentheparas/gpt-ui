use std::fs;

use crate::conversation::Conversation;
use crate::GptUi;

pub fn save_all(ui_data: &mut GptUi) {
    println!("Saving conversations to user settings save location");

    for conv in ui_data.conversations.clone() {
        if let Err(e) = save_conversation(conv.clone(), ui_data) {
            eprintln!("Error while saving conversation with title \"{}\"\n{}", conv.title, e);
        }
    }
}

pub fn save_conversation(mut conv: Conversation, ui_data: &mut GptUi) -> Result<bool, String> {
    let path = ui_data.user_settings.conv_save_location.clone();

    if let Ok(exists) = path.try_exists() {
        if !exists {
            std::fs::DirBuilder::new()
                .recursive(true)
                .create(&path)
                .unwrap();
        }
    }

    if let Some(mut conv_path) = conv.file_path.clone() {
        let file_name: String;

        if let Some(name_file) = conv_path.file_name() {
            file_name = name_file.to_string_lossy().to_string();
        } else {
            file_name = format!("{}.json", conv.title);
        }

        conv_path.pop();

        let conv_save_location = ui_data.user_settings.conv_save_location.clone();

        if conv_path != conv_save_location{
            conv.file_path = Some(conv_save_location.join(file_name));
        }
    }
    

    if let Some(file_path) = conv.file_path.clone() {
        conv.clone().save_to_file(file_path);
        return Ok(true);
    } else {
        let title = conv.title.clone();
        let file_path = path.join(format!("{}.json", title));

        match file_path.try_exists() {
            Ok(exists) => {
                if !exists {
                    conv.clone().save_to_file(file_path);
                } else {
                    let dir_contents = fs::read_dir(path.clone()).unwrap();

                    let mut same_name_file_count = 0;

                    dir_contents.for_each(|file_result| {
                        if let Ok(file) = file_result {
                            if file.file_name().to_string_lossy().starts_with(&conv.title) {
                                same_name_file_count += 1;
                            }
                        }
                    });

                    let save_file_name = format!(
                        "{}{}.json",
                        conv.title.clone(),
                        same_name_file_count.to_string()
                    );
                    conv.clone().save_to_file(path.join(save_file_name));
                }

                return Ok(true);
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}
