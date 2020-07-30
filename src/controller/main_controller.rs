use irc::proto::Command;
use irc::client::ext::ClientExt;
use irc::client::prelude::Config;
use irc::client::{IrcClient, Client};
use crate::controller::vote_controller::VoteController;
use std::collections::HashMap;
use crate::util::regex_commands::{COMMAND, STARTVOTE, ENDVOTE, PING, VOTE, STARTGIVEAWAY, ENDGIVEAWAY, ENTERGIVEAWAY};
use crate::controller::giveaway_controller::GiveawayController;

static POOL_SIZE: usize = 8;

pub struct MainController {
    pub client: IrcClient,
    pub vote_controller: VoteController,
    pub giveaway_controller: GiveawayController,
}

impl MainController {
    pub fn new(config: Config) -> MainController {
        let client = IrcClient::from_config(config).unwrap();
        let vote_controller = VoteController::new();
        let giveaway_controller = GiveawayController::new();
        MainController {
            client,
            vote_controller,
            giveaway_controller
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
                if STARTGIVEAWAY.is_match(&message) {
                    println!("Starting giveaway!");
                    self.giveaway_controller.start_giveaway();
                    send_client.send_privmsg(&channel, "Giveaway started");
                }
                if ENDGIVEAWAY.is_match(&message) {
                    println!("Ending giveaway!");
                    let user = self.giveaway_controller.choose_user();
                    self.giveaway_controller.stop_giveaway();
                    send_client.send_privmsg(&channel, "Giveaway has ended");
                }
                if ENTERGIVEAWAY.is_match(&message) {
                    println!("Entering giveaway!");
                    self.giveaway_controller.add_user(message);
                    send_client.send_privmsg(&channel, "Entered giveaway");
                }
            }
        }).unwrap()
    }
}
