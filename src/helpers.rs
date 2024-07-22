use std::str::from_utf8;

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
        if (0..8).contains(&x) & (0..8).contains(&y) {
            return Some(Position::new(x as usize, y as usize));
        }
        None
    }

    pub fn get_x_y_as_int(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    pub fn get_as_chess_string(&self) -> String {
        let mut string_position = "".to_owned();
        string_position.push_str(from_utf8(&[(104 - self.x) as u8]).unwrap());
        string_position.push_str(from_utf8(&[49 + self.y as u8]).unwrap());

        string_position
    }
}

pub struct Direction {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Move {
        Move { from, to }
    }
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Direction {
        Direction { x, y }
    }
}
