use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Editor;

impl Editor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> Result<()> {
        enable_raw_mode()?;
        for byte in io::stdin().bytes() {
            let byte = byte?;
            let c = byte as char;
            if c == 'q' {
                println!("quit");
                disable_raw_mode()?;
                break;
            }

            if c.is_control() {
                println!("ASCII: {0:03}, binary: {0:08b}", byte);
            } else {
                println!("ASCII: {0:03}, binary: {0:08b}, char: {1:}", byte, c);
            }
        }

        Ok(())
    }
}
