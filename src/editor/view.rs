mod buffer;
mod line;
mod location;

use std::path::Path;

use crate::editor::Result;
use crate::editor::command::{Direction, EditorCommand};
use crate::editor::terminal::{Position, Size, Terminal};
use crate::editor::view::buffer::Buffer;
use crate::editor::view::location::Location;

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
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            need_redraw: true,
            buf: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
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

        let offset = self.scroll_offset;
        for current_line in 0..self.size.height {
            if self.buf.is_empty() {
                let padding = Self::cal_welcome_msg_padding(&self.size, WELCOME_MSG);
                if current_line == padding.row {
                    let welcome = Self::build_welcome_msg(&padding);
                    Self::render_line(current_line, welcome);
                } else {
                    Self::draw_empty_line(current_line);
                }
                // TODO: 理解下面的
            } else if let Some(line) = self.buf.lines.get(current_line.saturating_add(offset.y)) {
                let visible_line = line.get(offset.horizontal_span(self.size.width));
                Self::render_line(current_line, visible_line);
            } else {
                Self::draw_empty_line(current_line);
            }
        }

        self.need_redraw = false;
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.need_redraw = true;
    }

    pub fn handle_cmd(&mut self, cmd: EditorCommand) {
        match cmd {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => (),
        }
    }

    pub fn get_position(&self) -> Position {
        self.location.saturating_sub(&self.scroll_offset).into()
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let (mut x, mut y) = self.location.to_tuple();
        let Size { width, height } = self.size;

        match direction {
            Direction::Left => x = x.saturating_sub(1),
            Direction::Right => x = x.saturating_add(1),
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Home => x = 0,
            Direction::End => x = x.saturating_add(width),
            // NOTE: 教程这里写的 PageDown 不符合预期
            Direction::PageUp => y = y.saturating_sub(height),
            Direction::PageDown => y = y.saturating_add(height),
        }

        self.location = Location::new(x, y);
        self.scroll_location_into_view();
    }

    fn scroll_location_into_view(&mut self) {
        let mut offset_changed = false;
        let (x, y) = self.location.to_tuple();
        let Size { width, height } = self.size;

        // 水平滚动
        let horizontal_window = self.scroll_offset.horizontal_span(width);
        if !horizontal_window.contains(&x) {
            // caret 不在窗口内
            self.scroll_offset.x = if x < horizontal_window.start {
                // 在窗口左侧
                x
            } else {
                // 在窗口右侧
                x.saturating_sub(width).saturating_add(1)
            };
            offset_changed = true;
        }

        // 竖直滚动
        let vertical_window = self.scroll_offset.vertical_span(height);
        if !vertical_window.contains(&y) {
            self.scroll_offset.y = if y < vertical_window.start {
                y
            } else {
                y.saturating_sub(height).saturating_add(1)
            };
            offset_changed = true;
        }

        if offset_changed {
            self.need_redraw = true;
        }
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
