use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::init().expect("failed to init");
        let res = self.repl();
        Self::terminate().expect("failed to terminate");

        res.expect("repl error");
    }

    fn init() -> Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;

        Ok(())
    }

    fn terminate() -> Result<()> {
        disable_raw_mode()?;

        Ok(())
    }

    fn clear_screen() -> Result<()> {
        let mut stdout = std::io::stdout();
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, Clear(ClearType::All))?;

        Ok(())
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
        Self::draw_rows()?;
        
        if self.should_quit {
            Self::clear_screen()?;
            println!("byebye~");
        }

        Ok(())
    }

    fn draw_rows() -> Result<()> {
        let mut stdout = std::io::stdout();
        let (_col, row) = size()?;
        for i in 0..row {
            execute!(stdout, MoveTo(0, i))?;
            print!("~");
        }
        execute!(stdout, MoveTo(0, 0))?;

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
