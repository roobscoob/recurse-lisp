use std::io::Read;

use clap::Parser;
use logos::Logos;

#[derive(clap::Parser)]
struct Command {
    filepath: String,
}

fn main() {
    let Command { filepath } = Command::parse();

    let mut file = std::fs::File::open(filepath).expect("Failed to open file");

    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();

    let mut parser = parse::lexer::Token::lexer(str.as_str());

    let value = parse::parse(&mut parser).unwrap().unwrap();

    println!("{:?}", value);
}
