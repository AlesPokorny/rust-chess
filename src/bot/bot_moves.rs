use crate::board::Board;
use crate::helpers::Move;

use rand::Rng;

pub fn make_random_move(board: &Board) -> Move {
    let all_moves = board.get_all_moves_of_color(board.turn);
    let rand_n = rand::thread_rng().gen_range(0..all_moves.len());

    *all_moves.get(rand_n).unwrap()
}
