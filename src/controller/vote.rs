use std::collections::HashMap;
use std::iter::FromIterator;

use crate::util::get_item::get_cmd_elem;

/// Manages voting on a channel
///
/// Enables voting on starting, ending and voting on multiple items that are chosen by the voting
/// initiater.
#[derive(Clone, Debug)]
pub struct VoteController {
    pub votes: HashMap<String, i32>,
}

impl VoteController {
    /// Instantiates a new VoteController with an empty HashMap
    pub fn new() -> VoteController {
        info!("Initiating vote controller");
        VoteController {
            votes: HashMap::new(),
        }
    }
    /// Adds a vote to the current motion.
    ///
    /// It checks if the string contains a vote.
    /// If it is valid, only then is it added to the votes.
    pub fn add(&mut self, vote_msg: &str) -> &str {
        let eval_vote: String;
        return if self.votes.len() > 0 {
            eval_vote = get_cmd_elem(&vote_msg).first().unwrap().to_owned().to_owned();
            debug!("Vote elements are {}", eval_vote);
            if self.check_if_valid(&eval_vote) {
                // Warning https://github.com/rust-lang/rust/issues/59159
                let shared = &self.votes;
                let val = shared.get_key_value(&eval_vote).unwrap().1 + 1;
                self.votes.insert(eval_vote, val);
                // End of warning
                "Entry added"
            } else {
                "Not a valid entry"
            }
        } else {
            "Vote not active"
        };
    }
    /// Starts a motion
    ///
    /// It looks for voting items in the passed message, which are then used as voting options
    pub fn start_vote(&mut self, msg: &str) -> &str {
        let vote_items = get_cmd_elem(msg);
        let options: HashMap<String, i32> =
            vote_items
                .into_iter()
                .fold(HashMap::new(), |mut acc, test| {
                    acc.insert(test.to_owned(), 0);
                    acc
                });
        self.votes = options;
        return "Vote started";
    }

    /// Closes a motion and returns the end result as a String.
    pub fn close_and_eval(&mut self) -> String {
        let test = self.votes.clone();
        let mut result: Vec<(String, i32)> = Vec::from_iter(test.into_iter());
        result.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        debug!("Results are {:?}", result);
        self.votes = HashMap::new();
        result_string_builder(result)
    }

    /// Checks if vote is valid by comparing passed key to elements in votes.
    pub fn check_if_valid(&self, key: &str) -> bool {
        if self.votes.contains_key(key) {
            debug!("Vote is valid");
            return true;
        }
        debug!("Vote is not valid");
        return false;
    }
}

/// Used to build a nicely readable string containing the end result of a vote
fn result_string_builder(result: Vec<(String, i32)>) -> String {
    let mut string = Vec::<String>::new();
    string.push("Vote ist closed.".to_string());
    for (key, val) in result {
        string.push(format!("|{} has {} votes|", key, val));
    }
    return string.join("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vote_controller_instantiation() {
        let vote_controller = VoteController::new();
        assert_eq!(vote_controller.votes.is_empty(), true);
    }

    #[test]
    fn test_adding_votes() {
        let mut vote_controller = VoteController::new();
        vote_controller.start_vote("!votestart test1 test2");
        vote_controller.add("!vote test1");
        assert_eq!(vote_controller.votes.is_empty(), false);
    }

    #[test]
    fn test_votes_evaluation() {
        let mut vote_controller = VoteController::new();
        vote_controller.start_vote("!votestart test1 test2");
        vote_controller.add("!vote test1");
        vote_controller.add("!vote test1");
        vote_controller.add("!vote test2");
        let result = vote_controller.close_and_eval();
        assert_eq!(result, "Vote ist closed.|test1 has 2 votes||test2 has 1 votes|");
    }

    #[test]
    fn test_vote_validation() {
        let mut vote_controller = VoteController::new();
        vote_controller.start_vote("!votestart test1 test2");
        let res = vote_controller.check_if_valid("test1");
        assert_eq!(res, true);
        let res = vote_controller.check_if_valid("test5");
        assert_eq!(res, false);
        let res = vote_controller.check_if_valid("");
        assert_eq!(res, false);
    }
}