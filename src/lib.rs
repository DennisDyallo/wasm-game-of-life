// Declares a module named 'utils' - likely contains helper functions
mod utils;

// Imports all public items from wasm_bindgen::prelude for WebAssembly bindings
use wasm_bindgen::prelude::*;

// Marks this enum as exportable to JavaScript via WebAssembly
#[wasm_bindgen]
// Specifies the underlying representation as u8 (8-bit unsigned integer)
#[repr(u8)]
// Derives common traits: Clone (copy semantics), Copy (bitwise copy), Debug (formatting), PartialEq/Eq (equality comparison)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,  // Dead cell represented as 0
    Alive = 1, // Alive cell represented as 1
}

// Marks this struct as exportable to JavaScript
#[wasm_bindgen]
pub struct Universe {
    width: u32,       // Grid width in cells
    height: u32,      // Grid height in cells
    cells: Vec<Cell>, // 1D vector storing all cells (flattened 2D grid)
}

// Implementation block for Universe
#[wasm_bindgen]
impl Universe {
    // Converts 2D coordinates to 1D vector index
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize // Row-major order indexing
    }

    // Counts living neighbors for Conway's Game of Life rules
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0; // Initialize neighbor counter

        // Iterate through 3x3 grid around target cell (using wrapping arithmetic)
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // Skip the center cell (the cell we're counting neighbors for)
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // Calculate neighbor coordinates with wrapping (toroidal topology)
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                // Get the 1D index for this neighbor
                let idx = self.get_index(neighbor_row, neighbor_col);

                // Add 1 if alive (Cell::Alive = 1), 0 if dead (Cell::Dead = 0)
                count += self.cells[idx] as u8;
            }
        }
        count // Return total living neighbors
    }

    // Advances the universe by one tick (generation) according to the Game of Life rules
    pub fn tick(&mut self) {
        // Create a copy of the current cells to store the next state
        let mut next = self.cells.clone();

        // Iterate over each row in the universe
        for row in 0..self.height {
            // Iterate over each column in the universe
            for col in 0..self.width {
                // Calculate the 1D index for the current cell
                let idx = self.get_index(row, col);
                // Get the current state of the cell (Alive or Dead)
                let cell = self.cells[idx];
                // Count the number of live neighbors around the current cell
                let live_neighbors = self.live_neighbor_count(row, col);

                // Determine the next state of the cell based on the Game of Life rules
                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours dies (underpopulation)
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours lives on
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live neighbours dies (overpopulation)
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours becomes alive (reproduction)
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state
                    (otherwise, _) => otherwise,
                };

                // Update the next state for this cell
                next[idx] = next_cell;
            }
        }

        // Replace the current cells with the next generation
        self.cells = next;
    }
}

// Import the formatting traits from the standard library
use std::fmt;

// Implements the Display trait for Universe, allowing it to be printed as a string.
// This is useful for debugging or logging the universe's state in a human-readable format.
impl fmt::Display for Universe {
    // The fmt method defines how the Universe should be formatted.
    // It writes each row of cells as a line of symbols: '◻' for dead, '◼' for alive.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `as_slice()` returns a slice reference to the underlying vector of cells,
        // allowing us to work with the data as a contiguous sequence.
        // `chunks(self.width as usize)` then splits this slice into sub-slices (rows),
        // each of length equal to the universe's width.
        for line in self.cells.as_slice().chunks(self.width as usize) {
            // Iterate over each cell in the current row.
            for &cell in line {
                // Choose a symbol based on whether the cell is dead or alive.
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                // Write the symbol to the formatter.
                write!(f, "{}", symbol)?;
            }
            // Write a newline character after each row.
            write!(f, "\n")?;
        }

        // Indicate that formatting was successful.
        Ok(())
    }
}

#[wasm_bindgen] // Makes the following impl block accessible from JavaScript via wasm-bindgen
impl Universe {
    // ...

    // Public function to create a new Universe instance
    pub fn new() -> Universe {
        let width = 64; // Set the width of the universe grid to 64 cells
        let height = 64; // Set the height of the universe grid to 64 cells

        // Create a vector of cells for the universe
        // For each cell index from 0 to width*height - 1:
        let cells = (0..width * height)
            .map(|i| {
                // If the index is divisible by 2 or 7, make the cell alive
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    // Otherwise, make the cell dead
                    Cell::Dead
                }
            })
            .collect(); // Collect the results into a vector

        // Return a new Universe struct with the specified width, height, and cells
        Universe {
            width,
            height,
            cells,
        }
    }

    // Public function to render the universe as a string
    pub fn render(&self) -> String {
        self.to_string() // Use the Display trait to convert the universe to a string
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
