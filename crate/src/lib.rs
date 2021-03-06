mod utils;

extern crate console_error_panic_hook;

// use std::panic;
use std::fmt;
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
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    // I would rather pass in `cells` but have to figure out wasm_bindgen error
    // pub fn new(width: u32, height: u32, cells: Option<Vec<Cell>>) -> Universe {
    pub fn new(width: u32, height: u32, empty_cells: Option<bool>) -> Universe {
        console_error_panic_hook::set_once();

        let cells = (0..width * height)
            .map(|i| {
                if empty_cells.unwrap_or(false) {
                    Cell::Dead
                } else if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn set_live(&mut self, col: u32, row: u32) {
        let idx = self.get_index(col, row);
        self.cells[idx] = Cell::Alive;
    }

    pub fn get_live(&self, col: u32, row: u32) -> Cell {
        self.cells[self.get_index(col, row)]
    }

    fn get_index(&self, col: u32, row: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn live_neighbor_count(&self, col: u32, row: u32) -> u8 {
        let home_idx = self.get_index(col, row) as i32;
        let mut count = 0;
        let width = self.width as i32;

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
            if idx > 0 && self.cells.get(idx as usize) == Some(&Cell::Alive) {
                count += 1;
            }
        }

        count
    }

    fn get_next_gen(&self) -> Vec<Cell> {
        let mut cells = self.cells.clone();

        // log(&format!("Before {:?}", cells));

        for col in 0..self.width {
            for row in 0..self.height {
                let idx = self.get_index(col, row);
                // Rules of Tim Conway's Game of Life:
                // 1. Any live cell with fewer than two live neighbours dies.
                // 2. Any live cell with two or three live neighbours lives.
                // 3. Any dead cell with exactly three live neighbours becomes a live cell.
                // 4. Any live cell with more than three live neighbours dies.
                cells[idx] = match self.live_neighbor_count(col, row) {
                    2 => cells[idx],
                    3 => Cell::Alive,
                    _ => Cell::Dead,
                }
            }
        }
        cells
    }

    pub fn tick(&mut self) {
        self.cells = self.get_next_gen();
    }
}
