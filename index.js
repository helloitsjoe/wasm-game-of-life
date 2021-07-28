import { Universe, Cell } from './crate/pkg/wasm-game-of-life';
import { memory } from './crate/pkg/wasm-game-of-life_bg.wasm';

const CELL_SIZE = 2;
const GRID_SIZE = 150;
const DEAD_COLOR = '#FFF';
const ALIVE_COLOR = '#000';

const width = GRID_SIZE;
const height = GRID_SIZE;

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d', { alpha: false });

canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const universe = Universe.new(width, height);

const getIdx = (col, row) => row * width + col;

const drawCells = ctx => {
  // Access `cells` from rust in memory directly. Neat!
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.fillStyle = DEAD_COLOR;
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  ctx.fillStyle = ALIVE_COLOR;

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const i = getIdx(col, row);

      if (cells[i] === Cell.Alive) {
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  }
};

const render = () => {
  universe.tick();

  drawCells(ctx);

  requestAnimationFrame(render);
};

requestAnimationFrame(render);
