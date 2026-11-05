use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Terminal;

impl Terminal {
    pub fn init() -> Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;

        Ok(())
    }

    pub fn terminate() -> Result<()> {
        disable_raw_mode()?;

        Ok(())
    }

    pub fn clear_screen() -> Result<()> {
        let mut stdout = std::io::stdout();
        Self::move_cursor_to(0, 0)?;
        execute!(stdout, Clear(ClearType::All))?;

        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<()> {
        let mut stdout = std::io::stdout();
        execute!(stdout, MoveTo(x, y))?;
        
        Ok(())
    }

    pub fn size() -> Result<(u16, u16)> {
        let size = size()?;

        Ok(size)
    }
}
