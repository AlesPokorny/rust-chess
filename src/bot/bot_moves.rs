use crate::board::Board;
use crate::helpers::Move;

use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

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
            (move_to_check, points)
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

fn alpha_beta_max(board: Board, depth_left: u8, mut alpha: i32, beta: i32) -> (i32, usize) {
    let mut total_moves: usize = 0;
    if depth_left == 0 {
        return (board.count_points(), 1);
    }

    for move_to_try in board.get_all_moves_of_color(board.turn) {
        let (score, n_moves) =
            alpha_beta_min(board.try_move(move_to_try), depth_left - 1, alpha, beta);
        total_moves += n_moves;
        if score >= beta {
            return (beta, total_moves);
        }
        if score > alpha {
            alpha = score;
        }
    }
    (alpha, total_moves)
}

fn alpha_beta_min(board: Board, depth_left: u8, alpha: i32, mut beta: i32) -> (i32, usize) {
    let mut total_moves: usize = 0;
    if depth_left == 0 {
        return (board.count_points(), 1);
    }

    for move_to_try in board.get_all_moves_of_color(board.turn) {
        let (score, n_moves) =
            alpha_beta_max(board.try_move(move_to_try), depth_left - 1, alpha, beta);
        total_moves += n_moves;
        if score <= alpha {
            return (alpha, total_moves);
        }
        if score < beta {
            beta = score;
        }
    }
    (beta, total_moves)
}

pub fn get_bot_move(board: &Board) -> Move {
    let start = Instant::now();
    let all_moves = board.get_all_moves_of_color(board.turn);

    let mut move_points: Vec<(usize, (i32, usize))> = all_moves
        .par_iter()
        .enumerate()
        .map(|(n, possible_move)| {
            (
                n,
                alpha_beta_max(
                    board.try_move(*possible_move),
                    2,
                    std::i32::MIN,
                    std::i32::MAX,
                ),
            )
        })
        .collect();

    let total_moves: usize = move_points
        .par_iter()
        .map(|(_, (_, n_moves))| n_moves)
        .sum();

    move_points.sort_by_key(|(_, points)| *points);
    let highest_point_move = move_points.first().unwrap();
    let highest_points_moves: Vec<(usize, i32)> = move_points
        .iter()
        .map(|(x, (y, _))| (*x, *y))
        .filter(|(_, points)| &highest_point_move.1 .0 == points)
        .collect();
    let rand_n = rand::thread_rng().gen_range(0..highest_points_moves.len());
    let chosen_move = move_points.get(rand_n).unwrap();
    println!("Evaluated {} moves", total_moves);
    println!("Highest point move: {:?}", highest_point_move.1);
    println!("Chosen move {:?}", chosen_move);
    println!(
        "n moves: {} and after filter: {}",
        all_moves.len(),
        highest_points_moves.len()
    );
    let end = start.elapsed();
    println!("Calculation took {:?}", end);

    all_moves[chosen_move.0]
}
