use irc::client::prelude::Config;
use std::env;
use mongodb::options::{ClientOptions, StreamAddress, Credential};


/// Builds a **Config** from given environment variables saved in .env
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

pub fn get_db_config() -> ClientOptions {
    // ("mongodb://root:root@database:27017/")
    let username: Option<String> = Option::from(env::var("MONGO_INITDB_ROOT_USERNAME").unwrap());
    let password: Option<String> = Option::from(env::var("MONGO_INITDB_ROOT_PASSWORD").unwrap());
    let source: Option<String> = Option::from(env::var("MONGO_INITDB_DATABASE").unwrap());
    let credential = Option::from(Credential {
        username,
        source,
        password,
        ..Default::default()
    });

    let hostname: String = env::var("MONGO_INITDB_HOSTNAME").unwrap();
    let port: Option<u16> = Option::from(27017 as u16);
    let hosts = vec![StreamAddress {
        hostname,
        port,
    }];
    let mut client_options = ClientOptions::default();
    client_options.hosts = hosts;
    client_options.app_name = Some("HAL9000".to_string());
    client_options.credential = credential;
    client_options.direct_connection = Some(true);
    return client_options;
}
/// Gets the language defined for Filter from LANG saved in .env
pub fn get_lang() -> String {
    return env::var("LANG").unwrap();
}
