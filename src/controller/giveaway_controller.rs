use std::collections::HashSet;
use rand::{thread_rng, Rng};
use regex::Regex;

struct Users {
    users: HashSet<String>
}

impl Users {
    fn add_user(&mut self, user: String) {
        let re = Regex::new(r"#\w*").unwrap();
        for matched_expression in re.captures_iter(user.as_str()) {
            let user = &matched_expression[0].replace("#", "");
            let _boo = self.users.insert(String::from(user));
        }
    }
    fn choose_user(&self) -> String {
        let mut rng = thread_rng();
        return if self.users.len() > 0 {
            let index = rng.gen_range(0, self.users.len());
            let chosen = self.users.iter().nth(index).unwrap().clone();
            chosen
        } else {
            String::from("Nobody entered the giveaway")
        }
    }
}

fn main() {
    let mut users = Users { users: HashSet::new() };
    users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rowe90 :test"));
    users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #sedev :test"));
    users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rall0 :test"));
    let chosen = users.choose_user();
    println!("{}", chosen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Users should add user when !entry command is called in chat
    fn users_from_chat_in_users() {
        let mut users = Users { users: HashSet::new() };
        users.add_user(String::from(
            ":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rowe90 :test"));
        assert!(users.users.contains("rowe90"));
    }

    #[test]
    // Users should choose a random User from list of Users
    fn choose_random_user() {
        let mut users = Users { users: HashSet::new() };
        users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rowe90 :test"));
        users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #sedev :test"));
        users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rall0 :test"));
        let _chosen = users.choose_user();
    }
    #[test]
    // Users should choose a random User from list of Users
    fn no_user_for_giveaway() {
        let mut users = Users { users: HashSet::new() };
        let chosen = users.choose_user();
        assert!(chosen,"Nobody entered the giveaway");
    }
}