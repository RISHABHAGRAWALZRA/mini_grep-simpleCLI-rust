use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file - {}", config.file_name);

    let content = fs::read_to_string(config.file_name)
        .expect("Unable to read and covert into string from file");

    println!("With text: \n{}", content);
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let query = args[1].clone();
        let file_name = args[2].clone();

        Config { query, file_name }
    }
}
