use crate::board::Board;
use crate::helpers::Position;
use crate::moves::{
    get_bishop_moves, get_king_moves, get_knight_moves, get_pawn_moves, get_queen_moves,
    get_rook_moves, is_king_in_check,
};

use eframe::egui;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PieceKind {
    P,
    R,
    N,
    B,
    K,
    Q,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub position: Position,
    pub points: i32,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind, position: Position) -> Piece {
        let points = match kind {
            PieceKind::P => 1,
            PieceKind::R => 5,
            PieceKind::N => 3,
            PieceKind::B => 3,
            PieceKind::Q => 9,
            PieceKind::K => 0,
        };
        Piece {
            color,
            kind,
            position,
            points,
            has_moved: false,
        }
    }

    pub fn move_piece(&mut self, new_position: Position) {
        self.position = new_position;
        self.has_moved = true;
    }

    pub fn get_piece_moves(
        &self,
        friendly_positions: &[Position],
        opponent_positions: &[Position],
        en_passant: &Option<Position>,
    ) -> Vec<Position> {
        let all_moves = match self.kind {
            PieceKind::P => get_pawn_moves(
                &self.position,
                &self.has_moved,
                friendly_positions,
                opponent_positions,
                if self.color == Color::White { 1 } else { -1 },
                en_passant,
            ),
            PieceKind::R => get_rook_moves(&self.position, friendly_positions, opponent_positions),
            PieceKind::N => get_knight_moves(&self.position, friendly_positions),
            PieceKind::B => {
                get_bishop_moves(&self.position, friendly_positions, opponent_positions)
            }
            PieceKind::Q => get_queen_moves(&self.position, friendly_positions, opponent_positions),
            PieceKind::K => get_king_moves(&self.position, friendly_positions),
        };
        all_moves
    }

    fn filter_check_moves(moves: &Vec<Position>, mut board: Board) {
        for piece_move in moves {}
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = match self.kind {
            PieceKind::R => 'R',
            PieceKind::N => 'N',
            PieceKind::B => 'B',
            PieceKind::Q => 'Q',
            PieceKind::K => 'K',
            PieceKind::P => 'P',
        };

        if matches!(self.color, Color::Black) {
            output = output.to_ascii_lowercase();
        }

        write!(f, "{}", output)
    }
}