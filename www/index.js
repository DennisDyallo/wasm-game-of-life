import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm"; // Import the WebAssembly memory from the wasm-game-of-life package
// Import the Universe class specifically from the wasm-game-of-life package
import { Universe, Cell } from "wasm-game-of-life";

const CELL_SIZE = 5; // Size of each cell in pixels
const GRID_COLOR = "#CCCCCC"; // Color for the grid lines
const DEAD_COLOR = "#FFFFFF"; // Color for dead cells
const ALIVE_COLOR = "#000000"; // Color for alive cells

// Create a new Universe instance using the constructor exposed by wasm-bindgen
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1; // Set the height of the canvas
canvas.width = (CELL_SIZE + 1) * width + 1; // Set the width of the canvas

const ctx = canvas.getContext("2d"); // Get the 2D rendering context for the canvas

// Define the main rendering loop function
const renderLoop = () => {
  // Advance the universe by one tick (generation)
  universe.tick();

  drawGrid(); // Draw the grid lines
  drawCells(); // Draw the cells based on their state

  // Schedule the next frame for smooth animation
  requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column; // Calculate the index in the flat array representation of the universe
}

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);

