// wasm import only works without dynamic import because of
// experiments.asyncWebAssembly in webpack.config
import * as m from './crate/pkg/wasm-game-of-life';

const universe = m.Universe.new(100, 100);

const canvas = document.getElementById('code');

const render = () => {
  canvas.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(render);
};

render();
