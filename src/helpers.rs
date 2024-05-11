
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn get_valid_position(x: i32, y: i32) -> Option<Position> {
        if (x >= 0) & (x <= 7) & (y >= 0) & (y <= 7) {
            return Some(Position::new(x as usize, y as usize))
        }
        None
    }

    pub fn get_x_y_as_int(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Direction {
        Direction { x, y }
    }
}
