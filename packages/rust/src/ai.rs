use rand::prelude::SliceRandom;

use crate::{
    board::{Board, Player},
    moves::Move,
    piece::Piece,
};

pub const DEPTH: usize = 4;

pub struct Ai {
    player: Player,
    color: Piece,
}

impl Ai {
    pub fn new(player: Player) -> Ai {
        let color = if let Player::White = player {
            Piece::WHITE
        } else {
            Piece::BLACK
        };
        Ai { player, color }
    }

    fn eval(&self, board: &Board) -> f32 {
        let mut score: f32 = 0.0;
        for pos in board.positions() {
            let piece = board[pos];
            if piece.color() == self.color {
                score += piece.score() as f32;
            } else {
                score -= piece.score() as f32;
            }
        }

        let move_count = board.generate_all_moves().len();

        score += 0.1 * move_count as f32;

        score
    }

    fn alpha_beta_negamax(
        &self,
        board: &Board,
        depth: usize,
        mut alpha: f32,
        mut beta: f32,
        max_player: bool,
    ) -> f32 {
        if depth == 0 || board.is_game_over() {
            return self.eval(board);
        }

        if max_player {
            let mut val = -f32::INFINITY;
            for move_ in board.generate_all_moves() {
                let mut board_ = board.clone();
                move_.execute(&mut board_);
                val = val.max(self.alpha_beta_negamax(&board_, depth - 1, -beta, -alpha, false));
                if val >= beta {
                    break; // Beta Cutoff
                }
                alpha = alpha.max(val);
            }
            val
        } else {
            let mut val = f32::INFINITY;
            for move_ in board.generate_all_moves() {
                let mut board_ = board.clone();
                move_.execute(&mut board_);
                val = val.min(self.alpha_beta_negamax(&board_, depth - 1, -beta, -alpha, true));
                if val <= alpha {
                    break; // Alpha Cutoff
                }
                beta = beta.min(val);
            }
            val
        }
    }

    pub fn eval_board(&self, board: &Board) -> f32 {
        self.alpha_beta_negamax(board, DEPTH, -f32::INFINITY, f32::INFINITY, true)
    }

    pub fn best_move(&self, board: &Board) -> Option<Move> {
        let mut best_move = None;
        let mut best_score = -f32::INFINITY;
        for move_ in board.generate_all_moves() {
            let mut board_ = board.clone();
            move_.execute(&mut board_);
            let score = self.eval_board(&board_);
            if score > best_score {
                best_move = Some(move_);
                best_score = score;
            }
        }
        best_move
    }

    pub fn best_move_or_random(&self, board: &Board) -> Move {
        let best_move = self.best_move(board);
        if best_move.is_some() {
            best_move.unwrap()
        } else {
            let moves = board.generate_all_moves();
            moves.choose(&mut rand::thread_rng()).unwrap().clone()
        }
    }
}
