#![allow(warnings)]

mod controller;
mod util;

extern crate log;
extern crate simple_logger;

use controller::main_controller::MainController;
use irc::client::prelude::Config;
use util::config::eval_config;
use util::config::get_lang;

fn main() {
    let lang = get_lang();
    let client_config: Config = eval_config();
    let mut main_controller: MainController = MainController::new(lang, client_config);
    main_controller.listen()
}
