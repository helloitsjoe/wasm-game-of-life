mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead= 0,
    Alive= 1,
}

pub struct Universe {
    width: u32,
    _height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: u32, _height: u32) -> Universe {
        Universe {
            width,
            _height,
            cells: vec![Cell::Dead; (width * _height) as usize],
        }
    }

    pub fn set_live(&mut self, col: u32, row: u32) {
        let idx =self.get_index(col, row);
        self.cells[idx] = Cell::Alive;
    }

    pub fn get_live(&self, col: u32, row: u32) -> Cell {
        self.cells[self.get_index(col, row)]
    }

    fn get_index(&self, col: u32, row: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn check_index(&self, idx: i32) -> bool {
        if idx < 0 {
            false
        } else {
            self.cells.get(idx as usize) == Some(&Cell::Alive)
        }
    }


    pub fn live_neighbor_count(&self, col: u32, row: u32) -> u8 {
        let home_idx = self.get_index(col, row) as i32;
        let mut count = 0;
        let width = self.width as i32;

        // log(&format!("{:?}", self.cells));

        // Naive solution, see https://rustwasm.github.io/docs/book/game-of-life/implementing.html
        // for a better solution
        let neighbors = vec![
            home_idx - 1,
            home_idx + 1,
            home_idx + width,
            home_idx - width,
            home_idx - 1 - width,
            home_idx + 1 - width,
            home_idx - 1 + width,
            home_idx + 1 + width,
        ];

        for idx in neighbors {
            if self.check_index(idx) {
                count += 1;
            }
        }

        count
    }
}
