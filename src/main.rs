extern crate log;
extern crate simple_logger;
extern crate irc;

use irc::client::prelude::*;
use std::env;

fn main() {
    let oauth_token = env::var("TWITCH_PW").unwrap();
    let cfg = Config {
        nickname: Some("HAL0900".to_owned()), //use the name of the account for the twitch bot
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6667),
        password: Some(oauth_token.to_owned()), // use the oauth token from the twitch bot account
        channels: Some(vec!["#rowe90".to_owned()]),
        ..Default::default()
    };

    let client = IrcClient::from_config(cfg).unwrap();

    client.identify().unwrap();

    client.for_each_incoming(|irc_msg| {
        print!("{}", irc_msg);
        if let Command::PRIVMSG(channel, message) = irc_msg.command {
            if message.contains(client.current_nickname()) {
                client.send_privmsg(&channel, "beep boop").unwrap();
            } else if message.contains("!help") {
                client.send_privmsg(&channel, "I am afraid i can't do that Jon.").unwrap();
            }
        }
    }).unwrap();
}
