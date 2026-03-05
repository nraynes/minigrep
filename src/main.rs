use std::env;
use std::fs;

fn parse_config(args: &Vec<String>) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    let contents = fs::read_to_string(file_path)
        .expect("There was a problem reading the file!");

    println!("The query is {query}!");
    println!("The file path is {file_path}!");

    println!("The file has the contents:\n{contents}");
}
