mod pieces;
mod board;
mod utils;
mod moves;
mod helpers;

use helpers::Position;
use utils::change_turn;

use crate::board::Board;
use crate::pieces::Color;
use crate::utils::get_user_input;

fn main() {
    let mut board = Board::new();
    let mut turn = Color::White;
    let mut turn_number: u32 = 0;
    let mut prev_turn_number: u32 = 0;

    loop {
        if turn_number == prev_turn_number {
            println!("Try again");
        } else {
            turn = change_turn(turn);
            prev_turn_number = turn_number;
        }

        board.print_board(&turn);
        let from_xy = match get_user_input("Select piece: ") {
            Some(from) => from,
            None => {
                println!("ERROR: Wrong coordinates");
                continue;
            },
        };

        let all_positions = board.get_all_positions();
        let piece = match &mut board.board[from_xy[0]][from_xy[1]] {
            Some(piece) => piece,
            None => {
                println!{"ERROR: No piece found at the coordinates"};
                continue;
            }
        };
        println!("{:?}", piece);

        if piece.color != turn {
            println!{"ERROR: Wrong color, dumbp"};
            continue;
        }

        let possible_moves = &piece.get_piece_moves(&all_positions[0], &all_positions[1], &None);
        println!("{:?}", possible_moves);
        let to_xy = match get_user_input("Move to: ") {
            Some(to) => to,
            None => {
                println!("ERROR: Wrong coordinates");
                continue;
            },
        };
        let new_position = match Position::get_valid_position(to_xy[0] as i32, to_xy[1] as i32) {
            Some(position) => position,
            None => {
                println!("ERROR: This position is not on the board");
                continue;
            }
        };

        if !possible_moves.contains(&new_position) {
            println!("ERROR: This move is not possible");
            continue;
        }
        piece.move_piece(new_position);
        board.move_piece(from_xy, to_xy);
        turn_number += 1;
    }
}
