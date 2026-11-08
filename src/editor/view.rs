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

pub struct View {
    buf: Buffer,
    need_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            need_redraw: true,
            buf: Buffer::default(),
            size: Size::default(),
        }
    }
}

impl View {
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let buf = Buffer::load(path)?;
        self.buf = buf;
        self.need_redraw = true;

        Ok(())
    }

    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }
        if self.size.width == 0 || self.size.height == 0 {
            return;
        }

        for current_line in 0..self.size.height {
            if self.buf.is_empty() {
                let padding = Self::cal_welcome_msg_padding(&self.size, WELCOME_MSG);
                if current_line == padding.row {
                    let welcome = Self::build_welcome_msg(&padding);
                    Self::render_line(current_line, welcome);
                } else {
                    Self::draw_empty_line(current_line);
                }
            } else if let Some(line) = self.buf.lines.get(current_line) {
                let visible_line = line.get(0..self.size.width).unwrap_or(line);
                Self::render_line(current_line, visible_line);
            } else {
                Self::draw_empty_line(current_line);
            }
        }

        self.need_redraw = false;
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.need_redraw = true;
    }

    fn render_line<T: std::fmt::Display>(at: usize, content: T) {
        let res = Terminal::print_row(at, &content);

        debug_assert!(
            res.is_ok(),
            "failed to render line {at}, content: {content}"
        );
    }

    fn draw_empty_line(at: usize) {
        Self::render_line(at, '~');
    }

    /// 计算欢迎信息距离终端上下左右的边距
    ///
    /// 这里假设 `msg` 是单行并且是 ASCII 字符，水平居中，竖直上位于终端的第 2 / 3 行
    #[allow(clippy::integer_division, reason = "对于终端的位置计算，我不在乎精度")]
    fn cal_welcome_msg_padding(size: &Size, msg: &str) -> Padding {
        let Size { width, height } = size;

        debug_assert!(
            msg.is_ascii(),
            "we can only calculate ASCII width pf welcome msg now"
        );
        let msg_width = msg.len(); // This length is in bytes

        let left = width.saturating_sub(msg_width) / 2;
        let right = width.saturating_sub(left.saturating_add(msg_width));
        let bottom = height / 3;

        debug_assert_eq!(
            msg.lines().count(),
            1,
            "we assume the welcome msg only have one line"
        );
        let top = height.saturating_sub(bottom.saturating_add(1)); // 假设 msg 只有一行
        let row = top.saturating_add(1);

        Padding { left, right, row }
    }

    fn build_welcome_msg(padding: &Padding) -> String {
        format!(
            "{}{WELCOME_MSG}{}",
            " ".repeat(padding.left),
            " ".repeat(padding.right)
        )
    }
}
