mod pieces;
mod board;
mod utils;
mod moves;

use crate::board::Board;
use crate::pieces::Color;


fn main() {
    let mut board = Board::new();
    let mut turn = Color::White;

    board.print_board();
}
