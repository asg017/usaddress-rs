mod constants;
mod parser;

use crate::parser::Parser;

fn usa() {
    let args: Vec<String> = std::env::args().collect();
    let mut parser = Parser::default();
    match parser.parse(args.get(1).unwrap()) {
        Ok(address) => println!("{:?}", address),
        Err(e) => eprintln!("err"),
    }
}

fn main() {
    usa();
}
