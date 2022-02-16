use crate::{
    board::{Board, Player},
    piece::Piece,
};

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

    fn eval(&self, board: &Board) -> i32 {
        let mut score = 0;
        for pos in board.positions() {
            let piece = board[pos];
            if piece.color() == self.color {
                score += piece.score();
            } else {
                score -= piece.score();
            }
        }

        score
    }
}
