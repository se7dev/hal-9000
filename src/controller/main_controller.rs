use irc::proto::Command;
use irc::client::ext::ClientExt;
use irc::client::prelude::Config;
use irc::client::{IrcClient, Client};
use crate::controller::vote_controller::VoteController;
use std::collections::HashMap;
use crate::util::regex::{COMMAND, STARTVOTE, ENDVOTE, PING, VOTE};
use crate::controller::filter::Filter;

static POOL_SIZE: usize = 8;

pub struct MainController {
    pub client: IrcClient,
    pub vote_controller: VoteController,
    pub filter: Filter,
}

impl MainController {
    pub fn new(lang: String, config: Config) -> MainController {
        let client = IrcClient::from_config(config).unwrap();
        let vote_controller = VoteController::new();
        let filter = Filter::new(&lang);
        MainController {
            client,
            vote_controller,
            filter,
        }
    }
    pub fn listen(&mut self) {
        let recv_client = self.client.clone();
        let send_client = self.client.clone();

        if let Err(e) = recv_client.identify() { println!("Error in auth") }
        recv_client.for_each_incoming(|irc_msg| {
            print!("{:#?}\n", irc_msg.command);
            if let Command::PRIVMSG(channel, message) = irc_msg.command {
                print!("{:#?}\n", message);
                if self.filter.contains_insult(&message) {
                    send_client.send_privmsg(&channel, "That is not nice to say");
                } else {
                    if PING.is_match(&message) {
                        println!("Sending ping as response");
                        if let Err(e) = send_client.send_privmsg(&channel, "!pong") {
                            println!("Cant send message")
                        }
                    }
                    if VOTE.is_match(&message) {
                        println!("Adding vote");
                        self.vote_controller.add(&message);
                        println!("{:?}", self.vote_controller);
                        send_client.send_privmsg(&channel, "Vote started");
                    }
                    if STARTVOTE.is_match(&message) {
                        println!("Starting vote");
                        self.vote_controller.start_vote(&message);
                        println!("{:?}", self.vote_controller);
                        send_client.send_privmsg(&channel, "Vote started");
                    }
                    if ENDVOTE.is_match(&message) {
                        println!("Ending vote!");
                        let result = self.vote_controller.close_and_eval();
                        send_client.send_privmsg(&channel, result);
                    }
                }
            }
        }).unwrap()
    }
}
