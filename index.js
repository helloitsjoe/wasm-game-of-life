const button = document.createElement('button');
button.innerText = 'Click me';
document.body.appendChild(button);

import('./crate/pkg/wasm-game-of-life').then(m => {
  button.onclick = () => {
    m.greet('from WASM!');
  };
});
