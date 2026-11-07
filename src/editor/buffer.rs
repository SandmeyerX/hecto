use std::fs;
use std::path::Path;

use crate::editor::Result;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(format!(
                "failed to load file `{}`: it is not a file or does not exist",
                path.display()
            )
            .into());
        }

        let content = fs::read_to_string(path)?;
        let mut lines = Vec::new();
        for line in content.lines() {
            lines.push(line.to_string());
        }

        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
