//! # hal9000
//! hal9000 is a chatbot for the streaming platform **Twitch.tv**
//! # Features
//! - send and receive messages from a chat on twitch
//! - react to certain chat commands, e.g. !startgiveaway lets you organize a giveaway for chat
//! participants
//! - logging of chat messages in the background

#![allow(warnings)]

mod controller;
mod util;

extern crate log;
extern crate simple_logger;
extern crate mongodb;
extern crate tokio;

use controller::main_controller::MainController;
use irc::client::prelude::Config;
use util::config::eval_config;
use util::config::get_lang;

#[tokio::main]
async fn main() {
    /// Main does 3 things:
    /// - get language for filter that is defined in .env
    /// - instantiate a MainController
    /// - start listening for incoming chat messages
    let lang = get_lang();
    let mut main_controller: MainController = MainController::new(lang);
    main_controller.listen()
}
