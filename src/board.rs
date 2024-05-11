
use crate::pieces::{ Piece, PieceKind, Color };
use crate::positions::Position;

pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Board { 
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

        let mut result_board: Board = Board {
            board: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ]
        };

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
                    let position = Position::new(7 - row_i, col_i);

                    result_board.board[row_i][col_i] = Some(Piece::new(color, piece_kind, position))
                }
            }
        }

        result_board
    }

    pub fn print_board(&self) {
        for (row_i, row) in self.board.iter().enumerate() {
            print!("{}.| ", 7 - row_i);
            for piece in row.iter() {
                match piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!("- "),
                };
            }
            println!();
        }
        println!("  |________________");
        println!("    A B C D E F G H");
    }

    pub fn move_piece(&mut self, from: [usize; 2], to: [usize; 2]) {
        match self.board[from[0]][from[1]] {
            Some(piece) => {
                match self.board[to[0]][to[1]] {
                    Some(old_piece) => {
                        if piece.color == old_piece.color {
                            panic!("Something went wrong, trying to overwrite same color piece");
                        } else {
                            self.board[to[0]][to[1]] = Some(piece);
                        }
                    },
                    None => self.board[to[0]][to[1]] = Some(piece),
                }
            },
            None => panic!("No piece at the position {:?}", from),
        }
        self.board[from[0]][from[1]] = None;
    }

    pub fn get_pieces(&self) -> [Vec<Piece>; 2] {
        let mut white: Vec<Piece> = Vec::new();
        let mut black: Vec<Piece> = Vec::new();
        for row in self.board {
            for col in row {
                if let Some(piece) = col {
                    if piece.color == Color::White {
                        white.push(piece);
                    } else {
                        black.push(piece);
                    }
                }
            }
        }
        [white, black]
    }

    pub fn get_color_positions(&self, pieces: &Vec<Piece>) -> Vec<Position> {
        pieces.iter().map(|piece| piece.position).collect()
    }

    pub fn get_all_positions(&self) -> [Vec<Position>; 2] {
        let pieces = self.get_pieces();
        let white_positions = self.get_color_positions(&pieces[0]);
        let black_positions = self.get_color_positions(&pieces[1]);

        [white_positions, black_positions]
    }
}


mod test_board {
    use crate::board::Board;
    use crate::pieces::{Color, PieceKind};

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        
        board.move_piece([1, 0], [2, 0]);

        let target_piece = board.board[2][0].unwrap();

        assert_eq!(target_piece.kind, PieceKind::P);
        assert_eq!(target_piece.color, Color::Black);

        match board.board[1][0] {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
