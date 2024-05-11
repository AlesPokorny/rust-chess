use crate::board::Board;
use crate::helpers::Position;
use crate::pieces::{Color, PieceKind};
use crate::gui::utils::*;

use eframe::egui::{CentralPanel, Context, Image, Pos2, Color32, Shape, Rect, Rounding, Vec2, PointerState};
use eframe::{App, Frame};
use std::collections::HashMap;

use std::thread;
use std::sync::mpsc::{Receiver, channel};

pub struct ChessApp<'a> {
    piece_images: HashMap<(PieceKind, Color), Image<'a>>,
    board: Board,
    window_size: f32,
    square_size: f32,
    turn: Color,
    test: i32,
    receiver: Receiver<i32>,
}

impl<'a> Default for ChessApp<'a> {
    fn default() -> ChessApp<'a> {
        let size = 400.;
        let square_size = size / 8.;
        let (_, receiver): (_, Receiver<i32>) = channel();

        ChessApp {
            piece_images: init_assets(square_size),
            board: Board::new(),
            window_size: size,
            square_size: square_size,
            turn: Color::White,
            test: 0,
            receiver: receiver,
        }
    }
}

impl<'a> App for ChessApp<'a> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            for row in 0..8 {
                for col in 0..8 {
                    let mut square_color = Color32::from_rgb(165, 82, 42);
                    if (row + col) % 2 == 0 {
                        square_color = Color32::from_rgb(255, 228, 196);
                    }
                    let row = row as f32;
                    let col = col as f32;

                    let rect = Rect{
                        min: Pos2{ x: col * self.square_size, y: row * self.square_size}, 
                        max: Pos2{ x: (col + 1.) * self.square_size, y: (row + 1.) * self.square_size}
                    };
                    
                    let square = make_square(rect, square_color);
                    ui.painter().add(square);

                    let mut board_x = col as usize;
                    let mut board_y = row as usize;

                    if self.turn == Color::White {
                        board_x = 7 - board_x;
                        board_y = 7 - board_y;
                    }
    
                    if let Some(piece) = self.board.get_piece_from_position(&Position::new(board_x, board_y)) {
                        let piece_image = self.piece_images.get(&(piece.kind, piece.color)).unwrap();
                        piece_image.paint_at(ui, rect);
                    }
                }
            }

            if let Some(pos) = ctx.input(|i| i.pointer.press_origin()) {
                println!("{:?}", pos);
                

            }
            println!("a");
        }
    );

}}