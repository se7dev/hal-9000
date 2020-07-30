use std::collections::{VecDeque, HashMap};
use std::iter::FromIterator;
use crate::controller::main_controller::MainController;
use irc::client::Client;

#[derive(Clone)]
pub struct Votes {
    pub vote_options: HashMap<String, i32>,
}

#[derive(Clone)]
pub struct VoteController {
    pub votes: Option<Votes>,
}

pub fn get_vote_item(msg: &'static String) -> Vec<&'static str> {
    let mut split = msg.split(" ");
    return split.collect::<Vec<&str>>();
}

impl VoteController {
    pub fn add(&self, vote: &'static String) {
        match &self.votes {
            Some(votes) => {
                let eval_vote = get_vote_item(&vote).first().unwrap().to_owned().to_owned();
                if self.check_if_valid(&eval_vote) {
                    let (_, val) = votes.vote_options.get_key_value(&eval_vote).unwrap();
                    let mut tmp = votes.vote_options.clone();
                    tmp.insert(eval_vote, val + 1);
                }
            }
            None => { println!("Vote not active") }
        }
    }
    // pub fn remove(&mut self, vote: String) {
    //     match &self.votes {
    //         Some(mut votes) => { votes.vote_options.retain(|v| &vote != v) }
    //         None => { println!("Vote not active") }
    //     }
    // }
    pub fn start_vote(&mut self, mut vote_options: HashMap<String, i32>) {
        self.votes = Option::from(Votes {
            vote_options,
        })
    }
    pub fn close_and_eval(&self) -> String {
        return match &self.votes {
            Some(votes) => {
                let mut result: Vec<(&String, &i32)> = Vec::from_iter(&votes.vote_options);
                result.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
                result_string_builder(result)
            }
            None => { "Vote not active".to_string() }
        };
    }
    pub fn check_if_valid(&self, key: &String) -> bool {
        return match &self.votes {
            Some(votes) => {
                let test: &str = &key;
                if (votes.vote_options.contains_key(test)) {
                    return true;
                }
                return false;
            }
            None => { false }
        };
    }
}

fn result_string_builder(result: Vec<(&String, &i32)>) -> String {
    let mut string = Vec::<String>::new();
    string.push("Vote ist closed the Results are: \n".to_string());
    for elem in result {
        string.push(format!("Option:{} has {} votes \n", elem.0, elem.1));
    }
    return string.join("");
}