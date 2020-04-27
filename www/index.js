import * as wasm from "canvasgame-rust";


let canvas = document.getElementById('game-canvas');
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.addEventListener('resize', () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
});


let ctx = canvas.getContext('2d');
wasm.init(ctx);
