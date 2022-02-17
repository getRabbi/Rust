extern crate whatlang;

use whatlang::detect;
use std::io;

fn main() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to get input");
    let match_lang = detect(&input_text).expect("something wrong");
    println!("Detected Language: {:?}", match_lang.lang());
    println!("Detected script: {:?}", match_lang.script());
    println!("Is reliable: {}", match_lang.is_reliable())
}