use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    println!("{}", args[1]);

    let content = fs::read_to_string(config.file_name)
        .expect("Unable to read and covert into string from file");

    println!("With text: \n{}", content);
}

struct Config {
    query: String,
    file_name: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let file_name = args[2].clone();

    println!("Searching for {}", query);
    println!("In file - {}", file_name);

    Config { query, file_name }
}
