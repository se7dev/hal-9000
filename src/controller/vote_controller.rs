use std::collections::{VecDeque, HashMap};
use std::iter::FromIterator;
use crate::controller::main_controller::MainController;
use irc::client::Client;

#[derive(Clone, Debug)]
pub struct VoteController {
    pub votes: HashMap<String, i32>
}

pub fn get_vote_item(msg: &str) -> Vec<&str> {
    let mut split = msg.split(" ");
    let options = split.collect::<Vec<&str>>();
    println!("{:?}", options);
    return options[1..].to_vec();
}

impl VoteController {
    pub fn new() -> VoteController {
        VoteController {
            votes: HashMap::new()
        }
    }

    pub fn add(&mut self, vote_msg: &str) {
        if self.votes.len() > 0 {
            let eval_vote: String = get_vote_item(&vote_msg).first().unwrap().to_owned().to_owned();
            println!("{}", eval_vote);
            if self.check_if_valid(&eval_vote) {
                println!("passed");
                let (_, val) = self.votes.get_key_value(&eval_vote).unwrap();
                self.votes.insert(eval_vote, val + 1);
            } else {
                println!("No a valid entry")
            }
        } else {
            println!("Vote not active");
        }
    }


    pub fn start_vote(&mut self, msg: &str) {
        let vote_items = get_vote_item(msg);
        let options: HashMap<String, i32> = vote_items
            .into_iter()
            .fold(HashMap::new(), |mut acc, test| {
                acc.insert(test.to_owned(), 0);
                acc
            });
        self.votes = options
    }

    pub fn close_and_eval(&mut self) -> String {
        let test = self.votes.clone();
        let mut result: Vec<(String, i32)> = Vec::from_iter(test.into_iter());
        result.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        println!("{:?}", result);
        self.votes = HashMap::new();
        result_string_builder(result)
    }


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