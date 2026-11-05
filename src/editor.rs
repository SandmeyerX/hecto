use crossterm::event::{Event, KeyCode, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Editor;

impl Editor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("err: {}", err);
        }
        println!("byebye~");
    }

    fn repl(&self) -> Result<()> {
        enable_raw_mode()?;
        loop {
            match read()? {
                Event::Key(key_event) => {
                    println!("Key event: {:#?}", key_event);
                    match key_event.code {
                        KeyCode::Char(c) => {
                            if c == 'q' {
                                break;
                            }
                            println!("ASCII: {0:03}, binary: {0:08b}, char: {1:}", c as u8, c);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        disable_raw_mode()?;

        Ok(())
    }
}
