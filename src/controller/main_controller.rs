use irc::proto::Command;
use irc::client::ext::ClientExt;
use irc::client::prelude::{Config, Response, Future};
use irc::client::{IrcClient, Client};
use crate::controller::vote_controller::VoteController;
use std::collections::HashMap;
use crate::util::regex::{COMMAND, STARTVOTE, ENDVOTE, PING, VOTE, STARTGIVEAWAY, ENDGIVEAWAY, ENTERGIVEAWAY};
use crate::controller::giveaway_controller::GiveawayController;

use crate::controller::filter::Filter;
use crate::controller::database::DatabaseConnector;
use crate::util::config::{eval_config, get_db_config};
use futures::executor::{block_on, LocalPool};

/// Manages the calls to the other controllers
/// as well as communication between the program and Twitch
///
/// - **client** manages IRC communication
/// - **vote_controller** manages votes in the channel
/// - **giveaway_controller** manages giveaways in the channel
/// - **filter** detects swear words in messages
pub struct MainController {
    pub client: IrcClient,
    pub vote_controller: VoteController,
    pub giveaway_controller: GiveawayController,
    pub filter: Filter,
    pub db_connector: DatabaseConnector,
}


impl MainController {
    /// Instantiates a new MainController
    ///
    /// # Example
    /// ```
    /// let lang = get_lang();
    /// let client_config: Config = eval_config();
    /// let mut main_controller: MainController = MainController::new(lang, client_config);
    /// ```
    pub fn new(lang: String) -> MainController {
        let client_config: Config = eval_config();
        let client = IrcClient::from_config(client_config).unwrap();

        let vote_controller = VoteController::new();
        let filter = Filter::new(&lang);
        let giveaway_controller = GiveawayController::new();

        let db_config = get_db_config();
        let db_connector = block_on(DatabaseConnector::new(db_config));


        MainController {
            client,
            vote_controller,
            giveaway_controller,
            filter,
            db_connector,
        }
    }
    /// Listens for incoming messages and reacts to them if they match one of the defined patterns
    pub fn listen(&mut self) {
        let recv_client = self.client.clone();
        let send_client = self.client.clone();
        if let Err(e) = recv_client.identify() { println!("Error in auth") }
        recv_client.for_each_incoming(|irc_msg| {
            // println!("{:#?}", irc_msg);
            match &irc_msg.command {
                &Command::PRIVMSG(ref channel, ref message) => {
                    // print!("{:#?}\n", message);
                    block_on(self.db_connector.write_logs(message.clone().as_str()));
                    if self.filter.contains_insult(&message) {
                        send_client.send_privmsg(&channel, "That is not nice to say");
                    } else {
                        if PING.is_match(&message) {
                            println!("Sending ping as response");
                            send_client.send_privmsg(&channel, "!pong");
                        }
                        if VOTE.is_match(&message) {
                            println!("Adding vote");
                            let res = self.vote_controller.add(&message);
                            send_client.send_privmsg(&channel, res);
                        }
                        if STARTVOTE.is_match(&message) {
                            println!("Starting vote");
                            let res = self.vote_controller.start_vote(&message);
                            send_client.send_privmsg(&channel, res);
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
                            send_client.send_privmsg(&channel, user);
                        }
                        if ENTERGIVEAWAY.is_match(&message) {
                            println!("Entering giveaway!");
                            self.giveaway_controller.add_user(message);
                            send_client.send_privmsg(&channel, "Entered giveaway");

                        }
                    }
                }
                &Command::NOTICE(ref channel, ref msg) => {
                    println!("Got notice");
                    println!("{}{}", channel, msg)
                }
                &Command::Response(Response::RPL_NAMREPLY, ref args, ref suffix) => {
                    println!("Got response");
                    println!("{:?}{:?}", args, suffix)
                }

                _ => ()
            }
        }).unwrap()
    }
}
