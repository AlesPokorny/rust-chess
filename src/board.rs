use crate::helpers::{Move, Position};
use crate::moves::is_field_in_check;
use crate::pieces::{Color, Piece, PieceKind};
use crate::utils::chess_coord_to_position;

use chrono::Local;
use std::collections::HashMap;
use std::fs;
use std::io::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub king_positions: HashMap<Color, Position>,
    pub turn: Color,
    pub next_turn: Color,
    pub en_passant: Option<Position>,
    pub castling: HashMap<Color, [bool; 2]>,
    pub n_half_moves: u16,
    pub n_full_moves: u16,
    pub history: Vec<String>,
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
            king_positions: HashMap::from([
                (
                    Color::White,
                    chess_coord_to_position(String::from("e1")).unwrap(),
                ),
                (
                    Color::Black,
                    chess_coord_to_position(String::from("e8")).unwrap(),
                ),
            ]),
            turn: Color::White,
            next_turn: Color::Black,
            en_passant: None,
            castling: HashMap::from([(Color::White, [true, true]), (Color::Black, [true, true])]),
            n_half_moves: 0_u16,
            n_full_moves: 1_u16,
            history: vec!["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned()],
        };

        for (row_i, row) in temp_board.iter().enumerate() {
            for (col_i, field) in row.iter().enumerate() {
                if field != &' ' {
                    let (piece_kind, color) = Piece::get_piece_kind_and_color(field);
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

    pub fn get_all_moves_of_color(&self, color: Color) -> Vec<Move> {
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

        let mut all_moves: Vec<Move> = Vec::new();
        for piece in color_pieces {
            let piece_moves = piece
                .get_piece_moves(friendly_positions, opponent_positions, self)
                .into_iter()
                .map(|new_position| Move::new(piece.position, new_position));
            all_moves.extend(piece_moves)
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

    pub fn no_possible_moves(&self) -> bool {
        let all_pieces = self.get_pieces();
        let all_positions = self.get_all_positions();
        let friendly_positions = &all_positions[(self.turn == Color::Black) as usize];
        let opponent_positions = &all_positions[(self.turn != Color::Black) as usize];
        let friendly_pieces = &all_pieces[(self.turn == Color::Black) as usize];
        for piece in friendly_pieces {
            let moves = piece.get_piece_moves(friendly_positions, opponent_positions, self);
            if !moves.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn to_fen(&self) -> String {
        let mut fen_string = "".to_owned();

        let prepared_board: Vec<Vec<Option<Piece>>> = (0..8)
            .map(|col| {
                (0..8)
                    .map(|row| self.board[7 - row][7 - col])
                    .collect::<Vec<Option<Piece>>>()
            })
            .collect();

        for (i, board_row) in prepared_board.iter().enumerate() {
            let mut blank_squares = 0;
            for square in board_row.iter() {
                match square {
                    Some(piece) => {
                        if blank_squares > 0 {
                            fen_string.push_str(&blank_squares.to_string())
                        }
                        fen_string.push(piece.get_piece_kind_as_char());
                        blank_squares = 0;
                    }
                    None => blank_squares += 1,
                }
            }
            if blank_squares > 0 {
                fen_string.push_str(&blank_squares.to_string())
            }
            if i < 7 {
                fen_string.push('/');
            }
        }

        fen_string.push(' ');

        if self.turn == Color::White {
            fen_string.push('w');
        } else {
            fen_string.push('b');
        }

        fen_string.push(' ');

        if self.castling[&Color::White][0] {
            fen_string.push('K');
        }
        if self.castling[&Color::White][1] {
            fen_string.push('Q');
        }
        if self.castling[&Color::Black][0] {
            fen_string.push('k');
        }
        if self.castling[&Color::Black][1] {
            fen_string.push('q');
        }

        fen_string.push(' ');

        match self.en_passant {
            Some(position) => fen_string.push_str(&position.get_as_chess_string()),
            None => fen_string.push('-'),
        }

        fen_string.push(' ');

        fen_string.push_str(&self.n_half_moves.to_string());

        fen_string.push(' ');

        fen_string.push_str(&self.n_full_moves.to_string());

        fen_string
    }

    pub fn from_fen(fen: &str) -> Result<Board, Error> {
        let fen_parts: Vec<&str> = fen.split(' ').collect();

        let invalid_fen_error = Error::new(std::io::ErrorKind::InvalidInput, "Invalid FEN string");

        if fen_parts.len() != 6 {
            return Err(invalid_fen_error);
        }
        let board_pieces: Vec<&str> = fen_parts[0]
            .split('/')
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .collect();

        if board_pieces.len() != 8 {
            return Err(invalid_fen_error);
        }

        let mut board: [[Option<Piece>; 8]; 8] = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        let mut king_positions: HashMap<Color, Position> = HashMap::new();

        for (y, row) in board_pieces.iter().enumerate() {
            let mut x: usize = 0;
            for fen_char in row.chars().rev() {
                if (x > 7) | (y > 7) {
                    return Err(invalid_fen_error);
                }
                let fen_char_digit = fen_char as usize;

                if (48..=56).contains(&fen_char_digit) {
                    let n_empty_spaces = fen_char_digit - '0' as usize;
                    x += n_empty_spaces
                } else {
                    if !['p', 'r', 'n', 'b', 'k', 'q'].contains(&fen_char.to_ascii_lowercase()) {
                        return Err(invalid_fen_error);
                    }
                    let (piece_kind, piece_color) = Piece::get_piece_kind_and_color(&fen_char);
                    let position = Position::new(x, y);
                    board[x][y] = Some(Piece::new(piece_color, piece_kind, position));
                    if piece_kind == PieceKind::K {
                        king_positions.insert(piece_color, position);
                    }
                    x += 1;
                }
            }
        }
        let turn: Color;
        let next_turn: Color;
        if fen_parts[1] == "w" {
            turn = Color::White;
            next_turn = Color::Black;
        } else if fen_parts[1] == "b" {
            turn = Color::Black;
            next_turn = Color::White;
        } else {
            return Err(invalid_fen_error);
        };

        let castling_str = fen_parts[2];
        let castling: HashMap<Color, [bool; 2]> = if castling_str == "-" {
            HashMap::from([
                (Color::White, [false, false]),
                (Color::Black, [false, false]),
            ])
        } else {
            let mut white = [false, false];
            let mut black = [false, false];

            for castling_char in castling_str.chars() {
                let i: usize = if castling_char.to_ascii_lowercase() == 'k' {
                    0
                } else if castling_char.to_ascii_lowercase() == 'q' {
                    1
                } else {
                    return Err(invalid_fen_error);
                };

                if castling_char.is_lowercase() {
                    black[i] = true;
                } else {
                    white[i] = true;
                };
            }
            HashMap::from([(Color::White, white), (Color::Black, black)])
        };

        // TODO: castling
        let en_passant = chess_coord_to_position(String::from(fen_parts[3]));

        let n_half_moves = match fen_parts[4].parse::<u16>() {
            Ok(x) => x,
            Err(_) => return Err(invalid_fen_error),
        };
        let n_full_moves = match fen_parts[5].parse::<u16>() {
            Ok(x) => x,
            Err(_) => return Err(invalid_fen_error),
        };
        Ok(Board {
            board,
            king_positions,
            turn,
            next_turn,
            en_passant,
            castling,
            n_half_moves,
            n_full_moves,
            history: vec![fen.to_owned()],
        })
    }

    pub fn increase_half_move(&mut self) {
        self.n_half_moves += 1;
    }

    pub fn increase_full_move(&mut self) {
        self.n_full_moves += 1;
    }

    pub fn reset_half_move(&mut self) {
        self.n_half_moves = 0;
    }

    pub fn is_repetition(&self, n_moves: u16) -> bool {
        let history_len = self.history.len();
        let history_subset: Vec<String> = if history_len >= n_moves as usize {
            self.history[history_len - ((n_moves + 1) as usize)..history_len - 1].to_vec()
        } else {
            self.history[..history_len - 1].to_vec()
        };

        let fen_items_to_find: Vec<&str> = self.history.last().unwrap().split(' ').collect();

        let mut repetition_count = 0_u8;
        for historical_board in history_subset {
            let historical_board_split: Vec<&str> = historical_board.split(' ').collect();

            // checks if board and turn are same
            if (historical_board_split[0] == fen_items_to_find[0])
                & (historical_board_split[1] == fen_items_to_find[1])
            {
                repetition_count += 1;
                if repetition_count >= 2 {
                    return true;
                }
            }
        }
        false
    }

    pub fn count_points(&self) -> (i32, i32) {
        let all_pieces = self.get_pieces();
        let white_points: i32 = all_pieces[0].iter().map(|piece| piece.points).sum();
        let black_points: i32 = all_pieces[1].iter().map(|piece| piece.points).sum();

        (white_points, black_points)
    }

    pub fn try_move(&self, move_to_try: &Move) -> Board {
        let mut board = self.clone();

        board.move_piece(&move_to_try.from, &move_to_try.to);
        board
    }

    pub fn write_to_file(&self) {
        let mut filename = "./logs/".to_owned();
        filename.push_str(&Local::now().to_string());
        filename.push_str(".txt");

        fs::write(filename, self.history.join("\n")).expect("");
    }

    pub fn is_king_in_check(&self, color: &Color) -> bool {
        is_field_in_check(self.king_positions[color], self)
    }

    pub fn is_material_draw(&self) -> bool {
        let all_pieces = self.get_pieces();
        let mut is_draw = true;
        let not_mating_pieces = [PieceKind::N, PieceKind::B];

        if all_pieces[0].len() <= 2 && all_pieces[1].len() <= 2 {
            for color_pieces in all_pieces {
                is_draw &= color_pieces
                    .into_iter()
                    .any(|piece| not_mating_pieces.contains(&piece.kind));
            }
        } else {
            is_draw = false
        }
        is_draw
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

    #[test]
    fn test_from_fen_new_game() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        assert_eq!(board.unwrap(), Board::new())
    }

    #[test]
    fn test_to_fen_new_game() {
        let new_board_fen = Board::new().to_fen();
        let expected_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned();

        assert_eq!(new_board_fen, expected_fen)
    }
}
