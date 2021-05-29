import * as wasm from "canvasgame-rust";


let canvas = document.getElementById('game-canvas');
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.addEventListener('resize', () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
});

window.addEventListener('keydown', e => {
    wasm.key_down(e);
});

window.addEventListener('keyup', e => {
    wasm.key_up(e);
})

 wasm.run();
