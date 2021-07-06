import { load_asset_file, load_asset_index } from "./assets";
window.load_asset_file_inner = load_asset_file;
window.load_asset_index_inner = load_asset_index;

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