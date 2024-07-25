use crate::board::Board;
use crate::helpers::{Move, Position};
use crate::pieces::Color;

use rand::seq::SliceRandom;
use std::cmp::{max, min};
use std::time::Instant;

struct AlphaBetaResult {
    le_move: Move,
    score: i32,
}

impl AlphaBetaResult {
    fn new(le_move: Move, score: i32) -> AlphaBetaResult {
        AlphaBetaResult { le_move, score }
    }

    fn default() -> AlphaBetaResult {
        AlphaBetaResult {
            le_move: Move::new(Position::new(10, 10), Position::new(10, 10)),
            score: std::i32::MAX,
        }
    }
}

pub struct ChessBot {
    pub color: Color,
    pub n_calculations: usize,
    pub max_depth: u8,
}

impl ChessBot {
    pub fn new(color: Color, max_depth: u8) -> ChessBot {
        ChessBot {
            color,
            n_calculations: 0,
            max_depth,
        }
    }

    fn alpha_beta_max(&mut self, board: Board, depth_left: u8, mut alpha: i32, beta: i32) -> i32 {
        if depth_left == 0 {
            self.n_calculations += 1;
            return board.count_points();
        }
        let all_moves = board.get_all_moves_of_color(board.turn);
        if all_moves.is_empty() {
            self.n_calculations += 1;
            return board.count_points();
        }
        let mut score = std::i32::MIN;

        for move_to_try in all_moves {
            if depth_left == 1
                && board.turn == self.color
                && board.get_piece_from_position(&move_to_try.to).is_some()
            {
                continue;
            }
            score = max(
                score,
                self.alpha_beta_min(board.try_move(move_to_try), depth_left - 1, alpha, beta),
            );
            if score >= beta {
                return beta;
            }
            alpha = max(alpha, score);
        }
        alpha
    }

    fn alpha_beta_min(&mut self, board: Board, depth_left: u8, alpha: i32, mut beta: i32) -> i32 {
        if depth_left == 0 {
            self.n_calculations += 1;
            return board.count_points();
        }
        let all_moves = board.get_all_moves_of_color(board.turn);
        if all_moves.is_empty() {
            self.n_calculations += 1;
            return board.count_points();
        }

        let mut score = std::i32::MAX;

        for move_to_try in all_moves {
            if depth_left == 1
                && board.turn == self.color
                && board.get_piece_from_position(&move_to_try.to).is_some()
            {
                continue;
            }
            score = min(
                score,
                self.alpha_beta_max(board.try_move(move_to_try), depth_left - 1, alpha, beta),
            );
            if score <= alpha {
                return alpha;
            }
            beta = min(beta, score);
        }
        beta
    }

    fn alpha_beta_outer(&mut self, board: Board) -> AlphaBetaResult {
        let mut rng = rand::thread_rng();
        let mut score: i32;
        let mut alpha = std::i32::MIN;
        let mut beta = std::i32::MAX;
        let mut best_move = AlphaBetaResult::default();
        let mut all_moves = board.get_all_moves_of_color(board.turn);
        all_moves.shuffle(&mut rng);

        if board.turn == Color::White {
            score = std::i32::MIN;
            for move_to_try in all_moves {
                score = max(
                    score,
                    self.alpha_beta_min(
                        board.try_move(move_to_try),
                        self.max_depth - 1,
                        alpha,
                        beta,
                    ),
                );
                if score >= beta {
                    break;
                }
                if score > alpha {
                    alpha = score;
                    best_move = AlphaBetaResult::new(move_to_try, score);
                }
            }
        } else {
            score = std::i32::MAX;
            for move_to_try in all_moves {
                score = min(
                    score,
                    self.alpha_beta_max(
                        board.try_move(move_to_try),
                        self.max_depth - 1,
                        alpha,
                        beta,
                    ),
                );
                if score <= alpha {
                    break;
                }
                if score < beta {
                    beta = score;
                    best_move = AlphaBetaResult::new(move_to_try, score);
                }
            }
        }

        best_move
    }

    pub fn get_bot_move(&mut self, board: &Board) -> Move {
        self.color = board.turn;
        self.n_calculations = 0;

        let start = Instant::now();
        let best_move = self.alpha_beta_outer(board.clone());
        let end = start.elapsed();

        println!("Highest point move: {}", best_move.score);
        println!(
            "Chosen move {} {}",
            best_move.le_move.from.get_as_chess_string(),
            best_move.le_move.to.get_as_chess_string()
        );

        println!("Calculation took {:?}", end);

        best_move.le_move
    }
}
