use crate::board::Board;
use crate::gui::utils::*;
use crate::helpers::Position;
use crate::pieces::{Color, Piece, PieceKind};
use crate::moves::get_rook_old_and_new_castling_positions;

use crate::utils::{get_en_passant, was_en_passant_played};

use eframe::egui::{CentralPanel, Color32, Context, Image, Pos2, Rect, Shape, Ui, Vec2};
use eframe::{App, Frame};
use std::collections::HashMap;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

pub struct ChessApp<'a> {
    piece_images: HashMap<(PieceKind, Color), Image<'a>>,
    board: Board,
    square_size: f32,
    chosen_piece: Option<Piece>,
    possible_moves: Vec<Position>,
}

impl<'a> Default for ChessApp<'a> {
    fn default() -> ChessApp<'a> {
        let size = 400.;
        let square_size = size / 8.;

        ChessApp {
            piece_images: init_assets(square_size),
            board: Board::new(),
            square_size,
            chosen_piece: None,
            possible_moves: Vec::new(),
        }
    }
}

impl<'a> App for ChessApp<'a> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.check_window_size(ctx);
            self.draw_board_with_pieces(ui);
            self.draw_move_selection(ui);

            if let Some(pos) = ctx.input(|i| i.pointer.press_origin()) {
                let click_position =
                    convert_click_to_board_position(pos, self.board.turn, self.square_size);
                match self.chosen_piece {
                    Some(piece) => {
                        if self.possible_moves.contains(&click_position) {
                            self.bust_a_move(piece, click_position);
                            self.set_values_at_the_end_of_turn();
                            // The ui is so damn fast that without sleep, it uses the same click multiple times
                            if self.board.is_checkmate() {
                                println!("Checkmate!");
                                exit(0);
                            }
                            sleep(Duration::from_secs_f32(0.1));
                        } else {
                            self.select_piece_and_update_moves(&click_position);
                        }
                    }
                    None => {
                        self.select_piece_and_update_moves(&click_position);
                    }
                }
            }
        });
    }
}

impl<'a> ChessApp<'a> {
    fn check_window_size(&mut self, ctx: &Context) {
        let size = f32::min(ctx.screen_rect().width(), ctx.screen_rect().height());
        self.square_size = size / 8.;
    }

    fn draw_board_with_pieces(&self, ui: &mut Ui) {
        for row in 0..8 {
            for col in 0..8 {
                let mut square_color = Color32::from_rgb(165, 82, 42);
                if (row + col) % 2 == 0 {
                    square_color = Color32::from_rgb(255, 228, 196);
                }
                let row = row as f32;
                let col = col as f32;

                let rect = Rect::from_min_size(
                    Pos2::new(col * self.square_size, row * self.square_size),
                    Vec2::new(self.square_size, self.square_size),
                );

                let square = make_square(rect, square_color, true);
                ui.painter().add(square);

                let mut board_x = col as usize;
                let mut board_y = row as usize;

                if self.board.turn == Color::White {
                    board_x = 7 - board_x;
                    board_y = 7 - board_y;
                }

                if let Some(piece) = self
                    .board
                    .get_piece_from_position(&Position::new(board_x, board_y))
                {
                    let piece_image = self.piece_images.get(&(piece.kind, piece.color)).unwrap();
                    piece_image.paint_at(ui, rect);
                }
            }
        }
    }

    fn draw_move_selection(&mut self, ui: &mut Ui) {
        if let Some(piece) = self.chosen_piece {
            let piece_pos =
                convert_board_position_to_ui(&piece.position, self.board.turn, self.square_size);
            let piece_rect =
                Rect::from_min_size(piece_pos, Vec2::new(self.square_size, self.square_size));
            let square = make_square(piece_rect, Color32::BLUE, false);
            ui.painter().add(square);
        }

        for position in &self.possible_moves {
            let move_pos =
                convert_board_position_to_ui(position, self.board.turn, self.square_size);
            let move_dot = Shape::circle_filled(
                Pos2::new(
                    move_pos.x + (self.square_size / 2.),
                    move_pos.y + (self.square_size / 2.),
                ),
                self.square_size / 10.,
                Color32::GRAY,
            );
            ui.painter().add(move_dot);
        }
    }

    fn select_piece_and_update_moves(&mut self, position: &Position) {
        self.select_piece(position);
        if let Some(piece) = self.chosen_piece {
            self.get_possible_moves(piece);
        } else {
            self.possible_moves = Vec::new();
        }
    }

    fn select_piece(&mut self, position: &Position) -> Option<Piece> {
        let piece_option = self.board.get_piece_from_position(position);
        if let Some(piece) = piece_option {
            if piece.color == self.board.turn {
                self.chosen_piece = Some(*piece);
                println!("{:?}", piece.position);
                return Some(*piece);
            }
        }
        self.chosen_piece = None;
        None
    }

    fn get_possible_moves(&mut self, chosen_piece: Piece) {
        let positions = self.board.get_all_positions();
        let friendly_positions = &positions[(self.board.turn == Color::Black) as usize];
        let opponent_positions = &positions[(self.board.turn == Color::White) as usize];
        self.possible_moves =
            chosen_piece.get_piece_moves(friendly_positions, opponent_positions, &self.board);
    }

    fn set_values_at_the_end_of_turn(&mut self) {
        self.chosen_piece = None;
        if self.board.turn == Color::White {
            self.board.turn = Color::Black;
        } else {
            self.board.turn = Color::White;
        }
        self.possible_moves = Vec::new();
    }

    fn bust_a_move(&mut self, piece: Piece, to_position: Position) {
        let old_position = piece.position;
        let piece_kind = piece.kind;

        // can we make this bit better? use the self.chosen_piece as mutable reference
        // so we dont have to dig it out again?
        // only if I knew how...
        let piece_to_move = match &mut self.board.board[old_position.x][old_position.y] {
            Some(piece_to_move) => piece_to_move,
            None => panic!("Oh no! There should be a piece at this position."),
        };
        piece_to_move.move_piece(to_position);

        // update king position
        if piece_kind == PieceKind::K {
            self.board
                .king_positions
                .insert(self.board.turn, to_position);
            if old_position.x.abs_diff(to_position.x) == 2 {
                let (old_rook_position, new_rook_position) = get_rook_old_and_new_castling_positions(&to_position);

                let rook_to_move = match &mut self.board.board[old_rook_position.x][old_rook_position.y] {
                    Some(rook_to_move) => rook_to_move,
                    None => panic!("Oh no! There should be a piece at this position."),
                };

                rook_to_move.move_piece(new_rook_position);
                self.board.move_piece(&old_rook_position, &new_rook_position);
            }
            self.board.castling.insert(self.board.turn, [false, false]);
        }
        let castling = self.board.castling[&self.board.turn];
        let is_rook = piece_kind == PieceKind::R;
        let new_castling = [
            castling[0] & !((is_rook) & (old_position.x == 0)),
            castling[1] & !((is_rook) & (old_position.x == 7)),
        ];
        if castling != new_castling { self.board.castling.insert(self.board.turn, new_castling); }
        if was_en_passant_played(&piece_kind, &to_position, &self.board.en_passant) {
            self.board
                .remove_piece(&Position::new(to_position.x, old_position.y));
        }

        self.board.en_passant = get_en_passant(&piece_kind, &old_position, &to_position);
        self.board.move_piece(&old_position, &to_position);
    }
}
