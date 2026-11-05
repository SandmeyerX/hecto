use crate::terminal::Terminal;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::init().expect("failed to init");
        let res = self.repl();
        Terminal::terminate().expect("failed to terminate");

        res.expect("repl error");
    }

    fn repl(&mut self) -> Result<()> {
        loop {
            let event = read()?;
            self.eval_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        Terminal::hide_cursor()?;
        Self::draw_rows()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("byebye~\n")?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<()> {
        let (_col, row) = Terminal::size()?;
        for i in 0..row {
            Terminal::move_cursor_to(0, i)?;
            Terminal::clear_line()?;
            Terminal::print("~")?;
        }
        Terminal::move_cursor_to(0, 0)?;

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char(c) => {
                    if c == 'q' && key_event.modifiers == KeyModifiers::CONTROL {
                        self.should_quit = true;
                    }
                }
                _ => (),
            }
        }
    }
}
