use crate::editor::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode, size,
};
use crossterm::{Command, queue};
use std::io::{Write, stdout};

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, Default, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl From<(u16, u16)> for Size {
    fn from(value: (u16, u16)) -> Self {
        Self {
            width: usize::from(value.0),
            height: usize::from(value.1),
        }
    }
}

pub struct Terminal;

impl Terminal {
    pub fn init() -> Result<()> {
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()?;
        enable_raw_mode()?;

        Ok(())
    }

    pub fn terminate() -> Result<()> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn clear_screen() -> Result<()> {
        Self::move_caret_to(Position::default())?;
        Self::queue(Clear(ClearType::All))?;

        Ok(())
    }

    pub fn clear_line() -> Result<()> {
        Self::queue(Clear(ClearType::CurrentLine))?;

        Ok(())
    }

    /// 移动 caret 到指定位置
    ///
    /// # NOTE
    /// 即使 [`Position`] 的行列字段值超过 [`u16::MAX`]，最大也只能取到该值
    pub fn move_caret_to(pos: Position) -> Result<()> {
        let col = pos.col.try_into().unwrap_or(u16::MAX);
        let row = pos.row.try_into().unwrap_or(u16::MAX);

        Self::queue(MoveTo(col, row))?;

        Ok(())
    }

    pub fn hide_caret() -> Result<()> {
        Self::queue(Hide)?;

        Ok(())
    }

    pub fn show_caret() -> Result<()> {
        Self::queue(Show)?;

        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<()> {
        Self::queue(EnterAlternateScreen)?;

        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<()> {
        Self::queue(LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn execute() -> Result<()> {
        stdout().flush()?;

        Ok(())
    }

    fn queue<C: Command>(cmd: C) -> Result<()> {
        queue!(stdout(), cmd)?;

        Ok(())
    }

    pub fn print<T: std::fmt::Display>(content: T) -> Result<()> {
        Self::queue(Print(content))?;

        Ok(())
    }

    pub fn print_row<T: std::fmt::Display>(at: usize, content: T) -> Result<()> {
        Self::move_caret_to(Position { col: 0, row: at })?;
        Self::clear_line()?;
        Self::print(content)?;

        Ok(())
    }

    pub fn size() -> Result<Size> {
        let size = size()?;

        Ok(Size {
            width: usize::from(size.0),
            height: usize::from(size.1),
        })
    }
}
