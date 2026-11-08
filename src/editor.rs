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

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();

        if self.should_quit {
            let _ = Terminal::print("byebye~\n");
        }
    }
}

impl Editor {
    pub fn build() -> Result<Self> {
        Self::setup_panic_hook();
        Terminal::init()?;

        let mut view = View::default();
        let args = std::env::args().collect::<Vec<_>>();
        if let Some(path) = args.get(1) {
            view.load(path)?;
        }

        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            if self.should_quit {
                break;
            }
            self.refresh_screen();

            match read() {
                Ok(event) => self.eval_event(&event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    panic!("failed to read event: {err}");
                }
            }
        }
    }

    fn setup_panic_hook() {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = Terminal::terminate();
            current_hook(info);
        }));
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.location.into());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn eval_event(&mut self, event: &Event) {
        match event {
            Event::Key(key_event) if key_event.is_press() => match key_event.code {
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
                | KeyCode::PageDown => self.move_point(key_event.code),
                _ => (),
            },
            Event::Resize(width, height) => {
                self.view.resize((*width, *height).into());
            }
            _ => (),
        }
    }

    fn move_point(&mut self, code: KeyCode) {
        let size = Terminal::size().unwrap_or_default();
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
    }
}
