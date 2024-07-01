use std::{
    fs::{self, read_to_string},
    path::PathBuf,
    sync::mpsc::Sender,
    time::Duration,
};

use rand::Rng;
use reqwest::header;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user_settings::ServerAddress;

/// Hold information about a conversation
///
/// ### INFO
/// `pre_query_entered` tracks if user has input or skipped the pre-query message.
/// So `pre_query_entered` might be true, even if the conversation does not have a pre-query message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    /// The name of the conversation.
    /// Program will generate a fun name when creating a new conversation.
    /// The user can rename a conversation later
    pub title: String,

    /// Messages to send to the AI.
    pub messages: Vec<Message>,

    /// [true] if the user has entered or skipped the pre-query message.
    pub pre_query_entered: bool,

    /// [true] if currently waiting for AI to respond.
    pub generating: bool,

    /// The text that is inside the users input field.
    /// Stored here so the input field wont clear if user wants to visit different conversations.
    /// Also this is saved to file, so if user quits the program, the input field text will be loaded from file.
    pub editor_text: String,

    /// [Conversation]'s filepath if it's saved
    #[serde(skip_serializing, skip_deserializing)]
    pub file_path: Option<PathBuf>,

    #[serde(skip_serializing, skip_deserializing)]
    /// Whether to show the [Conversation] renaming window
    pub show_rename_conv_ui: bool,

    #[serde(skip_serializing, skip_deserializing)]
    /// Wheter to show the [Conversation}] deleting window
    pub show_delete_conv_ui: bool,

    #[serde(skip_serializing, skip_deserializing)]
    /// Marked for deletion
    pub delete: bool,

    /// Input text from the renaming window, to later put into title, or discard if cancelled.
    pub rename_text: String,

    /// A unique id to conversations. Used to determine which list the conversation belongs to.
    pub uuid: Uuid,

    max_tokens: i32,
    temperature: i32,
    seed: i32,
    max_output_lenght: i32,
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            title: "unnamed".to_owned(),
            messages: vec![],
            pre_query_entered: false,
            generating: false,
            editor_text: "".to_owned(),
            file_path: None,
            show_rename_conv_ui: false,
            show_delete_conv_ui: false,
            delete: false,
            rename_text: "".to_owned(),
            uuid: Uuid::new_v4(),
            max_tokens: 500,
            temperature: 0,
            seed: rand::thread_rng().gen_range(1000..=9999),
            max_output_lenght: 5120,
        }
    }
}

impl Conversation {
    #[allow(dead_code)]
    /// Create new conversation.
    pub fn new(messages: Vec<Message>, pre_query_entered: bool) -> Self {
        Self {
            messages,
            pre_query_entered,
            ..Default::default()
        }
    }

    /// Adds a message to the conversation
    pub fn add_message(&mut self, role: String, content: String) -> &Self {
        println!("add_message called");

        if role == "system" {
            self.pre_query_entered = true;
            if content.trim().is_empty() {
                return self;
            }
        } else if role == "user" && content.is_empty() {
            return self;
        }
        self.messages.push(Message::new(role, content));
        self
    }

    /// Send the conversation to the AI to generate a response.
    ///  
    /// Returns a [Message]
    pub fn send(
        &mut self,
        tx: Sender<(Message, usize)>,
        conv_index: usize,
        server_address: ServerAddress,
    ) {
        println!("Send called");

        // Send gets called from conv_ui.rs even if there is no user message yet.
        // It's fixable from con_ui.rs, but it takes time and a lot of thinking.
        //
        // This is a horrible fix.
        // TODO: Find a better fix for this

        let last_sender = if !self.messages.is_empty() {
            &self.messages[self.messages.len() - 1].role
        } else {
            return;
        };

        // If last sender is "user" then proceed with sending, otherwise do nothing.

        if self.pre_query_entered && last_sender == "user" {
            let post_data = serde_json::to_string(&ConvSendToAI::from(self.clone())).unwrap();
            self.generating = true;

            let address = format!(
                "http://{}:{}/v1/chat/completions",
                server_address.ip, server_address.port
            );

            let _ = std::thread::Builder::new()
                .name("Curl request".to_string())
                .spawn(move || {
                    println!("Sending query");
                    println!("\"{}\"", post_data);

                    let mut headers = header::HeaderMap::new();
                    headers.insert("Content-Type", "application/json".parse().unwrap());

                    let client = reqwest::blocking::Client::builder()
                        .redirect(reqwest::redirect::Policy::none())
                        .timeout(Duration::from_secs(10 * 60)) // 10 minute timeout
                        .build()
                        .unwrap();

                    match client.post(address).headers(headers).body(post_data).send() {
                        Ok(response) => {
                            let response_object: AiResponse =
                                serde_json::from_str(&response.text().unwrap()).unwrap();
                            let message = response_object.choices[0].message.clone();

                            println!("\nRecieved response message");
                            println!("{:?}", message);

                            if let Err(e) = tx.send((message, conv_index)) {
                                eprintln!("Message from thread to main thread failed: {}", e);
                            }
                        }

                        Err(e) => {
                            let fail_message = Message {
                                role: "error".to_owned(),
                                content: e.to_string(),
                            };

                            if let Err(e) = tx.send((fail_message, conv_index)) {
                                eprintln!("Message from thread to main thread failed: {}", e);
                            }
                        }
                    }
                });
        };
    }

    /// If the first [Message] is a system message then return the [Message]
    /// If not, then return a system role [`Option<&Message>`] with empty content
    pub fn get_pre_query_message(&self) -> Option<&Message> {
        if !self.messages.is_empty() {
            let first_message = &self.messages[0];

            if first_message.role == "system" {
                return Some(first_message);
            }
        }

        None
    }

    pub fn load_from_file(file_path: PathBuf, is_import: bool) -> Conversation {
        let json_data = read_to_string(&file_path).unwrap();

        let mut conv: Conversation = serde_json::from_str(&json_data).unwrap();

        if !is_import {
            conv.file_path = Some(file_path.clone());
        }

        let components: Vec<_> = file_path.iter().collect();

        let last_two_items = &components[components.len() - 4..components.len()];

        let last_two_string: Vec<String> = last_two_items
            .iter()
            .map(|&s| s.to_str().unwrap_or("Invalid UTF-8").to_string())
            .collect();

        println!(
            "Loaded conversation from file: {}",
            last_two_string.join("/")
        );

        conv
    }

    pub fn save_to_file(self, file_path: PathBuf) -> Self {
        let json_data = serde_json::to_string(&self).unwrap();

        fs::write(&file_path, json_data).unwrap();

        let components: Vec<_> = file_path.iter().collect();

        let last_two_items = &components[components.len() - 4..components.len()];

        let last_two_string: Vec<String> = last_two_items
            .iter()
            .map(|&s| s.to_str().unwrap_or("Invalid UTF-8").to_string())
            .collect();

        println!("Saved conversation to file: {}", last_two_string.join("/"));

        self
    }
}

/// Hold Message information.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Message {
    /// Role of the message sender. (system, user, assistant)
    pub role: String,
    /// Message content.
    pub content: String,
}

impl Message {
    /// Craete new message with role and content.
    fn new(role: String, content: String) -> Self {
        Self { role, content }
    }
}

// This is horrible too (Angy emoji).
// I have to make a new struct for serializing data to send to the AI.
// I could have used the [Conversation] struct and serialize that, but when serializing fields that the AI doesn't need, it gets angry and errors.
// I could have put a #[serde(skip_serialize)] attribute, but it would skip serializing fields I wanted to save to a json file.
// There is a #[serde(skip_serialize_if)] attribute, but I don't know if I can conditionally use it. It takes in a function that return true or false, and based on that, it serializes.
// But I don't know if it's possible to decide when to serialize and when not to serialize. I made this temporary solution, so I can continue and not make my head explode.
// Argh.
#[derive(Serialize)]
struct ConvSendToAI {
    /// Messages to send to the AI.
    messages: Vec<Message>,
    max_tokens: i32,
    temperature: i32,
    seed: i32,
    max_output_lenght: i32,
}

impl From<Conversation> for ConvSendToAI {
    fn from(conv: Conversation) -> Self {
        ConvSendToAI {
            messages: conv.messages,
            max_tokens: conv.max_tokens,
            temperature: conv.temperature,
            seed: conv.seed,
            max_output_lenght: conv.max_output_lenght,
        }
    }
}

/// Struct to deserialize the AI response to.
#[allow(unused)]
#[derive(Deserialize)]
struct AiResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Choice {
    index: i32,
    finish_reason: String,
    message: Message,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}
