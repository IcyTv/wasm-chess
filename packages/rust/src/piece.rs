use bitflags::bitflags;
use wasm_bindgen::prelude::*;

use std::fmt;

bitflags! {
    pub struct Piece: u8 {
        const NONE = 0;

        const WHITE = 0b0000_0001;
        const BLACK = 0b0000_0010;

        const PAWN = 0b0000_0100;
        const KNIGHT = 0b0000_1000;
        const BISHOP = 0b0001_0000;
        const ROOK = 0b0010_0000;
        const QUEEN = 0b0100_0000;
        const KING = 0b1000_0000;

        const PIECE = 0b1111_1100;
        const COLOR = 0b0000_0011;
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut out = '.';
        if self.contains(Piece::PAWN) {
            out = 'P';
        } else if self.contains(Piece::KNIGHT) {
            out = 'N';
        } else if self.contains(Piece::BISHOP) {
            out = 'B';
        } else if self.contains(Piece::ROOK) {
            out = 'R';
        } else if self.contains(Piece::QUEEN) {
            out = 'Q';
        } else if self.contains(Piece::KING) {
            out = 'K';
        }

        if self.contains(Piece::BLACK) {
            out = out.to_ascii_lowercase();
        }

        write!(f, "{}", out)
    }
}

impl Piece {
    pub fn opposite(&self) -> Piece {
        let mut out = *self;
        out.toggle(Piece::WHITE);
        out.toggle(Piece::BLACK);
        out
    }
    pub fn opposite_color(&self) -> Piece {
        let mut out = *self;
        out.toggle(Piece::WHITE);
        out.toggle(Piece::BLACK);
        out & Piece::COLOR
    }

    pub fn color(&self) -> Piece {
        *self & Piece::COLOR
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        if self.contains(Piece::PAWN) {
            score += 1;
        } else if self.contains(Piece::BISHOP) || self.contains(Piece::KNIGHT) {
            score += 3;
        } else if self.contains(Piece::ROOK) {
            score += 5;
        } else if self.contains(Piece::QUEEN) {
            score += 9;
        } else if self.contains(Piece::KING) {
            score += 200;
        }
        score
    }
}
