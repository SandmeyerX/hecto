use crate::editor::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::io::{Write, stdout};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn init() -> Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;

        Ok(())
    }

    pub fn terminate() -> Result<()> {
        disable_raw_mode()?;
        Self::execute()?;

        Ok(())
    }

    pub fn clear_screen() -> Result<()> {
        Self::move_cursor_to(0, 0)?;
        Self::queue(Clear(ClearType::All))?;

        Ok(())
    }

    pub fn clear_line() -> Result<()> {
        Self::queue(Clear(ClearType::CurrentLine))?;

        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<()> {
        Self::queue(MoveTo(x, y))?;

        Ok(())
    }

    pub fn hide_cursor() -> Result<()> {
        Self::queue(Hide)?;

        Ok(())
    }

    pub fn show_cursor() -> Result<()> {
        Self::queue(Show)?;

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

    pub fn size() -> Result<Size> {
        let size = size()?;

        Ok(Size {
            width: size.0,
            height: size.1,
        })
    }
}
