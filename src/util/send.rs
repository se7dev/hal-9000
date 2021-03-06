use irc::client::prelude::ClientExt;
use irc::client::IrcClient;

pub fn send(client: &IrcClient, channel: &String, message: &str) {
    if let Err(_e) = client.send_privmsg(channel, message) {
        error!("Could not send message {} to {}", message, channel)
    }
}
