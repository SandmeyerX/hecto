use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::queue;
use std::io::{Write, stdout};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
        queue!(stdout(), Clear(ClearType::All))?;

        Ok(())
    }

    pub fn clear_line() -> Result<()> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;

        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<()> {
        queue!(stdout(), MoveTo(x, y))?;

        Ok(())
    }

    pub fn hide_cursor() -> Result<()> {
        queue!(stdout(), Hide)?;

        Ok(())
    }

    pub fn show_cursor() -> Result<()> {
        queue!(stdout(), Show)?;

        Ok(())
    }

    pub fn execute() -> Result<()> {
        stdout().flush()?;
        
        Ok(())
    }

    pub fn print<T: std::fmt::Display>(content: T) -> Result<()> {
        queue!(stdout(), Print(content))?;
        
        Ok(())
    }

    pub fn size() -> Result<(u16, u16)> {
        let size = size()?;

        Ok(size)
    }
}
