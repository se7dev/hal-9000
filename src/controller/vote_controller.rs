use std::collections::{VecDeque, HashMap};
use std::iter::FromIterator;
use crate::controller::main_controller::MainController;
use irc::client::Client;
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
        VoteController {
            votes: HashMap::new(),
        }
    }

    pub fn add(&mut self, vote_msg: &str) -> &str {
    /// Yet to be documented
        if self.votes.len() > 0 {
            let eval_vote: String = get_cmd_elem(&vote_msg).first().unwrap().to_owned().to_owned();
            println!("{}", eval_vote);
            if self.check_if_valid(&eval_vote) {
                println!("passed");
                let (_, val) = self.votes.get_key_value(&eval_vote).unwrap();
                self.votes.insert(eval_vote, val + 1);
                return "Entry added";
            } else {
                return "No a valid entry";
            }
        } else {
            return "Vote not active";
        }
    }

    /// Yet to be documented
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

    /// Yet to be documented
    pub fn close_and_eval(&mut self) -> String {
        let test = self.votes.clone();
        let mut result: Vec<(String, i32)> = Vec::from_iter(test.into_iter());
        result.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        println!("{:?}", result);
        self.votes = HashMap::new();
        result_string_builder(result)
    }

    /// Yet to be documented
    pub fn check_if_valid(&self, key: &str) -> bool {
        if self.votes.contains_key(key) {
            return true;
        }
        return false;
    }
}

fn result_string_builder(result: Vec<(String, i32)>) -> String {
    let mut string = Vec::<String>::new();
    string.push("Vote ist closed.".to_string());
    for (key, val) in result {
        string.push(format!("|{} has {} votes|", key, val));
    }
    return string.join("");
}
