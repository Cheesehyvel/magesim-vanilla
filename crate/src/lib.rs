#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod common;
mod stats;
mod macros;
mod config;
mod item;
mod spell;
mod aura;
mod cooldown;
mod log;
mod target;
mod unit;
mod apl;
mod mage;
mod event;
mod sim;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use gloo_utils::format::JsValueSerdeExt;


/**
 * Constants
 */
const MANA_TICK_T: f64 = 2.0;

/**
 * Entry points from javascript
 * - Convert configuration from json
 * - Run simulation
 * - Convert result to json
 */

#[wasm_bindgen]
pub fn run_simulation(cfg: JsValue) -> JsValue {
    common::set_panic_hook();

    let config = cfg.into_serde().unwrap();
    let result = sim::run_single(config);

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn run_simulations(cfg: JsValue, iterations: i32) -> JsValue {
    common::set_panic_hook();

    let config = cfg.into_serde().unwrap();
    let result = sim::run_multiple(config, iterations);

    serde_wasm_bindgen::to_value(&result).unwrap()
}