use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for byte in io::stdin().bytes() {
        let c = byte.unwrap() as char;
        if c == 'q' {
            println!("quit");
            disable_raw_mode().unwrap();
            return;
        }
        println!("{}", c);
    }
}
