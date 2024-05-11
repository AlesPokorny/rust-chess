use crate::pieces::Color;
use std::io;

fn chess_coord_to_array_coord(coord: String) -> Option<[usize; 2]> {
    if coord.trim().chars().count() != 2 {
        return None
    }
    let col = 104 - coord.chars().nth(0).unwrap().to_ascii_lowercase() as usize;
    let row = (coord.chars().nth(1).unwrap() as usize - '0' as usize) - 1;

    if (row > 7) | (col > 7) {
        return None
    }

    return Some([col, row])
}

pub fn get_user_input(message: &str) -> Option<[usize; 2]> {
    println!("{}", message);
    let mut from_input = String::new();
    io::stdin()
        .read_line(&mut from_input)
        .expect("Failed to read line");

    chess_coord_to_array_coord(from_input)
}

pub fn change_turn(mut turn: Color) -> Color {
    if turn == Color::White {
        turn = Color::Black;
        println!("--- Black turn ---")
    } else {
        turn = Color::White;
        println!("--- White turn ---");
    }
    turn
}

#[cfg(test)]
mod test_board {
    use crate::utils::chess_coord_to_array_coord;

    #[test]
    fn test_chess_coord_to_array_coord() {

        assert_eq!(Some([6, 2]), chess_coord_to_array_coord(String::from("b3")));

    }
}
