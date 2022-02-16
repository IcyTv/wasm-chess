use wasm_bindgen::prelude::wasm_bindgen;

use crate::{board::Board, piece::Piece, pos::Position};

#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone)]
pub struct Move {
    pub(crate) start: Position,
    pub(crate) end: Position,
}

#[wasm_bindgen]
impl Move {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        format!("{} -> {}", self.start, self.end)
    }

    #[wasm_bindgen(js_name = "getStart")]
    pub fn get_start(&self) -> Position {
        self.start.clone()
    }

    #[wasm_bindgen(js_name = "getEnd")]
    pub fn get_end(&self) -> Position {
        self.end.clone()
    }

    #[wasm_bindgen(js_name = "do")]
    pub fn do_move(&self, board: &mut Board) {
        self.execute(board);
    }
}

impl Move {
    pub fn new(start: Position, end: Position) -> Move {
        Move { start, end }
    }

    pub fn execute(&self, board: &mut Board) {
        board[&self.end] = board[&self.start];
        board[&self.start] = Piece::NONE;
        board.turn = board.turn.next();
    }
}
