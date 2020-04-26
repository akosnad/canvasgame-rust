import * as wasm from "canvasgame-rust";

wasm.init();

let canvas = document.getElementById('game-canvas');
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.addEventListener('resize', () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
});

let ctx = canvas.getContext('2d');
function renderLoop() {
    wasm.render_loop(ctx);
    window.requestAnimationFrame(renderLoop);
}

renderLoop();