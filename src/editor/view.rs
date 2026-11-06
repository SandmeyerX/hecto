use std::fs;
use std::path::Path;

use crate::editor::Result;
use crate::editor::buffer::Buffer;
use crate::editor::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buf: Buffer,
}

impl View {
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(format!(
                "failed to load file `{}`: it is not a file or does not exist",
                path.display()
            )
            .into());
        }

        let content = fs::read_to_string(path)?;
        for line in content.lines() {
            self.buf.lines.push(line.to_string());
        }

        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        let size = Terminal::size()?;
        let (_col, row) = (size.width, size.height);
        let info = format!("{NAME} v{VERSION}");
        let (info_x, info_y) = Self::hello_info_position(&size, &info);

        for y in 0..row {
            Terminal::move_cursor_to(0, y)?;
            Terminal::clear_line()?;

            if let Some(line) = self.buf.lines.get(usize::from(y)) {
                Terminal::print(line)?;
            }
            if y == info_y && self.buf.is_empty() {
                Terminal::move_cursor_to(info_x, info_y)?;
                Self::draw_hello_info(&info)?;
            } else {
                Self::draw_empty_row()?;
            }
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
}
