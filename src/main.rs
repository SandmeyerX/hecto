use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for byte in io::stdin().bytes() {
        let byte = byte.unwrap();
        let c = byte as char;
        if c == 'q' {
            println!("quit");
            disable_raw_mode().unwrap();
            return;
        }

        if c.is_control() {
            println!("ASCII: {0:03}, binary: {0:08b}", byte);
        }
        else {
            println!("ASCII: {0:03}, binary: {0:08b}, char: {1:}", byte, c);
        }
    }
}
