use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_name: &String = &args[2];

    println!("Searching for {}", query);
    println!("In file - {}", file_name);

    let content =
        fs::read_to_string(file_name).expect("Unable to read and covert into string from file");

    println!("With text: \n{}", content);
}
