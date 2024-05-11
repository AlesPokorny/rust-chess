
use crate::pieces::{ Piece, PieceKind, Color };


pub fn create_starting_board() -> [[Option<Piece>; 8]; 8] {
    let temp_board = [
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ];

    let mut result_board: [[Option<Piece>; 8]; 8] = [
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
    ];

    for (row_i, row) in temp_board.iter().enumerate() {
        for (col_i, field) in row.iter().enumerate() {
            if field != &' ' {
                let piece_kind = match field.to_ascii_lowercase() {
                    'r' => PieceKind::R,
                    'n' => PieceKind::N,
                    'b' => PieceKind::B,
                    'q' => PieceKind::Q,
                    'k' => PieceKind::K,
                    'p' => PieceKind::P,
                    _ => panic!("Unexpected piece"),
                };
                let color = if field.is_lowercase() { Color::Black } else { Color:: White };
                let position = [(7 - row_i) as i8, col_i as i8];

                result_board[row_i][col_i] = Some(Piece::new(color, piece_kind, position))
            }
        }
    }

    result_board
}

pub fn print_board(board: [[Option<Piece>; 8]; 8]) {
    for (row_i, row) in board.iter().enumerate() {
        print!("{}.| ", row_i);
        for piece in row.iter() {
            match piece {
                Some(piece) => print!("{} ", piece),
                None => print!("- "),
            };
        }
        print!("\n");
    }
    println!("  |________________");
    println!("    A B C D E F G H");
}