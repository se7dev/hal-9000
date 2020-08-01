use std::collections::HashSet;

use rand::{thread_rng, Rng};

#[derive(Debug)]
/// # GiveawayController
/// Serves as the controller to organize giveaways in channels
/// Holds **users** as a HashSet and **giveaway_running** to check if giveaway has been started.
/// # Example
/// ```
///fn choose_random_user() {
///         let mut users = GiveawayController { users: HashSet::new(), giveaway_running: true };
///         users.add_user(String::from("!giveawayenter basti"));
///         users.add_user(String::from("!giveawayenter robin"));
///         users.add_user(String::from("!giveawayenter ralph"));
///         let chosen = users.choose_user();
///         assert!(!chosen.is_empty());
/// ```
pub struct GiveawayController {
    users: HashSet<String>,
    giveaway_running: bool,
}

impl GiveawayController {
    /// Instantiates a new GiveawayController
    /// # Example
    /// ```
    ///  let mut users = GiveawayController { users: HashSet::new(), giveaway_running: false };
    /// ```
    pub fn new() -> GiveawayController {
        info!("Init Giveaway controller");
        GiveawayController {
            users: HashSet::new(),
            giveaway_running: false,
        }
    }
    /// Enables adding participants and choosing a winner
    /// # Example
    /// ```
    /// let mut users = GiveawayController { users: HashSet::new(), giveaway_running: false };
    /// users.add_user("!giveawayenter user");
    /// assert_eq!(users.users.is_empty(), true);
    /// users.start_giveaway();
    /// users.add_user("!giveawayenter user");
    /// assert_eq!(users.users.is_empty(), true);
    /// ```
    pub fn start_giveaway(&mut self) {
        self.giveaway_running = true;
    }
    /// Disables adding participants and choosing a winner and clears the users HashSet.
    /// # Example
    /// ```
    /// let mut users = GiveawayController { users: HashSet::new(), giveaway_running: true };
    /// users.add_user("!giveawayenter user");
    /// assert_eq!(users.users.is_empty(), false);
    /// users.stop_giveaway();
    /// assert_eq!(users.users.is_empty(), true);
    /// users.add_user("!giveawayenter user");
    /// assert_eq!(users.users.is_empty(), true);
    /// ```
    pub fn stop_giveaway(&mut self) {
        self.giveaway_running = false;
        self.users.clear();
    }
    /// Adds users to the price pool.
    /// # Example
    /// ```
    /// let mut users = GiveawayController { users: HashSet::new(), giveaway_running: true };
    /// users.add_user("!giveawayenter user");
    /// assert!(users.users.contains("user"));
    /// ```
    pub fn add_user(&mut self, user: &String) {
        if self.giveaway_running {
            let enter = user.split_whitespace();
            let usr = enter.collect::<Vec<_>>();
            let _boo = self.users.insert(String::from(usr[1]));
        }
    }
    /// Chooses a user to win the price when a giveaway is running.
    /// # Example
    /// ```
    /// let mut users = GiveawayController { users: HashSet::new(), giveaway_running: true };
    /// users.add_user(String::from("!giveawayenter basti"));
    /// users.add_user(String::from("!giveawayenter robin"));
    /// users.add_user(String::from("!giveawayenter ralph"));
    /// let chosen = users.choose_user();
    /// assert!(!chosen.is_empty());
    /// ```
    pub fn choose_user(&self) -> String {
        if self.giveaway_running == true {
            let mut rng = thread_rng();
            return if self.users.len() > 0 {
                let index = rng.gen_range(0, self.users.len());
                let mut chosen = self.users.iter().nth(index).unwrap().clone();
                chosen.push_str(" has won the giveaway. Well done!");
                chosen
            } else {
                String::from("Nobody entered the giveaway")
            };
        } else {
            return String::from("No giveaway running at the moment. Check back later.");
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Users should add user when !entry command is called in chat
    fn users_from_chat_in_users() {
        let mut users = GiveawayController {
            users: HashSet::new(),
            giveaway_running: true,
        };
        users.add_user(&String::from("!giveawayenter rowe90"));
        assert!(users.users.contains("rowe90"));
    }

    #[test]
    // Users should choose a random User from list of Users
    fn choose_random_user() {
        let mut users = GiveawayController {
            users: HashSet::new(),
            giveaway_running: true,
        };
        users.add_user(&String::from("!giveawayenter basti"));
        users.add_user(&String::from("!giveawayenter robin"));
        users.add_user(&String::from("!giveawayenter ralph"));
        let chosen = users.choose_user();
        assert!(!chosen.is_empty());
    }

    #[test]
    // Users should choose a random User from list of Users
    fn no_user_for_giveaway() {
        let users = GiveawayController {
            users: HashSet::new(),
            giveaway_running: true,
        };
        let chosen = users.choose_user();
        assert_eq!(chosen, "Nobody entered the giveaway");
    }

    #[test]
    // Users should choose a random User from list of Users
    fn dont_add_users_when_no_giveaway_running() {
        let mut users = GiveawayController {
            users: HashSet::new(),
            giveaway_running: false,
        };
        users.add_user(&String::from("!giveawayenter basti"));
        let chosen = users.choose_user();
        assert_eq!(
            chosen,
            "No giveaway running at the moment. Check back later."
        );
    }
}
