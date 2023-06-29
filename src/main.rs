use mini_grep_simpleCLI_rust::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file - {}", config.file_name);

    if let Err(e) = mini_grep_simpleCLI_rust::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
