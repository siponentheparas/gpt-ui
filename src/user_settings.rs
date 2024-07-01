use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

use dirs::data_dir;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSettings {
    /// Whether to save conversations automatically or not.
    pub auto_save: bool,

    /// The location to save these conversations.
    pub conv_save_location: PathBuf,

    /// Connection URL to use when querying the AI.
    pub server_address: ServerAddress,

    /// Dark theme
    pub dark_theme: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        let conv_save_dir = data_dir().unwrap().join("gpt-ui").join("conversations");
        let address = ServerAddress::default();

        UserSettings {
            auto_save: true,
            conv_save_location: conv_save_dir,
            server_address: address,
            dark_theme: true,
        }
    }
}

impl UserSettings {
    /// Save user settings as a .json file next to the executable file.
    pub fn save(&self) {
        let mut save_path = env::current_exe().unwrap();
        let _ = save_path.pop();

        let data_json = serde_json::to_string(&self).unwrap();

        fs::write(save_path.join("gpt-ui_user_settings.json"), data_json).unwrap();

        println!(
            "Settings saved in: {}",
            save_path
                .join("gpt-ui_user_settings.json")
                .to_string_lossy()
        );
    }

    /// Load user settings from a .json file next to the executable file.
    pub fn load() -> Option<UserSettings> {
        let mut load_path = env::current_exe().unwrap();
        let _ = load_path.pop();

        if let Ok(data_json) = fs::read_to_string(load_path.join("gpt-ui_user_settings.json")) {
            let settings: UserSettings = serde_json::from_str(&data_json).unwrap();

            println!(
                "Loaded user settings from {}\\gpt-ui_user_settings.json",
                load_path
                    .join("gpt-ui_user_settings.json")
                    .to_string_lossy()
            );
            Some(settings)
        } else {
            println!("Did not find user settings file");
            None
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerAddress {
    pub ip: String,
    pub port: String,
}

impl Default for ServerAddress {
    fn default() -> ServerAddress {
        ServerAddress {
            ip: "127.0.0.1".to_owned(),
            port: "5000".to_owned(),
        }
    }
}
