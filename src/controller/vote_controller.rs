use std::collections::{VecDeque, HashMap};
use std::iter::FromIterator;
use crate::controller::main_controller::MainController;
use irc::client::Client;

#[derive(PartialEq)]
pub struct UserVote {
    id: String,
    nickname: String,
    result: String,
}

struct Vote {
    vote_pool: VecDeque<UserVote>,
    vote_options: HashMap<String, i32>,
}

pub struct VoteController {
    pub vote: Option<Vote>,
}


impl VoteController {
    pub fn add(&mut self, vote: UserVote) {
        match self.vote {
            Some(_) => {
                self.vote_pool.push_back(vote)
            }
            None => {
                println!("Vote not active")
            }
        }
    }
    pub fn remove(&mut self, vote: &UserVote) {
        self.vote_pool.retain(|v| vote != v)
    }
    pub fn start_vote(&mut self, vote_options: HashMap<String, i32>) {
        self.vote = Option::from(Vote {
            vote_pool: VecDeque::new(),
            vote_options,
        })
    }
    pub fn close_and_eval(&self, controller: MainController) {
        let mut result: Vec<(&String, &i32)> = Vec::from_iter(&self.vote_options);
        result.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        let result_string = result_string_builder(result);
        if let Err(e) = controller.client.send(result_string) {
            println!("Could not send vote result")
        }
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