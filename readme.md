# MageSim - Vanilla

Mage simulator for WoW vanilla.

[Live sim website](https://cheesehyvel.github.io/magesim-sod/)

Written in Rust and Vuejs

## Requirements

* [Rust toolchain](https://www.rust-lang.org/tools/install)
* wasm-pack - [installer](https://rustwasm.github.io/wasm-pack/installer/) or `cargo install wasm-pack`
* npm

## Development

WebAssembly and front-end are built by separate processes.\ 
Front-end is built with vite, while the webassembly is built with wasm-pack.\ 
Easiest way to start development is to run `npm run dev`. This will start a local webserver, and build and watch for changes in the front-end.\ 
Then to build the webassembly run `npm run wasm`.

To build for production, simply run `npm run prod`.