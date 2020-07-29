#![allow(warnings)]

mod controller;
mod util;

extern crate log;
extern crate simple_logger;

use controller::client_config::eval_config;
use irc::client::prelude::Config;
use controller::main_controller::MainController;

fn main() {
    let client_config: Config = eval_config();
    let main_controller: MainController = MainController::new(client_config);
    main_controller.start()
}
