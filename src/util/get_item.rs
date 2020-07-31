pub fn get_cmd_elem(msg: &str) -> Vec<&str> {
    let mut split = msg.split(" ");
    let options = split.collect::<Vec<&str>>();
    println!("{:?}", options);
    return options[1..].to_vec();
}

