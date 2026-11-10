use std::ops::Range;

use crate::editor::terminal::Position;

#[derive(Debug, Default, Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn saturating_sub(&self, rhs: &Location) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }

    pub fn to_tuple(self) -> (usize, usize) {
        (self.x, self.y)
    }

    // 视口在 x 轴上的可见列区间 [x, x + width)
    pub fn horizontal_span(&self, width: usize) -> Range<usize> {
        self.x..self.x.saturating_add(width)
    }

    /// 视口在 y 轴上的可见行区间 [y, y + height)
    pub fn vertical_span(&self, height: usize) -> Range<usize> {
        self.y..self.y.saturating_add(height)
    }
}

impl From<Location> for Position {
    fn from(value: Location) -> Self {
        Self {
            col: value.x,
            row: value.y,
        }
    }
}
