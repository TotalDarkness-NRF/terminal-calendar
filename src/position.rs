use crate::terminal::Terminal;

#[derive(Clone, Copy)]
pub struct Position {
    x: u16,
    y: u16,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.get_x() == other.get_x() && self.get_y() == other.get_y()
    }
}

// TODO maybe look at ordering better

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position { x, y }
    }

    pub fn new_origin() -> Self {
        Position::new(1, 1)
    }

    pub fn new_center() -> Self {
        let bound = Terminal::get_boundaries();
        Position::new(bound.get_x() / 2, bound.get_y() / 2)
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn set_x(&mut self, x: u16) -> bool {
        self.set(x, self.y)
    }

    pub fn set_y(&mut self, y: u16) -> bool {
        self.set(self.x, y)
    }

    pub fn set(&mut self, x: u16, y: u16) -> bool {
        if self.respect_boundary(x, y) {
            self.x = x; self.y = y;
            true
        } else { false }
    }

    fn respect_boundary(&self, x: u16, y: u16) -> bool {
        // Check current and future positions
        self.is_in_boundary() && Position::new(x, y).is_in_boundary()
    }

    pub fn is_in_boundary(&self) -> bool {
        let bounds = &Terminal::get_boundaries();
        self.x > 0 && self.y > 0 && self.get_x() <= bounds.get_x() + 1 && self.get_y() <= bounds.get_y() + 1
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}