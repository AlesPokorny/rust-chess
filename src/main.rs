mod gui;
mod board;
mod helpers;
mod moves;
mod pieces;
mod utils;

use crate::gui::{make_square, init_assets, ChessApp};



use crate::helpers::Position;
use crate::utils::change_turn;

use crate::board::Board;
use crate::pieces::{Color, PieceKind};
use crate::utils::{get_en_passant, get_user_input, was_en_passant_played};


use eframe::egui::{self, Pos2, Color32, Shape, Rect, Rounding, Vec2, PointerState};

fn main() -> Result<(), eframe::Error> {
    let window_size = (400., 400.);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size(window_size),
        ..Default::default()
    };

    let turn = Color::White;

    let pointer_state = PointerState::default();
    pointer_state.interact_pos();

    eframe::run_native(
        "Chess", 
        options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<ChessApp>::default()

            

        })
    )
}


fn chess() {
    let mut board = Board::new();
    let mut turn = Color::White;
    let mut turn_number: u32 = 0;
    let mut prev_turn_number: u32 = 0;
    let mut en_passant: Option<Position> = None;

    loop {
        if turn_number == prev_turn_number {
            println!("Try again");
        } else {
            turn = change_turn(turn);
            prev_turn_number = turn_number;
        }
        let mut orig_board = board.clone();

        board.print_board(&turn);
        let from_position = match get_user_input("Select piece: ") {
            Some(from) => from,
            None => {
                println!("ERROR: Wrong coordinates");
                continue;
            }
        };

        let all_positions = board.get_all_positions();

        let piece = match &mut board.board[from_position.x][from_position.y] {
            // let piece = match &mut board.get_piece_from_position(&from_position) {
            Some(piece) => piece,
            None => {
                println! {"ERROR: No piece found at the coordinates"};
                continue;
            }
        };
        let piece_kind = piece.kind;
        println!("{:?}", piece);

        if piece.color != turn {
            println! {"ERROR: Wrong color, dumbp"};
            continue;
        }

        let possible_moves =
            &piece.get_piece_moves(&all_positions[0], &all_positions[1], &en_passant);

        println!("{:?}", possible_moves);
        let new_position = match get_user_input("Move to: ") {
            Some(to) => to,
            None => {
                println!("ERROR: Wrong coordinates");
                continue;
            }
        };

        if !possible_moves.contains(&new_position) {
            println!("ERROR: This move is not possible");
            continue;
        }

        piece.move_piece(new_position);

        if piece_kind == PieceKind::K {
            board.king_positions[(turn == Color::Black) as usize] = new_position;
        }

        if was_en_passant_played(&piece_kind, &new_position, &en_passant) {
            board.remove_piece(&Position::new(new_position.x, from_position.y));
        }
        println!("EN PASSANT: {:?}", en_passant);
        en_passant = get_en_passant(&piece_kind, &from_position, &new_position);
        board.move_piece(&from_position, &new_position);

        turn_number += 1;
    }
}

