use irc::proto::Command;
use irc::client::ext::ClientExt;
use irc::client::prelude::Config;
use irc::client::{IrcClient, Client};
use crate::controller::vote_controller::VoteController;
use std::collections::HashMap;
use crate::util::regex_commands::{COMMAND, STARTVOTE, ENDVOTE, PING};

static POOL_SIZE: usize = 8;

pub struct MainController {
    pub client: IrcClient,
    pub vote_controller: VoteController,
}

impl MainController {
    pub fn new(config: Config) -> MainController {
        let client = IrcClient::from_config(config).unwrap();
        let vote_controller = VoteController { votes: None };
        MainController {
            client,
            vote_controller,
        }
    }
    pub fn init(&self) {}

    pub fn listen(&mut self) {
        let recv_client = self.client.clone();
        let send_client = self.client.clone();

        if let Err(e) = recv_client.identify() { println!("Error in auth") }
        recv_client.for_each_incoming(|irc_msg| {
            print!("{:#?}\n", irc_msg.command);
            if let Command::PRIVMSG(channel, message) = irc_msg.command {
                print!("{:#?}\n", message);
                if PING.is_match(&message) {
                    println!("Sending ping as response");
                    if let Err(e) = send_client.send_privmsg(&channel, "!pong") {
                        println!("Cant send message")
                    }
                }
                if STARTVOTE.is_match(&message) {
                    println!("Starting vote");
                    let options = HashMap::new();
                    self.vote_controller.start_vote(options);
                    send_client.send_privmsg(&channel, "Vote started");
                }
                if ENDVOTE.is_match(&message) {
                    println!("Ending vote!");
                    let result = self.vote_controller.close_and_eval();
                    send_client.send_privmsg(&channel, result);
                }
            }
        }).unwrap()
    }
}
