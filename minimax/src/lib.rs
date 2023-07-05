mod minimax;
mod utils;
use minimax::{Board, BoardOps, Minimax, AI};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn minimax(val: JsValue) -> JsValue {
    let example: Vec<Vec<isize>> = serde_wasm_bindgen::from_value(val).unwrap();

    let mut board: Board = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            board[i][j] = example[i][j] as i8;
        }
    }

    let Minimax { moves, .. } = &board.optimal_move(board.depth(), AI);

    let moves = [moves.x.unwrap(), moves.y.unwrap()];

    serde_wasm_bindgen::to_value(&moves).unwrap()
}
