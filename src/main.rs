extern crate log;
extern crate simple_logger;
use irc::client::prelude::*;
use clientConfig;

fn main() {
    let client_config = eval

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
    }).unwrap(); */
}
