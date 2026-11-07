use std::path::Path;

use crate::editor::Result;
use crate::editor::buffer::Buffer;
use crate::editor::terminal::{Size, Terminal};

// TODO: 也许可以使用 `const_format` crate，这样就能拼接这两个常量了
// const NAME: &str = env!("CARGO_PKG_NAME");
// const VERSION: &str = env!("CARGO_PKG_VERSION");
const WELCOME_MSG: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

#[derive(Default)]
struct Padding {
    left: usize,
    right: usize,
    row: usize,
}

#[derive(Default)]
pub struct View {
    buf: Buffer,
}

impl View {
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let buf = Buffer::load(path)?;
        self.buf = buf;

        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        if self.buf.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buf()?;
        }

        Ok(())
    }

    fn render_buf(&self) -> Result<()> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buf.lines.get(current_row) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row < height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_empty_row() -> Result<()> {
        Terminal::print('~')?;

        Ok(())
    }

    /// 计算欢迎信息距离终端上下左右的边距
    ///
    /// 这里假设 `msg` 是单行并且是 ASCII 字符，水平居中，竖直上位于终端的第 2 / 3 行
    #[allow(clippy::integer_division, reason = "对于终端的位置计算，我不在乎精度")]
    fn cal_welcome_msg_padding(size: &Size, msg: &str) -> Padding {
        let Size { width, height } = size;
        let msg_width = msg.len(); // This length is in bytes

        let left = width.saturating_sub(msg_width) / 2;
        let right = width.saturating_sub(left.saturating_add(msg_width));
        let bottom = height / 3;
        let top = height.saturating_sub(bottom.saturating_add(1)); // 假设 msg 只有一行
        let row = top.saturating_add(1);

        Padding { left, right, row }
    }

    fn render_welcome_screen() -> Result<()> {
        let size = Terminal::size()?;
        let padding = Self::cal_welcome_msg_padding(&size, WELCOME_MSG);
        let row = format!(
            "{}{WELCOME_MSG}{}",
            " ".repeat(padding.left),
            " ".repeat(padding.right)
        );

        for current_row in 0..size.height {
            Terminal::clear_line()?;
            if current_row == padding.row {
                Terminal::print(&row)?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row < size.height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
}
