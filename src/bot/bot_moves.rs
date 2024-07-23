use crate::board::Board;
use crate::helpers::Move;
use crate::pieces::Color;

use rand::Rng;
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

fn minmax(board: Board, depth: u8) -> i32 {
    let all_points: Vec<i32> = board
        .get_all_moves_of_color(board.turn)
        .into_iter()
        .map(|move_to_try| {
            let new_board = board.try_move(move_to_try);
            if depth + 1 < 2 {
                minmax(new_board, depth + 1)
            } else {
                new_board.count_points()
            }
        })
        .collect();

    if all_points.len() == 0 {
        if board.is_king_in_check(&board.turn) {
            if board.turn == Color::White {
                return -1000;
            } else {
                return 1000;
            }
        } else {
            return 0;
        }
    }

    if depth % 2 == 0 {
        return *all_points.iter().min().unwrap();
    } else {
        return *all_points.iter().max().unwrap();
    }
}

pub fn get_bot_move(board: &Board) -> Move {
    let start = Instant::now();
    let all_moves = board.get_all_moves_of_color(board.turn);

    let mut move_points: Vec<(usize, i32)> = all_moves
        .iter()
        .enumerate()
        .map(|(n, possible_move)| (n, minmax(board.try_move(*possible_move), 1)))
        .collect();

    move_points.sort_by_key(|(_, points)| *points);
    let highest_point_move = move_points.first().unwrap();
    let highest_points_moves: Vec<&(usize, i32)> = move_points
        .iter()
        .filter(|(_, points)| &highest_point_move.1 == points)
        .collect();
    let rand_n = rand::thread_rng().gen_range(0..highest_points_moves.len());
    let chosen_move = move_points.get(rand_n).unwrap();
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
