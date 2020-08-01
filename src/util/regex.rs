use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
/// Regular Expressions to match incoming chatbot command: send a "pong!" message
pub static ref PING: Regex = Regex::new(r"^!ping.*$").unwrap();
/// Regular Expressions to match incoming chatbot command: Capture any command
pub static ref COMMAND: Regex = Regex::new(r"^!.*$").unwrap();
/// Regular Expressions to match incoming chatbot command: Vote on a motion
pub static ref VOTE: Regex = Regex::new(r"^!vote\s.*$").unwrap();
/// Regular Expressions to match incoming chatbot command: Start a vote
pub static ref STARTVOTE: Regex = Regex::new(r"^!votestart.*$").unwrap();
/// Regular Expressions to match incoming chatbot command: End a vote
pub static ref ENDVOTE: Regex = Regex::new(r"^!voteend$").unwrap();
/// Regular Expressions to match incoming chatbot command: Start a giveaway
pub static ref STARTGIVEAWAY: Regex = Regex::new(r"^!giveawaystart.*$").unwrap();
/// Regular Expressions to match incoming chatbot command: End a giveaway
pub static ref ENDGIVEAWAY: Regex = Regex::new(r"^!giveawayend.*$").unwrap();
/// Regular Expressions to match incoming chatbot command: Enter a giveaway
pub static ref ENTERGIVEAWAY: Regex = Regex::new(r"^!giveawayenter.*$").unwrap();
}
