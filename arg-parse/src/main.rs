extern crate argparse;
use argparse::{ ArgumentParser, Store };
use std::fs;

#[derive(Debug)]
struct Options {
    filename: String,
}

fn read_file_as_string(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file)?;
    Ok(data)
}

fn parse_args() ->  Options {
    let mut filename = String::from("");
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Reading File Content");
        parser.refer(&mut filename)
          .add_option(&["-f", "--filename"], Store, "File path");
        parser.parse_args_or_exit();
    }
    Options { filename }
}

fn main() {
    let options: Options = parse_args();
    match read_file_as_string(&options.filename) {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("{}", e),
    }
}