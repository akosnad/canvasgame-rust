import * as wasm from "canvasgame-rust";
import { memory } from "../pkg/canvasgame_rust_bg.wasm";

async function load(path) {
    const resp = await fetch(path);
    const buf = await resp.arrayBuffer();

    const len = buf.byteLength;
    const ptr = wasm.alloc(len);

    const array = new Uint8Array(memory.buffer, ptr, len);
    array.set(new Uint8Array(buf));

    return {ptr, len};
}

export async function load_asset_index() {
    let index = await import("../assets/index.json");
    console.log("Passing asset index to wasm:", index.default);

    let enc = new TextEncoder();
    const data = enc.encode(JSON.stringify(index.default));
    const buf = data.buffer;

    const len = buf.byteLength;
    const ptr = wasm.alloc(len);

    const array = new Uint8Array(memory.buffer, ptr, len);
    array.set(new Uint8Array(buf));

    return {ptr, len};
}

export async function load_asset_file(ptr, len) {
    const buf = new Uint8Array(memory.buffer, ptr, len);
    let dec = new TextDecoder();
    const path = dec.decode(buf);

    return await load(path);
}