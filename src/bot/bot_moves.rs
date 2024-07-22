use crate::board::Board;
use crate::helpers::Move;

use rand::Rng;

pub fn find_random_move(board: &Board) -> Move {
    let all_moves = board.get_all_moves_of_color(board.turn);
    let rand_n = rand::thread_rng().gen_range(0..all_moves.len());

    *all_moves.get(rand_n).unwrap()
}

pub fn find_best_point_move_depth_one(board: &Board) -> Move {
    let mut moves_with_points = board
        .get_all_moves_of_color(board.turn)
        .into_iter()
        .map(|move_to_check| {
            let points = board.try_move(move_to_check).count_points();
            (
                move_to_check,
                points[&board.turn] - points[&board.next_turn],
            )
        })
        .collect::<Vec<(Move, i32)>>();
    moves_with_points.sort_by_key(|item| -item.1);
    let highest_points_moves: Vec<&(Move, i32)> = moves_with_points
        .iter()
        .filter(|x| moves_with_points.first().unwrap().1 == x.1)
        .collect();

    let rand_n = rand::thread_rng().gen_range(0..highest_points_moves.len());

    highest_points_moves.get(rand_n).unwrap().0
}
