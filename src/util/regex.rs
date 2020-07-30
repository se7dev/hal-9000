use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
pub static ref PING: Regex = Regex::new(r"^!ping.*$").unwrap();
pub static ref COMMAND: Regex = Regex::new(r"^!.*$").unwrap();
pub static ref VOTE: Regex = Regex::new(r"^!vote\s.*$").unwrap();
pub static ref STARTVOTE: Regex = Regex::new(r"^!votestart.*$").unwrap();
pub static ref ENDVOTE: Regex = Regex::new(r"^!voteend$").unwrap();
pub static ref STARTGIVEAWAY: Regex = Regex::new(r"^!giveawaystart.*$").unwrap();
pub static ref ENDGIVEAWAY: Regex = Regex::new(r"^!giveawayend.*$").unwrap();
pub static ref ENTERGIVEAWAY: Regex = Regex::new(r"^!giveawayenter.*$").unwrap();
}

