pub mod clientConfig {
    use irc::client::prelude::Config;
    use std::env;

    pub fn evalConfig() -> Result<Config> {
        let oauth_token = env::var("TWITCH_PW").unwrap()?;
        let nick = env::var("NICKNAME").unwrap()?;
        let server_name = env::var("SERVERNAME").unwrap()?;
        let server_port = env::var("SERVERPORT").unwrap()?;
        let channels = env::var("CHANNELS").unwrap().split_whitespace().collect()?;

        let cfg = Config {
            nickname: Some(nick.to_owned()),
            server: Some(server_name.to_owned()),
            port: Some(server_port.to_owned()),
            password: Some(oauth_token.to_owned()),
            channels: Some(channels.to_owned()),
            ..Default::default()
        };
    }
}
