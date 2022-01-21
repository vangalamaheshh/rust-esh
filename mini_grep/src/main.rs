use std::env;
use std::process;

use mini_grep::{self, Config};

fn main() {
    let arg_list: Vec<String> = env::args().collect();
    let config = Config::new(&arg_list).unwrap_or_else(|err| {
        println!("Problem parsing the args: {}", err);
        process::exit(1);
    });
    println!("{:#?}; {:#?}", config.query, config.filename);
    //reading file contents
    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}