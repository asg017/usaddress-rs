use usaddress::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut parser = Parser::default();
    match parser.parse(args.get(1).unwrap()) {
        Ok(address) => println!("{:?}", address),
        Err(_e) => eprintln!("err"),
    }
}
