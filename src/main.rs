//! # hal9000
//! hal9000 is a chatbot for the streaming platform **Twitch.tv**
//! # Features
//! - send and receive messages from a chat on twitch
//! - react to certain chat commands, e.g. !startgiveaway lets you organize a giveaway for chat
//! participants
//! - logging of chat messages in the background

#[macro_use]
extern crate log;
extern crate mongodb;
extern crate simple_logger;
extern crate tokio;

use controller::bot::MainController;
use util::config::get_lang;
use util::config::get_logger_level;

mod controller;
mod util;

#[tokio::main]
/// Main does 3 things:
/// get language for filter that is defined in .env,
/// instantiate a MainController,
/// start listening for incoming chat messages,
async fn main() {
    simple_logger::init_with_level(get_logger_level()).unwrap();
    debug!("Program starting");
    let lang = get_lang();
    debug!("Lang is {}", lang);
    let mut main_controller: MainController = MainController::new(lang);
    trace!("Controller {:#?}", main_controller);
    main_controller.listen()
}
