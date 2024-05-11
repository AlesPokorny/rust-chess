use crate::helpers::Position;
use crate::pieces::{Color, Piece, PieceKind};
use crate::utils::chess_coord_to_array_coord;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub king_positions: [Position; 2],
}

impl Board {
    pub fn new() -> Board {
        let temp_board = [
            ['R', 'N', 'B', 'K', 'Q', 'B', 'N', 'R'],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            ['r', 'n', 'b', 'k', 'q', 'b', 'n', 'r'],
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
            ],
            king_positions: [
                chess_coord_to_array_coord(String::from("e1")).unwrap(),
                chess_coord_to_array_coord(String::from("d5")).unwrap(),
            ],
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
                    let color = if field.is_lowercase() {
                        Color::Black
                    } else {
                        Color::White
                    };
                    let position = Position::new(col_i, row_i);

                    result_board.board[col_i][row_i] = Some(Piece::new(color, piece_kind, position))
                }
            }
        }

        result_board
    }

    pub fn print_board(&self, turn: &Color) {
        let mut transposed_board: Vec<Vec<Option<Piece>>> = (0..8)
            .map(|col| (0..8).map(|row| self.board[row][col]).collect())
            .collect();
        let mut column_label = "    H G F E D C B A";

        if turn == &Color::White {
            transposed_board.reverse();
            column_label = "    A B C D E F G H"
        }

        for (row_i, row) in transposed_board.iter().enumerate() {
            let mut row_label = row_i + 1;
            let mut row_to_loop = row.clone();
            if turn == &Color::White {
                row_to_loop.reverse();
                row_label = 9 - row_label;
            }
            print!("{}.| ", row_label);
            for piece in row_to_loop.iter() {
                match piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!("- "),
                };
            }
            println!();
        }
        println!("  |________________");
        println!("{}", column_label);
    }

    pub fn move_piece(&mut self, from: &Position, to: &Position) {
        match self.board[from.x][from.y] {
            Some(piece) => match self.board[to.x][to.y] {
                Some(old_piece) => {
                    if piece.color == old_piece.color {
                        panic!("Something went wrong, trying to overwrite same color piece");
                    } else {
                        self.board[to.x][to.y] = Some(piece);
                    }
                }
                None => self.board[to.x][to.y] = Some(piece),
            },
            None => panic!("No piece at the position {:?}", from),
        }
        self.board[from.x][from.y] = None;
    }

    pub fn get_piece_from_position(&self, position: &Position) -> &Option<Piece> {
        &self.board[position.x][position.y]
    }

    pub fn remove_piece(&mut self, position: &Position) {
        self.board[position.x][position.y] = None
    }

    pub fn get_pieces(&self) -> [Vec<Piece>; 2] {
        let mut white: Vec<Piece> = Vec::new();
        let mut black: Vec<Piece> = Vec::new();
        for row in self.board {
            for piece in row.into_iter().flatten() {
                if piece.color == Color::White {
                    white.push(piece);
                } else {
                    black.push(piece);
                }
            }
        }
        [white, black]
    }

    pub fn get_all_moves_of_color(
        &self,
        color: Color,
        en_passant: &Option<Position>,
    ) -> Vec<Position> {
        let color_index: usize;
        let opponent_index: usize;
        if color == Color::White {
            color_index = 0;
            opponent_index = 1;
        } else {
            color_index = 1;
            opponent_index = 0;
        }
        let all_pieces = self.get_pieces();
        let color_pieces = &all_pieces[color_index];

        let friendly_positions = &self.get_color_positions(color_pieces);
        let opponent_positions = &self.get_color_positions(&all_pieces[opponent_index]);

        let mut all_moves: Vec<Position> = Vec::new();
        for piece in color_pieces {
            let piece_moves =
                piece.get_piece_moves(friendly_positions, opponent_positions, en_passant);
            all_moves.extend(piece_moves);
        }
        all_moves
    }

    pub fn get_color_positions(&self, pieces: &[Piece]) -> Vec<Position> {
        pieces.iter().map(|piece| piece.position).collect()
    }

    pub fn get_all_positions(&self) -> [Vec<Position>; 2] {
        let pieces = self.get_pieces();
        let white_positions = self.get_color_positions(&pieces[0]);
        let black_positions = self.get_color_positions(&pieces[1]);

        [white_positions, black_positions]
    }
}

#[cfg(test)]
mod test_board {
    use crate::board::Board;
    use crate::helpers::Position;
    use crate::pieces::{Color, PieceKind};

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();

        board.move_piece(&Position::new(0, 1), &Position::new(0, 7));

        let target_piece: crate::pieces::Piece = board.board[0][7].unwrap();

        assert_eq!(target_piece.kind, PieceKind::P);
        assert_eq!(target_piece.color, Color::White);

        match board.board[0][1] {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
