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
    let lang = get_lang();
    let mut main_controller: MainController = MainController::new(lang);
    main_controller.listen()
}
