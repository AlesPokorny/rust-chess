use crate::board::Board;
use crate::gui::utils::*;
use crate::helpers::Position;
use crate::moves::get_rook_old_and_new_castling_positions;
use crate::pieces::{Color, Piece, PieceKind};

use crate::utils::{get_en_passant, was_en_passant_played};

use eframe::egui::{
    self, Button, CentralPanel, Color32, Context, Image, Pos2, Rect, Shape, Ui, Vec2,
};
use eframe::{self, App, Frame};
use std::collections::HashMap;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

pub struct ChessApp<'a> {
    piece_images: HashMap<(PieceKind, Color), Image<'a>>,
    board: Board,
    window_size: f32,
    square_size: f32,
    chosen_piece: Option<Piece>,
    possible_moves: Vec<Position>,
    promotion_position: Option<Position>,
    in_menu: bool,
    in_from_fen: bool,
    font_size: f32,
    fen_string: String,
}

impl<'a> Default for ChessApp<'a> {
    fn default() -> ChessApp<'a> {
        let size = 400.;
        let square_size = size / 8.;

        ChessApp {
            piece_images: init_assets(square_size),
            board: Board::new(),
            window_size: size,
            square_size,
            chosen_piece: None,
            possible_moves: Vec::new(),
            promotion_position: None,
            in_menu: true,
            in_from_fen: false,
            font_size: size / 20.,
            fen_string: String::from(""),
        }
    }
}

impl<'a> App for ChessApp<'a> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let my_frame = egui::containers::Frame::default().fill(Color32::from_rgb(165, 82, 42));

        CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            self.check_window_size(ctx);
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(self.font_size, eframe::epaint::FontFamily::Proportional),
            );

            if self.in_menu {
                self.draw_menu(ui);
            } else if self.in_from_fen {
                self.draw_from_fen(ui);
            } else {
                self.draw_board_with_pieces(ui);
                self.draw_move_selection(ui);

                if let Some(promotion_position) = self.promotion_position {
                    self.do_promotion_stuff(promotion_position, ui, ctx);
                } else if let Some(pos) = ctx.input(|i| i.pointer.press_origin()) {
                    let click_position =
                        convert_click_to_board_position(pos, self.board.turn, self.square_size);
                    match self.chosen_piece {
                        Some(piece) => {
                            if self.possible_moves.contains(&click_position) {
                                self.bust_a_move(piece, click_position);
                                if self.promotion_position.is_none() {
                                    self.end_of_turn_ceremonies();
                                }
                                // The ui is so damn fast that without sleep, it uses the same click multiple times
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
            }
        });
    }
}

impl<'a> ChessApp<'a> {
    fn check_window_size(&mut self, ctx: &Context) {
        let new_size = f32::min(ctx.screen_rect().width(), ctx.screen_rect().height());
        if self.window_size != new_size {
            self.window_size = new_size;
            self.square_size = self.window_size / 8.;
            self.font_size = self.window_size / 20.
        };
    }

    fn draw_menu(&mut self, ui: &mut Ui) {
        let rect_size = Vec2::new(self.window_size / 4., self.window_size / 10.);
        let start_rect = Rect::from_center_size(
            Pos2::new(self.window_size / 2., self.window_size / 6.),
            rect_size,
        );
        let start_button = ui.put(start_rect, Button::new("Start"));

        let start_from_fen_rect = Rect::from_center_size(
            Pos2::new(self.window_size / 2., self.window_size * 2. / 6.),
            rect_size,
        );
        let start_from_fen_button = ui.put(start_from_fen_rect, Button::new("Start from FEN"));

        let options_rect = Rect::from_center_size(
            Pos2::new(self.window_size / 2., self.window_size * 3. / 6.),
            rect_size,
        );
        let options_button = ui.put(options_rect, Button::new("Options"));

        let quit_rect = Rect::from_center_size(
            Pos2::new(self.window_size / 2., self.window_size * 4. / 6.),
            rect_size,
        );
        let quit_button = ui.put(quit_rect, Button::new("Quit"));

        if start_button.clicked() {
            self.in_menu = false;
        } else if options_button.clicked() {
        } else if start_from_fen_button.clicked() {
            self.in_from_fen = true;
            self.in_menu = false;
        } else if quit_button.clicked() {
            exit(0)
        }
    }

    fn draw_from_fen(&mut self, ui: &mut Ui) {
        ui.put(
            Rect::from_center_size(
                Pos2::new(self.window_size / 2., self.window_size / 2.),
                Vec2::new(self.window_size / 1.5, self.window_size / 10.),
            ),
            egui::TextEdit::singleline(&mut self.fen_string),
        );

        let submit_button = ui.put(
            Rect::from_center_size(
                Pos2::new(
                    self.window_size / 2.,
                    self.window_size / 2. + self.window_size / 9.,
                ),
                Vec2::new(self.window_size / 5., self.window_size / 10.),
            ),
            Button::new("Submit"),
        );

        if ui.input(|i| i.key_pressed(egui::Key::Enter)) | submit_button.clicked() {
            match Board::from_fen(self.fen_string.trim()) {
                Ok(board) => {
                    board.print_board(&board.turn);
                    self.board = board;
                    self.in_from_fen = false;
                    self.in_menu = false;
                }
                Err(_) => println!("Invalid FEN string"),
            };
        }
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

    fn do_promotion_stuff(&mut self, promotion_position: Position, ui: &mut Ui, ctx: &Context) {
        let ui_pos =
            convert_board_position_to_ui(&promotion_position, self.board.turn, self.square_size);
        let pieces = [
            Piece::new(self.board.turn, PieceKind::Q, promotion_position),
            Piece::new(self.board.turn, PieceKind::R, promotion_position),
            Piece::new(self.board.turn, PieceKind::N, promotion_position),
            Piece::new(self.board.turn, PieceKind::B, promotion_position),
        ];

        for (i, piece) in pieces.iter().enumerate() {
            let rect_starting_y = ui_pos.y + (i as f32) * self.square_size;
            let rect = Rect::from_min_size(
                Pos2::new(ui_pos.x, rect_starting_y),
                Vec2::new(self.square_size, self.square_size),
            );

            let square = make_square(rect, Color32::from_rgb(255, 255, 255), true);
            ui.painter().add(square);
            let piece_image = self.piece_images.get(&(piece.kind, piece.color)).unwrap();
            piece_image.paint_at(ui, rect);
        }
        if let Some(click_pos) = ctx.input(|i| i.pointer.press_origin()) {
            let click_position =
                convert_click_to_board_position(click_pos, self.board.turn, self.square_size);

            if (promotion_position.x == click_position.x)
                & (promotion_position.y.abs_diff(click_position.y) <= 3)
            {
                let piece = pieces[promotion_position.y.abs_diff(click_position.y)];
                self.board.board[promotion_position.x][promotion_position.y] = Some(piece);
                self.promotion_position = None;
                self.end_of_turn_ceremonies();
            }
        }
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

        let is_capture = !self.board.get_piece_from_position(&to_position).is_none();
        let reset_half_moves = is_capture | (piece_kind == PieceKind::P);

        // update king position
        if piece_kind == PieceKind::K {
            self.board
                .king_positions
                .insert(self.board.turn, to_position);
            if old_position.x.abs_diff(to_position.x) == 2 {
                let (old_rook_position, new_rook_position) =
                    get_rook_old_and_new_castling_positions(&to_position);

                let rook_to_move =
                    match &mut self.board.board[old_rook_position.x][old_rook_position.y] {
                        Some(rook_to_move) => rook_to_move,
                        None => panic!("Oh no! There should be a piece at this position."),
                    };

                rook_to_move.move_piece(new_rook_position);
                self.board
                    .move_piece(&old_rook_position, &new_rook_position);
            }
            self.board.castling.insert(self.board.turn, [false, false]);
        } else if (piece_kind == PieceKind::P) & ((to_position.y == 0) | (to_position.y == 7)) {
            self.promotion_position = Some(to_position);
        }
        let castling = self.board.castling[&self.board.turn];
        let is_rook = piece_kind == PieceKind::R;
        let new_castling = [
            castling[0] & !((is_rook) & (old_position.x == 0)),
            castling[1] & !((is_rook) & (old_position.x == 7)),
        ];
        if castling != new_castling {
            self.board.castling.insert(self.board.turn, new_castling);
        }
        if was_en_passant_played(&piece_kind, &to_position, &self.board.en_passant) {
            self.board
                .remove_piece(&Position::new(to_position.x, old_position.y));
        }

        self.board.en_passant = get_en_passant(&piece_kind, &old_position, &to_position);
        self.board.move_piece(&old_position, &to_position);
        if reset_half_moves {
            self.board.reset_half_move();
        } else {
            self.board.increase_half_move();
        }

        println!("N half moves: {}", self.board.n_half_moves);
    }

    fn end_of_turn_ceremonies(&mut self) {
        self.set_values_at_the_end_of_turn();

        if self.board.is_checkmate() {
            println!("Checkmate!");
            exit(0);
        }

        if self.board.n_half_moves >= 100 {
            println!("Tis draw");
            exit(0);
        }
    }
}
