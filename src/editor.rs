mod buffer;
mod terminal;
mod view;

use crate::editor::terminal::{Position, Terminal};
use crate::editor::view::View;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_tuple(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl From<Location> for Position {
    fn from(value: Location) -> Self {
        Self {
            col: value.x,
            row: value.y,
        }
    }
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        self.handle_args().expect("handle args err");
        Terminal::init().expect("failed to init");
        let res = self.repl();
        Terminal::terminate().expect("failed to terminate");

        res.expect("repl error");
    }

    pub fn handle_args(&mut self) -> Result<()> {
        let args = std::env::args().collect::<Vec<_>>();
        if let Some(path) = args.get(1) {
            self.view.load(path)?;
        }

        Ok(())
    }

    fn repl(&mut self) -> Result<()> {
        loop {
            let event = read()?;
            self.eval_event(&event)?;
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("byebye~\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(self.location.into())?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) -> Result<()> {
        if let Event::Key(key_event) = event
            && key_event.is_press()
        {
            match key_event.code {
                KeyCode::Char(c) if c == 'q' && key_event.modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => self.move_point(key_event.code)?,
                _ => (),
            }
        }

        Ok(())
    }

    fn move_point(&mut self, code: KeyCode) -> Result<()> {
        let size = Terminal::size()?;
        let (max_x, max_y) = (size.width.saturating_sub(1), size.height.saturating_sub(1));
        let (x, y) = self.location.to_tuple();
        let loc = match code {
            KeyCode::Left => Location::new(x.saturating_sub(1), y),
            KeyCode::Right if x < max_x => Location::new(x.saturating_add(1), y),
            KeyCode::Up => Location::new(x, y.saturating_sub(1)),
            KeyCode::Down if y < max_y => Location::new(x, y.saturating_add(1)),
            KeyCode::Home => Location::new(0, y),
            KeyCode::End => Location::new(max_x, y),
            KeyCode::PageUp => Location::new(x, 0),
            KeyCode::PageDown => Location::new(x, max_y),
            _ => self.location,
        };

        self.location = loc;

        Ok(())
    }
}
