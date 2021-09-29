import { load_asset_file, load_asset_index } from "./assets";
window.load_asset_file_inner = load_asset_file;
window.load_asset_index_inner = load_asset_index;

import * as wasm from "canvasgame-rust";


let canvas = document.getElementById('game-canvas');
{ 
    let scale = window.devicePixelRatio;
    canvas.width = Math.floor(window.innerWidth * scale);
    canvas.height = Math.floor(window.innerHeight * scale);
}

window.addEventListener('resize', () => {
    let scale = window.devicePixelRatio;
    canvas.width = Math.floor(window.innerWidth * scale);
    canvas.height = Math.floor(window.innerHeight * scale);
});

window.addEventListener('keydown', e => {
    wasm.key_down(e);
});

window.addEventListener('keyup', e => {
    wasm.key_up(e);
})

wasm.run();