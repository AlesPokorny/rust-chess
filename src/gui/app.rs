use crate::board::Board;
use crate::bot::find_best_point_move_depth_one;
use crate::gui::utils::*;
use crate::helpers::{Move, Position};
use crate::moves::get_rook_old_and_new_castling_positions;
use crate::pieces::{Color, Piece, PieceKind};

use crate::utils::{get_en_passant, was_en_passant_played};

use eframe::egui::{
    self, Align2, Button, CentralPanel, Color32, Context, Image, Layout, Pos2, Rect, RichText,
    Shape, Ui, Vec2,
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
    in_options: bool,
    font_size: f32,
    fen_string: String,
    white_color: [f32; 3],
    black_color: [f32; 3],
    colors: [[f32; 3]; 2],
    player_color: Color,
}

impl<'a> Default for ChessApp<'a> {
    fn default() -> ChessApp<'a> {
        let size = 600.;
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
            in_options: false,
            font_size: size / 20.,
            fen_string: String::from(""),
            white_color: [255., 228., 196.],
            black_color: [165., 82., 42.],
            colors: [[255., 228., 196.], [165., 82., 42.]],
            player_color: Color::White,
        }
    }
}

impl<'a> App for ChessApp<'a> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.used_rect().set_height(self.window_size);
        ctx.screen_rect().set_width(self.window_size);

        let my_frame = egui::containers::Frame::default().fill(Color32::from_rgb(100, 100, 100));

        CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            ui.set_min_size(Vec2::new(self.window_size, self.window_size));
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(self.font_size, eframe::epaint::FontFamily::Proportional),
            );

            if self.in_menu {
                self.draw_menu(ui);
            } else if self.in_from_fen {
                self.draw_from_fen(ui);
            } else if self.in_options {
                self.draw_options(ui, ctx);
            } else {
                self.draw_board_with_pieces(ui);
                self.draw_move_selection(ui);

                if self.player_color == self.board.turn {
                    if let Some(promotion_position) = self.promotion_position {
                        println!("how did I get here?");
                        self.do_promotion_stuff(promotion_position, ui, ctx);
                    } else if let Some(pos) = ctx.input(|i| i.pointer.press_origin()) {
                        let click_position = convert_click_to_board_position(
                            pos,
                            self.player_color,
                            self.square_size,
                        );
                        match self.chosen_piece {
                            Some(piece) => {
                                if self.possible_moves.contains(&click_position) {
                                    self.bust_a_move(Move::new(piece.position, click_position));
                                    if self.promotion_position.is_none() {
                                        self.end_of_turn_ceremonies();
                                    }
                                    // The ui is so damn fast that without sleep, it uses the same click multiple times
                                    sleep(Duration::from_secs_f32(0.3));
                                } else {
                                    self.select_piece_and_update_moves(&click_position);
                                }
                            }
                            None => {
                                self.select_piece_and_update_moves(&click_position);
                            }
                        }
                    }
                } else {
                    let bot_move = find_best_point_move_depth_one(&self.board);
                    self.bust_a_move(bot_move);
                    if self.promotion_position.is_some() {
                        let new_piece =
                            Some(Piece::new(self.board.turn, PieceKind::Q, bot_move.to));
                        self.board.board[bot_move.to.x][bot_move.to.y] = new_piece;
                        self.promotion_position = None;
                    }
                    self.end_of_turn_ceremonies();
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
            self.in_menu = false;
            self.in_options = true;
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
                }
                Err(_) => println!("Invalid FEN string"),
            };
        }
    }

    fn draw_color_wheel(&mut self, ui: &mut Ui) {
        egui::color_picker::color_edit_button_rgb(ui, &mut self.white_color);
    }

    fn draw_options(&mut self, ui: &mut Ui, ctx: &Context) {
        self.draw_page_title("OPTIONS", ui);
        let slider_ui_size = Vec2::new(2. * self.window_size / 3., self.window_size / 4.);

        let white_rect = Rect::from_min_size(
            Pos2::new(self.window_size / 6., self.window_size / 5.),
            slider_ui_size,
        );
        let mut white_ui = ui.child_ui(white_rect, Layout::default());
        draw_color_sliders(
            &mut self.white_color,
            &mut white_ui,
            "Light square background",
            self.font_size * 0.8,
        );

        let black_rect = Rect::from_min_size(
            Pos2::new(
                white_rect.left_top().x,
                white_rect.left_top().y + slider_ui_size.y,
            ),
            slider_ui_size,
        );
        let mut black_ui = ui.child_ui(black_rect, Layout::default());
        draw_color_sliders(
            &mut self.black_color,
            &mut black_ui,
            "Dark square background",
            self.font_size * 0.8,
        );

        let resolution_height = 0.1 * self.window_size;
        let resolution_rect = Rect::from_min_size(
            Pos2::new(
                white_rect.left_top().x,
                black_rect.left_top().y + slider_ui_size.y,
            ),
            Vec2::new(2. * self.window_size / 3., resolution_height),
        );
        let mut resolution_ui =
            ui.child_ui(resolution_rect, Layout::left_to_right(egui::Align::LEFT));
        egui::ComboBox::from_label(RichText::new("Resolution").size(self.font_size * 0.8))
            .selected_text(format!(
                "{}x{}",
                self.window_size as i32, self.window_size as i32
            ))
            .show_ui(&mut resolution_ui, |ui| {
                ui.selectable_value(&mut self.window_size, 600., "600x600");
                ui.selectable_value(&mut self.window_size, 800., "800x800");
            });
        ctx.screen_rect().set_height(self.window_size);
        ctx.screen_rect().set_width(self.window_size);

        let back_button = ui.put(
            Rect::from_center_size(
                Pos2::new(self.window_size / 2., self.window_size * 0.9),
                Vec2::new(self.window_size / 5., resolution_height),
            ),
            Button::new("Back"),
        );

        if ui.input(|i| i.key_pressed(egui::Key::Enter)) | back_button.clicked() {
            self.in_menu = true;
            self.in_options = false;
        }
    }

    fn draw_page_title(&self, label: &str, ui: &mut Ui) {
        // Layout::centered_and_justified(Label::new(RichText::new(label).size(self.font_size).strong()));
        ui.with_layout(
            Layout::top_down_justified(egui::Align::Center),
            |label_ui| {
                label_ui.painter().text(
                    label_ui.max_rect().center_top() + Vec2::new(0.0, self.square_size / 2.0),
                    Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(self.font_size),
                    Color32::from_rgb(255, 255, 255),
                );
            },
        );
    }

    fn draw_board_with_pieces(&self, ui: &mut Ui) {
        for row in 0..8 {
            for col in 0..8 {
                let mut square_color = Color32::from_rgb(
                    self.black_color[0] as u8,
                    self.black_color[1] as u8,
                    self.black_color[2] as u8,
                );
                if (row + col) % 2 == 0 {
                    square_color = Color32::from_rgb(
                        self.white_color[0] as u8,
                        self.white_color[1] as u8,
                        self.white_color[2] as u8,
                    );
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

                if self.player_color == Color::White {
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
                convert_board_position_to_ui(&piece.position, self.player_color, self.square_size);
            let piece_rect =
                Rect::from_min_size(piece_pos, Vec2::new(self.square_size, self.square_size));
            let square = make_square(piece_rect, Color32::BLUE, false);
            ui.painter().add(square);
        }

        for position in &self.possible_moves {
            let move_pos =
                convert_board_position_to_ui(position, self.player_color, self.square_size);
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
            self.board.increase_full_move();
            self.board.turn = Color::White;
        }
        self.possible_moves = Vec::new();
        self.board.history.push(self.board.to_fen());
    }

    fn do_promotion_stuff(&mut self, promotion_position: Position, ui: &mut Ui, ctx: &Context) {
        let ui_pos =
            convert_board_position_to_ui(&promotion_position, self.player_color, self.square_size);
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
                convert_click_to_board_position(click_pos, self.player_color, self.square_size);

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

    fn bust_a_move(&mut self, piece_move: Move) {
        let piece = self
            .board
            .get_piece_from_position(&piece_move.from)
            .unwrap();
        let piece_kind = piece.kind;

        // can we make this bit better? use the self.chosen_piece as mutable reference
        // so we dont have to dig it out again?
        // only if I knew how...
        let piece_to_move = match &mut self.board.board[piece_move.from.x][piece_move.from.y] {
            Some(piece_to_move) => piece_to_move,
            None => panic!("Oh no! There should be a piece at this position."),
        };
        piece_to_move.move_piece(piece_move.to);

        let is_capture = !self.board.get_piece_from_position(&piece_move.to).is_none();
        let reset_half_moves = is_capture | (piece_kind == PieceKind::P);

        // update king position
        if piece_kind == PieceKind::K {
            self.board
                .king_positions
                .insert(self.board.turn, piece_move.to);
            if piece_move.from.x.abs_diff(piece_move.to.x) == 2 {
                let (old_rook_position, new_rook_position) =
                    get_rook_old_and_new_castling_positions(&piece_move.to);

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
        } else if (piece_kind == PieceKind::P) & ((piece_move.to.y == 0) | (piece_move.to.y == 7)) {
            self.promotion_position = Some(piece_move.to);
        }
        let castling = self.board.castling[&self.board.turn];
        let is_rook = piece_kind == PieceKind::R;
        let new_castling = [
            castling[0] & !((is_rook) & (piece_move.from.x == 0)),
            castling[1] & !((is_rook) & (piece_move.from.x == 7)),
        ];
        if castling != new_castling {
            self.board.castling.insert(self.board.turn, new_castling);
        }
        if was_en_passant_played(&piece_kind, &piece_move.to, &self.board.en_passant) {
            self.board
                .remove_piece(&Position::new(piece_move.to.x, piece_move.from.y));
        }

        self.board.en_passant = get_en_passant(&piece_kind, &piece_move.from, &piece_move.to);
        self.board.move_piece(&piece_move.from, &piece_move.to);
        if reset_half_moves {
            self.board.reset_half_move();
        } else {
            self.board.increase_half_move();
        }

        println!("N half moves: {}", self.board.n_half_moves);
    }

    fn end_of_turn_ceremonies(&mut self) {
        self.set_values_at_the_end_of_turn();

        if self.board.no_possible_moves() {
            if self.board.is_king_in_check(&self.board.turn) {
                let winning_color = if self.board.turn == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
                println!("Checkmate! {:?} won", winning_color);
            } else {
                println!("Draw!");
            }
            self.board.write_to_file();
            exit(0);
        }

        if self.board.n_half_moves > 7 && self.board.is_repetition(self.board.n_half_moves) {
            println!("Repetition draw. Y'all suck!");
            self.board.write_to_file();
            exit(0);
        }

        if self.board.n_half_moves >= 100 || self.board.is_material_draw() {
            println!("Tis a draw");
            self.board.write_to_file();
            exit(0);
        }
    }
}
