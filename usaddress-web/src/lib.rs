use lazy_static::lazy_static;
use std::sync::Mutex;
use usaddress::Parser;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref PARSER: Mutex<Parser<'static>> = Mutex::new(Parser::default());
}

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    match PARSER.lock().unwrap().parse(input) {
        Ok(parsed) => serde_json::to_string(&parsed).unwrap(),
        Err(_err) => "".to_string(),
    }
}
