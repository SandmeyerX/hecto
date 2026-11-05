use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("err: {err}");
        }
        println!("byebye~");
    }

    fn repl(&mut self) -> Result<()> {
        enable_raw_mode()?;
        loop {
            if let Event::Key(key_event) = read()? {
                // println!("Key event: {key_event:#?}");
                match key_event.code {
                    KeyCode::Char(c) => {
                        if c == 'q' && key_event.modifiers == KeyModifiers::CONTROL {
                            self.should_quit = true;
                        }
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;

        Ok(())
    }
}
