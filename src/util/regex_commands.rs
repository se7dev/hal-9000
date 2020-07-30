use regex::Regex;

pub static COMMAND: Regex = Regex::new(r"^!.*$").unwrap();
pub static VOTE: Regex = Regex::new(r"^!vote.*$").unwrap();
pub static STARTVOTE: Regex = Regex::new(r"^!votestart.*$").unwrap();
pub static ENDVOTE: Regex = Regex::new(r"^!voteend.*$").unwrap();
