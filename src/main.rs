mod pieces;
mod board;

use crate::board::{create_starting_board, print_board};


fn main() {
    let board = create_starting_board();

    print_board(board);
}
