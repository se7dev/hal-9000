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
/// Builds and returns **ClientOptions** based on config keys stored in .env
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


#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::options::{ClientOptions, StreamAddress, Credential};
    use std::borrow::Borrow;

    #[test]
    fn test_connection_initialized_config(){
        env::set_var("TWITCHPW", "test_pw");
        env::set_var("NICKNAME", "test_nick");
        env::set_var("SERVERNAME", "test_server");
        env::set_var("CHANNELS", "#test_channel");
        let cfg = eval_config();
        /*
        nickname: Some(nick.to_owned()),
        server: Some(server_name.to_owned()),
        port: Some(server_port.to_owned()),
        password: Some(oauth_token.to_owned()),
        channels: Some(channels),
        */

        assert_eq!(cfg.nickname, Some("test_nick".to_string()));
        assert_eq!(cfg.server, Some("test_server".to_string()));
        assert_eq!(cfg.password, Some("test_pw".to_string()));
        assert_eq!(cfg.channels, Some(vec!("#test_channel".to_string())));
    }
    #[test]
    fn test_initialized_db_config(){
        env::set_var("MONGO_INITDB_ROOT_USERNAME", "test_root_user");
        env::set_var("MONGO_INITDB_ROOT_PASSWORD", "test_root_pw");
        env::set_var("MONGO_INITDB_DATABASE", "test_initdb_db");
        env::set_var("MONGO_INITDB_HOSTNAME", "test_initdb_name");
        let db_conf  = get_db_config();
        assert_eq!(db_conf.credential.borrow().as_ref().unwrap().password, Some(String::from("test_root_pw")));
        assert_eq!(db_conf.credential.borrow().as_ref().unwrap().username, Some(String::from("test_root_user")));
        assert_eq!(db_conf.credential.borrow().as_ref().unwrap().source, Some(String::from("test_initdb_db")));
        assert_eq!(db_conf.hosts[0].hostname, "test_initdb_name");
    }
    #[test]
    fn test_language(){
        env::set_var("LANG", "English");
        let language = get_lang();
        assert_eq!(language,"English");

    }
}