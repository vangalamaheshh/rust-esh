use std::env;
use std::fs;
use std::process;

fn main() {
    let arg_list: Vec<String> = env::args().collect();
    let config = Config::new(&arg_list).unwrap_or_else(|err| {
        println!("Problem parsing the args: {}", err);
        process::exit(1);
    });
    println!("{:#?}; {:#?}", config.query, config.filename);
    //reading file contents
    let contents = fs::read_to_string(config.filename).expect("Something went wrong");
    println!("{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}