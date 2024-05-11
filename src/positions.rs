
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
}
