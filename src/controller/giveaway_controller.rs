use std::collections::HashSet;
use rand::{thread_rng, Rng};
use regex::Regex;


pub struct GiveawayController {
    users: HashSet<String>,
    giveaway_running: bool,
}

impl GiveawayController {
    pub fn new() -> GiveawayController {
        GiveawayController {
            users: HashSet::new(),
            giveaway_running: false,
        }
    }
    pub fn start_giveaway(&mut self) {
        self.giveaway_running = true;
    }
    pub fn stop_giveaway(&mut self) {
        self.giveaway_running = false;
    }
    pub fn add_user(&mut self, user: String) {
        if self.giveaway_running {
            let re = Regex::new(r"#\w*").unwrap();
            for matched_expression in re.captures_iter(user.as_str()) {
                let user = &matched_expression[0].replace("#", "");
                let _boo = self.users.insert(String::from(user));
            }
        }
    }
    pub fn choose_user(&self) -> String {
        if self.giveaway_running == true {
            let mut rng = thread_rng();
            return if self.users.len() > 0 {
                let index = rng.gen_range(0, self.users.len());
                let chosen = self.users.iter().nth(index).unwrap().clone();
                chosen
            } else {
                String::from("Nobody entered the giveaway")
            };
        } else {
            return String::from("No giveaway running at the moment. Check back later.");
        };
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Users should add user when !entry command is called in chat
    fn users_from_chat_in_users() {
        let mut users = GiveawayController { users: HashSet::new(), giveaway_running: true };
        users.add_user(String::from(
            ":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rowe90 :test"));
        assert!(users.users.contains("rowe90"));
    }

    #[test]
    // Users should choose a random User from list of Users
    fn choose_random_user() {
        let mut users = GiveawayController { users: HashSet::new(), giveaway_running: true };
        users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rowe90 :test"));
        users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #sedev :test"));
        users.add_user(String::from(":rowe90!rowe90@rowe90.tmi.twitch.tv PRIVMSG #rall0 :test"));
        let _chosen = users.choose_user();
    }

    #[test]
    // Users should choose a random User from list of Users
    fn no_user_for_giveaway() {
        let users = GiveawayController { users: HashSet::new(), giveaway_running: true };
        let chosen = users.choose_user();
        assert_eq!(chosen, "Nobody entered the giveaway");
    }
}