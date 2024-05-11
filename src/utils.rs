use crate::helpers::Position;
use crate::pieces::{Color, PieceKind};

use std::io;

pub fn chess_coord_to_array_coord(coord: String) -> Option<Position> {
    if coord.trim().chars().count() != 2 {
        return None;
    }
    let col = 104 - coord.chars().next().unwrap().to_ascii_lowercase() as i32;
    let row = (coord.chars().nth(1).unwrap() as i32 - '0' as i32) - 1;

    Position::get_valid_position(col, row)
}

#[allow(dead_code)]
pub fn get_user_input(message: &str) -> Option<Position> {
    println!("{}", message);
    let mut from_input = String::new();
    io::stdin()
        .read_line(&mut from_input)
        .expect("Failed to read line");

    chess_coord_to_array_coord(from_input)
}

pub fn get_en_passant(
    piece_kind: &PieceKind,
    from_position: &Position,
    to_position: &Position,
) -> Option<Position> {
    if (piece_kind == &PieceKind::P) & ((from_position.y as i32 - to_position.y as i32).abs() == 2)
    {
        if to_position.y == 3 {
            return Some(Position::new(to_position.x, 2));
        }
        return Some(Position::new(to_position.x, 5));
    }
    None
}

pub fn was_en_passant_played(
    piece_kind: &PieceKind,
    position: &Position,
    en_passant: &Option<Position>,
) -> bool {
    match en_passant {
        Some(en_passant_position) => {
            (en_passant_position == position) & (piece_kind == &PieceKind::P)
        }
        None => false,
    }
}

#[cfg(test)]
mod test_board {
    use crate::helpers::Position;
    use crate::pieces::PieceKind;
    use crate::utils::{chess_coord_to_array_coord, get_en_passant};

    #[test]
    fn test_chess_coord_to_array_coord() {
        assert_eq!(
            Position::get_valid_position(6, 2),
            chess_coord_to_array_coord(String::from("b3"))
        );
    }

    #[test]
    fn test_get_en_passant() {
        let output = get_en_passant(&PieceKind::P, &Position::new(2, 1), &Position::new(2, 3));
        assert_eq!(Some(Position::new(2, 2)), output);

        let output = get_en_passant(&PieceKind::P, &Position::new(2, 7), &Position::new(2, 5));
        assert_eq!(Some(Position::new(2, 5)), output);

        let output = get_en_passant(&PieceKind::P, &Position::new(2, 7), &Position::new(2, 6));
        assert_eq!(None, output);

        let output = get_en_passant(&PieceKind::R, &Position::new(2, 7), &Position::new(2, 5));
        assert_eq!(None, output);
    }
}
