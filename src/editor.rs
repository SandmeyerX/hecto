use crate::terminal::Terminal;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
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

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("byebye~\n")?;
        } else {
            Self::draw_rows()?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<()> {
        let (col, row) = Terminal::size()?;
        let info = format!("{NAME} v{VERSION}");
        let (info_x, info_y) = Self::hello_info_position(col, row, &info);

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
    fn hello_info_position(w: u16, h: u16, info: &str) -> (u16, u16) {
        let info_width = u16::try_from(info.len()).expect("hello info too long"); // This length is in bytes

        let x = w.saturating_sub(info_width) / 2; // 水平居中

        #[allow(
            clippy::arithmetic_side_effects,
            reason = "数学上这是绝对安全的，不会溢出"
        )]
        let y = h / 3 * 2; // 垂直 2/3 处

        (x, y)
    }

    fn draw_hello_info(info: &str) -> Result<()> {
        Terminal::print(info)?;

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) {
        if let Event::Key(key_event) = event
            && let KeyCode::Char(c) = key_event.code
            && c == 'q'
            && key_event.modifiers == KeyModifiers::CONTROL
        {
            self.should_quit = true;
        }
    }
}
