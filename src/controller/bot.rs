use futures::executor::block_on;
use irc::client::ext::ClientExt;
use irc::client::prelude::{Config, Response};
use irc::client::{Client, IrcClient};
use irc::proto::Command;

use crate::controller::database::DatabaseConnector;
use crate::controller::filter::Filter;
use crate::controller::giveaway::GiveawayController;
use crate::controller::vote::VoteController;
use crate::util::config::{eval_config, get_db_config};
use crate::util::regex::{
    ENDGIVEAWAY, ENDVOTE, ENTERGIVEAWAY, PING, STARTGIVEAWAY, STARTVOTE, VOTE,
};
use crate::util::send::send;

#[derive(Debug)]
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
        info!("Initiating main controller");
        let client_config: Config = eval_config();
        debug!("Client config {:#?}", client_config);
        let client = IrcClient::from_config(client_config).unwrap();
        trace!("IRC CLient {:#?}", client);

        let vote_controller = VoteController::new();
        trace!("Vote Controller {:#?}", vote_controller);

        let filter = Filter::new(&lang);
        trace!("Filter {:#?}", filter);

        let giveaway_controller = GiveawayController::new();
        trace!("Giveaway controller {:#?}", giveaway_controller);

        let db_config = get_db_config();
        debug!("DB config {:#?}", db_config);

        let db_connector = block_on(DatabaseConnector::new(db_config));
        trace!("DB Connector {:#?}", db_connector);

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
        info!("Start twitch bot");
        let recv_client = self.client.clone();
        let send_client = self.client.clone();
        if let Err(e) = recv_client.identify() {
            panic!("Error in auth {:?}", e)
        }

        info!("Listening for chat messages");
        recv_client
            .for_each_incoming(|irc_msg| {
                trace!("Incoming message {:#?}", &irc_msg);
                match &irc_msg.command {
                    &Command::PRIVMSG(ref channel, ref message) => {
                        debug!("Received message: {}", message);
                        if let Err(e) =
                            block_on(self.db_connector.write_logs(message.clone().as_str()))
                        {
                            error!("Message could not be saved {:#?}", e)
                        }
                        if self.filter.contains_insult(&message) {
                            send(&send_client, &channel, "That is not nice to say");
                        } else {
                            if PING.is_match(&message) {
                                debug!("Sending ping as response");
                                send(&send_client, &channel, &"!pong".to_string());
                            }
                            if VOTE.is_match(&message) {
                                debug!("Adding vote");
                                let res = self.vote_controller.add(&message);
                                send(&send_client, &channel, &res.to_string());
                            }
                            if STARTVOTE.is_match(&message) {
                                info!("Starting vote");
                                let res = self.vote_controller.start_vote(&message);
                                send(&send_client, &channel, &res.to_string());
                            }
                            if ENDVOTE.is_match(&message) {
                                info!("Ending vote!");
                                let result = self.vote_controller.close_and_eval();
                                send(&send_client, &channel, &result.as_str())
                            }
                            if STARTGIVEAWAY.is_match(&message) {
                                info!("Starting giveaway!");
                                self.giveaway_controller.start_giveaway();
                                send(&send_client, &channel, &"Giveaway started");
                            }
                            if ENDGIVEAWAY.is_match(&message) {
                                info!("Ending giveaway!");
                                let user = self.giveaway_controller.choose_user();
                                self.giveaway_controller.stop_giveaway();
                                send(&send_client, &channel, &user);
                            }
                            if ENTERGIVEAWAY.is_match(&message) {
                                info!("User entered giveaway!");
                                self.giveaway_controller.add_user(message);
                                send(&send_client, &channel, &"Entered giveaway");
                            }
                        }
                    }
                    &Command::NOTICE(ref channel, ref msg) => {
                        debug!("Got notice {:?}, {:?}", channel, msg)
                    }
                    &Command::Response(Response::RPL_NAMREPLY, ref args, ref suffix) => {
                        debug!("Got response {:?}{:?}", args, suffix)
                    }

                    _ => (),
                }
            })
            .unwrap()
    }
}
