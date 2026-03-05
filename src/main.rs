use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    println!("The query is {}!", config.query);
    println!("The file path is {}!", config.file_path);

    let contents: String = fs::read_to_string(config.file_path)
        .expect("There was a problem reading the file!");

    println!("The file has the contents:\n{contents}");
}
