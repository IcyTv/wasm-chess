#![allow(clippy::unused_unit)]
#![allow(clippy::inherent_to_string)]

mod ai;
mod board;
mod moves;
mod piece;
mod pos;
mod utils;

use pos::Position;
use wasm_bindgen::prelude::*;

use piece::*;

use crate::board::Board;

extern crate js_sys;
extern crate web_sys;
extern crate wee_alloc;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: String);
}

#[wasm_bindgen]
pub fn setup_panic() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen(js_name = "printPiece")]
pub fn print_piece(piece: u32) -> String {
    let piece = Piece::from_bits(piece as u8).unwrap();
    web_sys::console::log_1(&format!("{:?}", piece).into());
    format!("{}", piece)
}

#[wasm_bindgen(js_name = "printBoard")]
pub fn print_board(board: &Board) {
    log!("{}", board);
}

#[wasm_bindgen(js_name = "getBoard")]
pub fn get_board() -> board::Board {
    board::Board::default()
}

#[wasm_bindgen(js_name = "getMoves")]
pub fn get_moves(pos: Position) {
    let board = board::Board::default();
    let moves = board.generate_moves_for(pos);
    log!("{:#?}", moves);
}
