use wasm_bindgen::prelude::*;

use std::fmt;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let x = (self.x as u8 + 'A' as u8) as char;
        write!(f, "{}{}", x, self.y + 1)
    }
}

impl From<i32> for Position {
    fn from(i: i32) -> Position {
        let x = (i % 8) as i8;
        let y = (i / 8) as i8;
        Position { x, y }
    }
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i8, y: i8) -> Position {
        Position { x, y }
    }

    #[wasm_bindgen]
    pub fn is_valid(&self) -> bool {
        self.x < 8 && self.y < 8 && self.x >= 0 && self.y >= 0
    }

    // #[wasm_bindgen(getter = x)]
    // pub fn get_x(&self) -> i8 {
    //     self.x
    // }

    // #[wasm_bindgen(setter = x)]
    // pub fn set_x(&mut self, x: i8) {
    //     self.x = x;
    // }

    // #[wasm_bindgen(getter = y)]
    // pub fn get_y(&self) -> i8 {
    //     self.y
    // }

    // #[wasm_bindgen(setter = y)]
    // pub fn set_y(&mut self, y: i8) {
    //     self.y = y;
    // }
}
