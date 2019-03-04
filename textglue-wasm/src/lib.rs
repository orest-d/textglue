#[macro_use]
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate textglue_lib;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use textglue_lib::*;
use std::sync::{Mutex};

lazy_static! {
    static ref DB: Mutex<Database> = Mutex::new(Database::new());
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, textglue-wasm!");
}

#[wasm_bindgen]
pub fn set_database(new_db:&JsValue) {
    let new_db:Database = new_db.into_serde().unwrap();
}


#[wasm_bindgen]
pub fn get_database() -> JsValue {
    let db:&Database = &*DB.lock().unwrap();

    JsValue::from_serde(db).unwrap()
}