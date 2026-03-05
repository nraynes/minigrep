use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) {
    let contents: String = fs::read_to_string(config.file_path)
        .expect("There was a problem reading the file!");

    println!("The file has the contents:\n{contents}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("The query is {}!", config.query);
    println!("The file path is {}!", config.file_path);

    run(config);
}
