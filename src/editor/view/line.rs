use std::ops::Range;

pub struct Line {
    inner: String,
}

impl Line {
    /// 返回子字符串
    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = range.end.min(self.inner.len());
        self.inner.get(start..end).unwrap_or_default().to_string()
    }
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Self {
            inner: String::from(value),
        }
    }
}
