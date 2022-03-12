extern crate argparse;
use argparse::{ ArgumentParser, Store };

#[derive(Debug)]
struct Options {
    filename: String,
}

#[derive(Debug)]
struct List(Vec<u8>);

/// Reads file content into a giant String
fn read_file_as_string(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(file)?;
    Ok(data)
}

/// Reads file content into a vector of bytes
fn read_file_as_list(file: &str) -> Result<List, Box<dyn std::error::Error>> {
    let data = std::fs::read(file)?;
    Ok(List(data))
}

/// Returns command line args
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
    eprintln!("Reading file content as String.");
    match read_file_as_string(&options.filename) {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("{}", e),
    }
    eprintln!("Reading file as Vector of bytes.");
    match read_file_as_list(&options.filename) {
        Ok(data) => println!("{:?}", data),
        Err(e) => eprintln!("{}", e),
    }
}