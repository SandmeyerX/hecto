use crate::terminal::{Size, Terminal};
use crossterm::event::{Event, KeyCode, KeyModifiers, read};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default, Clone, Copy)]
struct Location {
    x: u16,
    y: u16,
}

impl Location {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn to_tuple(self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Debug, Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::init().expect("failed to init");
        let res = self.repl();
        Terminal::terminate().expect("failed to terminate");

        res.expect("repl error");
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
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("byebye~\n")?;
        } else {
            Self::draw_rows()?;
        }

        Terminal::move_cursor_to(self.location.x, self.location.y)?;
        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<()> {
        let size = Terminal::size()?;
        let (_col, row) = (size.width, size.height);
        let info = format!("{NAME} v{VERSION}");
        let (info_x, info_y) = Self::hello_info_position(&size, &info);

        for y in 0..row {
            Terminal::move_cursor_to(0, y)?;
            Terminal::clear_line()?;

            if y == info_y {
                Terminal::move_cursor_to(info_x, info_y)?;
                Self::draw_hello_info(&info)?;
                continue;
            }

            Self::draw_empty_row()?;
        }
        Terminal::move_cursor_to(0, 0)?;

        Ok(())
    }

    fn draw_empty_row() -> Result<()> {
        Terminal::print('~')?;

        Ok(())
    }

    #[allow(clippy::integer_division, reason = "对于终端的位置计算，我不在乎精度")]
    fn hello_info_position(size: &Size, info: &str) -> (u16, u16) {
        let info_width = u16::try_from(info.len()).expect("hello info too long"); // This length is in bytes

        let x = size.width.saturating_sub(info_width) / 2; // 水平居中

        #[allow(
            clippy::arithmetic_side_effects,
            reason = "数学上这是绝对安全的，不会溢出"
        )]
        let y = size.height / 3 * 2; // 垂直 2/3 处

        (x, y)
    }

    fn draw_hello_info(info: &str) -> Result<()> {
        Terminal::print(info)?;

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
