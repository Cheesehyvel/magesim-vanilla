# MageSim - Vanilla

Mage simulator for WoW vanilla.

[Live sim website](https://cheesehyvel.github.io/magesim-vanilla/)

Written in Rust and Vuejs

## Requirements

* [Rust toolchain](https://www.rust-lang.org/tools/install)
* wasm-pack - [installer](https://rustwasm.github.io/wasm-pack/installer/) or `cargo install wasm-pack`
* npm

## Setup
* Install wasm dependencies with `wasm-pack build`
* Install front-end dependencies with `npm install`
* Build project with `npm run prod`

## Development
* Build wasm only: `npm run wasm`
* Build front-end only: `npm run build`

You can make a local development server with `npm run dev`. This will start a local webserver and rebuild the front-end when changes occur.  
For more information about the build process, see [wasm-pack](https://rustwasm.github.io/wasm-pack/book/) and [vite](https://vite.dev/guide/) documentation.