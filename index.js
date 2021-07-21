// wasm import only works without dynamic import because of
// experiments.asyncWebAssembly in webpack.config
import { Universe, Cell } from './crate/pkg/wasm-game-of-life';
import { memory } from './crate/pkg/wasm-game-of-life_bg.wasm';

const CELL_SIZE = 2;
const GRID_COLOR = '#CCC';
const DEAD_COLOR = '#FFF';
const ALIVE_COLOR = '#000';
const WIDTH = 150;
const HEIGHT = 150;

const canvas = document.getElementById('canvas');
canvas.width = (CELL_SIZE + 1) * WIDTH + 1;
canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;

const ctx = canvas.getContext('2d');

const universe = Universe.new(WIDTH, HEIGHT);

const getIdx = (col, row) => row * WIDTH + col;

const drawGrid = ctx => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Verticals
  for (let i = 0; i < WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
  }

  // Horizontals
  for (let j = 0; j < HEIGHT; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = ctx => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, WIDTH * HEIGHT);

  ctx.beginPath();

  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const i = getIdx(col, row);

      ctx.fillStyle = cells[i] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
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

const render = () => {
  universe.tick();

  // drawGrid(ctx);
  drawCells(ctx);

  requestAnimationFrame(render);
};

render();
