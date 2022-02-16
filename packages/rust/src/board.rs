use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::moves::Move;
use crate::piece::Piece;
use crate::pos::{self, Position};

use std::fmt;

#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Copy)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn next(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Board {
    pieces: [Piece; 64],
    pub turn: Player,
}

impl Default for Board {
    fn default() -> Self {
        let mut pieces: [Piece; 64] = [Piece::NONE; 64];

        pieces[0] = Piece::ROOK | Piece::WHITE;
        pieces[1] = Piece::KNIGHT | Piece::WHITE;
        pieces[2] = Piece::BISHOP | Piece::WHITE;
        pieces[3] = Piece::QUEEN | Piece::WHITE;
        pieces[4] = Piece::KING | Piece::WHITE;
        pieces[5] = Piece::BISHOP | Piece::WHITE;
        pieces[6] = Piece::KNIGHT | Piece::WHITE;
        pieces[7] = Piece::ROOK | Piece::WHITE;

        for i in 8..16 {
            pieces[i] = Piece::PAWN | Piece::WHITE;
        }

        for i in 48..56 {
            pieces[i] = Piece::PAWN | Piece::BLACK;
        }

        pieces[56] = Piece::ROOK | Piece::BLACK;
        pieces[57] = Piece::KNIGHT | Piece::BLACK;
        pieces[58] = Piece::BISHOP | Piece::BLACK;
        pieces[59] = Piece::QUEEN | Piece::BLACK;
        pieces[60] = Piece::KING | Piece::BLACK;
        pieces[61] = Piece::BISHOP | Piece::BLACK;
        pieces[62] = Piece::KNIGHT | Piece::BLACK;
        pieces[63] = Piece::ROOK | Piece::BLACK;

        Board {
            pieces,
            turn: Player::White,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut out = String::new();

        for i in 0..8 {
            for j in 0..8 {
                out.push_str(&format!("{} ", self.pieces[i * 8 + j]));
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

impl std::ops::Index<Position> for Board {
    type Output = Piece;

    fn index(&self, index: Position) -> &Self::Output {
        &self.pieces[index.y as usize * 8 + index.x as usize]
    }
}

impl std::ops::IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.pieces[index.y as usize * 8 + index.x as usize]
    }
}

impl std::ops::Index<&Position> for Board {
    type Output = Piece;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.pieces[index.y as usize * 8 + index.x as usize]
    }
}

impl std::ops::IndexMut<&Position> for Board {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.pieces[index.y as usize * 8 + index.x as usize]
    }
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(js_name = generateMoves)]
    pub fn js_generate_moves_for(&self, idx: Position) -> js_sys::Array {
        let moves: Vec<Move> = self.generate_moves_for(idx);
        moves.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = asArray)]
    pub fn js_as_array(&self) -> js_sys::Uint8Array {
        let vec: Vec<u8> = self.pieces.into_iter().map(|v| v.bits()).collect();
        let arr = js_sys::Uint8Array::new_with_length(vec.len() as u32);
        arr.copy_from(&vec);
        arr
    }

    #[wasm_bindgen(js_name = get)]
    pub fn js_get(&self, row: i8, col: i8) -> u8 {
        self.pieces[row as usize * 8 + col as usize].bits()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn js_to_string(&self) -> String {
        format!("{}", self)
    }

    #[wasm_bindgen(js_name = isTurnFor)]
    pub fn is_turn_for(&self, row: i8, col: i8) -> bool {
        let col = self.pieces[row as usize * 8 + col as usize].color();
        match self.turn {
            Player::White => col == Piece::WHITE,
            Player::Black => col == Piece::BLACK,
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board::default()
    }

    pub fn generate_all_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.is_turn_for(i as i8 / 8, i as i8 % 8) {
                moves.append(&mut self.generate_moves_for(i.into()));
            }
        }

        moves
    }

    pub fn generate_moves_for(&self, idx: Position) -> Vec<Move> {
        let piece = self[idx.clone()];

        match piece & !Piece::BLACK & !Piece::WHITE {
            Piece::PAWN => self.generate_pawn_moves(&idx, piece),
            Piece::KNIGHT => self.generate_knight_moves(&idx, piece),
            Piece::BISHOP => self.generate_bishop_moves(idx, piece),
            Piece::ROOK => self.generate_rook_moves(idx, piece),
            Piece::QUEEN => self.generate_queen_moves(idx, piece),
            Piece::KING => self.generate_king_moves(idx, piece),
            _ => panic!("Invalid piece {:?}", piece),
        }
    }

    fn generate_pawn_moves(&self, idx: &Position, piece: Piece) -> Vec<Move> {
        const MOVES: [(i8, i8); 4] = [(0, 1), (0, 2), (1, 1), (-1, 1)];

        let mut moves = vec![];

        for &(dx, dy) in &MOVES {
            let end = if piece.contains(Piece::WHITE) {
                Position {
                    x: idx.x + dx,
                    y: idx.y + dy,
                }
            } else {
                Position {
                    x: idx.x - dx,
                    y: idx.y - dy,
                }
            };

            if !end.is_valid() {
                continue;
            }

            if (dy == 1 || dy == 2) && dx.abs() != 1 && self[end.clone()] != Piece::NONE {
                continue;
            }

            if dy == 2
                && ((piece.color() == Piece::WHITE && idx.y != 1)
                    || piece.color() == Piece::BLACK && idx.y != 6)
            {
                continue;
            }

            if dx.abs() == 1 && !self[&end].contains(piece.opposite_color()) {
                continue;
            }

            moves.push(Move {
                start: idx.clone(),
                end: end.clone(),
            });
        }

        moves
    }

    fn generate_knight_moves(&self, idx: &Position, piece: Piece) -> Vec<Move> {
        const MOVES: [(i8, i8); 8] = [
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
            (-1, -2),
            (-2, -1),
        ];

        let mut moves = Vec::<Move>::new();

        for &(x, y) in &MOVES {
            let pos = Position {
                x: idx.x + x,
                y: idx.y + y,
            };

            if pos.is_valid() {
                let end_piece = self[pos.clone()];
                if end_piece == Piece::NONE || end_piece.contains(piece.opposite_color()) {
                    moves.push(Move {
                        start: idx.clone(),
                        end: pos.clone(),
                    });
                }
            }
        }

        moves
    }

    fn generate_bishop_moves(&self, idx: Position, piece: Piece) -> Vec<Move> {
        const MOVES: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        let mut moves = Vec::<Move>::new();

        for &(x, y) in &MOVES {
            let mut pos = Position {
                x: idx.x + x,
                y: idx.y + y,
            };

            while pos.is_valid() {
                let end_piece = self[pos.clone()];
                if end_piece == Piece::NONE {
                    // There is no piece in the way, therefore we can continue to search
                    moves.push(Move {
                        start: idx.clone(),
                        end: pos.clone(),
                    });

                    pos.x += x;
                    pos.y += y;
                } else if end_piece.contains(piece.opposite_color()) {
                    moves.push(Move {
                        start: idx.clone(),
                        end: pos.clone(),
                    });
                    break;
                } else {
                    break;
                }
            }
        }

        moves
    }

    fn generate_rook_moves(&self, idx: Position, piece: Piece) -> Vec<Move> {
        const MOVES: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut moves = Vec::<Move>::new();

        for &(x, y) in &MOVES {
            let mut pos = Position {
                x: idx.x + x,
                y: idx.y + y,
            };

            while pos.is_valid() {
                let end_piece = self[pos.clone()];
                if end_piece == Piece::NONE {
                    // There is no piece in the way, therefore we can continue to search
                    moves.push(Move {
                        start: idx.clone(),
                        end: pos.clone(),
                    });

                    pos.x += x;
                    pos.y += y;
                } else if end_piece.contains(piece.opposite_color()) {
                    moves.push(Move {
                        start: idx.clone(),
                        end: pos.clone(),
                    });
                    break;
                } else {
                    break;
                }
            }
        }

        moves
    }

    fn generate_queen_moves(&self, idx: Position, piece: Piece) -> Vec<Move> {
        let mut moves = self.generate_bishop_moves(idx.clone(), piece);
        moves.extend(self.generate_rook_moves(idx, piece));

        moves
    }

    fn generate_king_moves(&self, idx: Position, piece: Piece) -> Vec<Move> {
        const MOVES: [(i8, i8); 8] = [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
        ];

        let mut moves = Vec::<Move>::new();
        for (x, y) in &MOVES {
            let pos = Position {
                x: idx.x + x,
                y: idx.y + y,
            };

            if pos.is_valid() {
                let end_piece = self[pos.clone()];

                if end_piece == Piece::NONE || end_piece.contains(piece.opposite_color()) {
                    moves.push(Move {
                        start: idx.clone(),
                        end: pos.clone(),
                    });
                }
            }
        }

        moves
    }

    pub fn positions(&self) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();

        for i in 0..64 {
            let pos: Position = i.into();
            if self[pos.clone()] != Piece::NONE {
                positions.push(pos);
            }
        }

        positions
    }

    pub fn is_game_over(&self) -> bool {
        false
    }
}
