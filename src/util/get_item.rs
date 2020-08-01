/// Returns the strings after the command string as a Vector of Strings that has been split by whitespace
pub fn get_cmd_elem(msg: &str) -> Vec<&str> {
    let split = msg.split(" ");
    let options = split.collect::<Vec<&str>>();
    println!("{:?}", options);
    options[1..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_elem() {
        let test_string = "!command First";
        let first_element = get_cmd_elem(test_string);
        assert_eq!(first_element[0], "First");
    }
    #[test]
    fn get_further_elem() {
        let test_string = "!command First Second Third";
        let first_element = get_cmd_elem(test_string);
        assert_eq!(first_element[1], "Second");
        assert_eq!(first_element[2], "Third");
    }
    #[test]
    fn no_elem_after_command() {
        let test_string = "!command";
        let first_element = get_cmd_elem(test_string);
        assert_eq!(first_element.is_empty(), true);
    }
}
