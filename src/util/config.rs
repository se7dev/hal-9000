use irc::client::prelude::Config;
use std::env;

pub fn eval_config() -> Config {
    let oauth_token: String = env::var("TWITCHPW").unwrap();
    let nick: String = env::var("NICKNAME").unwrap();
    let server_name: String = env::var("SERVERNAME").unwrap();
    let server_port = 6667;
    let channels = env::var("CHANNELS")
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let cfg = Config {
        nickname: Some(nick.to_owned()),
        server: Some(server_name.to_owned()),
        port: Some(server_port.to_owned()),
        password: Some(oauth_token.to_owned()),
        channels: Some(channels),
        ..Default::default()
    };
    return cfg;
}

pub fn get_lang() -> String {
    return env::var("LANG").unwrap();
}
