use irc::proto::Command;
use tokio::runtime::Runtime;
use irc::client::ext::ClientExt;
use irc::client::prelude::Config;
use irc::client::{IrcClient, Client};
use crate::controller::vote_controller::VoteController;
use std::collections::HashMap;
use crate::util::regex_commands::{COMMAND, STARTVOTE};

static POOL_SIZE: usize = 8;

pub struct MainController {
    pub client: IrcClient,
    pub vote_controller: VoteController,
}

impl MainController {
    pub fn new(config: Config) -> MainController {
        let client = IrcClient::from_config(config).unwrap();
        let vote_controller = VoteController { vote: None };
        MainController {
            client,
            vote_controller,
        }
    }
    pub fn init(&self) {}

    pub fn listen(&self) {
        self.client.identify().unwrap();
        self.client.for_each_incoming(|irc_msg| {
            print!("{}", irc_msg);

            if let Command::PRIVMSG(channel, message) = irc_msg.command {
                match message {
                    irc_msg if COMMAND.is_match(&message) => {
                        instructions.append(&mut filter_words)
                    }
                    message if STARTVOTE.is_match(&message) => (),
                    message if STARTVOTE.is_match(&message) => (),
                    message if STARTVOTE.is_match(&message) => (),
                    message if STARTVOTE.is_match(&message) => (),
                }

                if message.contains("!ping") {
                    println!("pong");
                    self.client.send_privmsg(&channel, "pong").unwrap()
                }
            }

            /*
            PoisonPill received
             println!("Result: {:?}", rx.wait());
             pool.shutdown().wait().unwrap();
            */
        }).unwrap();
    }
}
