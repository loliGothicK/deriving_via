#![cfg(target_arch = "wasm32")]
use deriving_via::DerivingVia;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
#[derive(DerivingVia, PartialEq, Debug)]
#[deriving(Serialize, Deserialize)]
pub struct WasmStruct(i32);

#[wasm_bindgen_test]
fn test_serde_wasm_bindgen() {
    let original = WasmStruct(42);
    let js_val = serde_wasm_bindgen::to_value(&original).unwrap();
    let deserialized: WasmStruct = serde_wasm_bindgen::from_value(js_val).unwrap();
    assert_eq!(original, deserialized);
}
