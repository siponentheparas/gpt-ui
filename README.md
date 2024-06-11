# GPT-UI
An easy to use client GUI for H2OGPT servers.

## Features
- Multi-threading
  - When sending a message to AI, program will spawn a new thread to wait for the response from AI. Why? So the UI won't freeze. Also, windows gets no response so windows will crash the app.
- Conversations
  - You can can have multiple conversations with the AI.
- Settings
  - Theme toggle (Light, Dark)
  - Conversation save location changing
  - H2OGPT server ip and port changing
- Conversation saving
  - Conversations are saved when you exit the app. So, please don't crash it, or you will lose progress.
- Conversation exporting and loading
  - You can export a conversation to a .json file
  - You can load a conversation from a .json file
- Conversation renaming
  - If you don't like the name "unnamed", you can change it.
- Delete conversations
  - If you want to get rid of your secret conversation 😉, you can delete them.

Screenshot of GPT-UI
![Screenshot of GPT-UI](gpt-ui.png "GPT-UI")

## Build from source
### Prerequisites:
- Rust toolkit from https://rustup.rs/
. Contains install command for linux and installer for windows.
- The source code, of course. How'd you think, you could build the source code, without the source code.

### Build
1. Clone the repository
2. Navigate to the GPT-UI directory (The one with a Cargo.toml file and a src/ directory)
3. Run the command "cargo r" \
Or "cargo r --release" to run it with optimizations.

You can find more cargo commands with the "cargo --help" command.

### Usage
First, you need a [H2OGPT](https://github.com/h2oai/h2ogpt) server to connect to.

Then open the settings and set you settings there. Put your H2OGPT server's ip and port in there.\
By default, the ip address is the loopback address (127.0.0.1).\
If the app cannot connect to a server when sending a message, it will tell you that it failed.

## Current state
There are known bugs in the GUI part, and I will fix them when I find time for it.

## Contribution
Contributions are welcome, please note that this repository uses the Unlicense.

## 3rd party library licenses
This project uses several third-party libraries. The licenses and copyright notices for these libraries can be found in the `third_party` directory.
