mod pieces;
mod board;
mod utils;
mod moves;
mod helpers;

use crate::board::Board;
use crate::pieces::Color;

use std::time::{Duration, Instant};

fn main() {
    let mut board = Board::new();
    let mut turn = Color::White;

    board.print_board();
}
