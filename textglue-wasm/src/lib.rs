#[macro_use]
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate textglue_lib;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate web_sys;


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
pub fn set_database(new_db:&JsValue) -> String{
    match new_db.into_serde::<Database>(){
       Ok(mut db) => {db.tidy(); *DB.lock().unwrap() = db; "OK".to_string()},
       Err(e) => format!("Error in set_database: {}",e)
    }
}

#[wasm_bindgen]
pub fn set_database_json(json:&str) -> String {
    match Database::from_json(json){
       Ok(mut db) => {db.tidy(); *DB.lock().unwrap() = db; "OK".to_string()},
       Err(e) => format!("Error in set_database_json: {}",e)
    }
}

#[wasm_bindgen]
pub fn get_database() -> JsValue {
    let db:&Database = &*DB.lock().unwrap();

    JsValue::from_serde(db).unwrap()
}

#[wasm_bindgen]
pub fn get_database_json() -> String {
    let db:&Database = &*DB.lock().unwrap();
    db.to_json().unwrap()
}

#[wasm_bindgen]
pub fn get_database_pretty_json() -> String {
    let db:&Database = &*DB.lock().unwrap();
    db.to_pretty_json().unwrap()
}

#[wasm_bindgen]
pub fn get_snippet(id:&str) -> Option<String> {
    let db:&Database = &*DB.lock().unwrap();
    db.snippets.get(id).map(|x| x.to_string())
}

#[wasm_bindgen]
pub fn contains_snippet(id:&str) -> bool {
    let db:&Database = &*DB.lock().unwrap();
    db.snippets.contains_key(id)
}

#[wasm_bindgen]
pub fn set_snippet(id:&str,text:&str) {
    let db:&mut Database = &mut *DB.lock().unwrap();
    db.snippets.insert(id.to_string(),text.to_string());
    if !db.metadata.contains_key(id){
        db.metadata.insert(id.to_string(),Metadata::new(id));
    }
}

#[wasm_bindgen]
pub fn snippet_ids(id:&str) -> JsValue{
    let db:&Database = &*DB.lock().unwrap();
    JsValue::from_serde(&db.snippets.keys().collect::<Vec<&String>>()).unwrap()
}

#[wasm_bindgen]
pub fn get_metadata() -> JsValue {
    let db:&Database = &*DB.lock().unwrap();
    JsValue::from_serde(&db.metadata).unwrap()
}

#[wasm_bindgen]
pub fn set_metadata(id:&str, metadata:&JsValue) -> String{
    match metadata.into_serde::<Metadata>(){
       Ok(m) => {
           let db:&mut Database = &mut *DB.lock().unwrap();
           db.metadata.insert(id.to_string(),m);
           "OK".to_string()
       },
       Err(e) => format!("Error in set_metadata(id={}): {}",id,e)
    }
}

#[wasm_bindgen]
pub fn get_documents() -> JsValue {
    let db:&Database = &*DB.lock().unwrap();
    JsValue::from_serde(&db.documents).unwrap()
}

#[wasm_bindgen]
pub fn new_document_autoid(name:&str) -> JsValue {
    let db:&mut Database = &mut *DB.lock().unwrap();
    JsValue::from_serde(&db.new_document_autoid(name)).unwrap()
}

#[wasm_bindgen]
pub fn get_document(name:&str) -> JsValue {
    let db:&mut Database = &mut *DB.lock().unwrap();
    JsValue::from_serde(&db.get_document(name)).unwrap()
}

#[wasm_bindgen]
pub fn new_snippet(id:&str) -> String {
    let db:&mut Database = &mut *DB.lock().unwrap();
    db.new_snippet(id)
}

#[wasm_bindgen]
pub fn add_chapter_autoname(document:&str) -> JsValue {
    let db:&mut Database = &mut *DB.lock().unwrap();
    JsValue::from_serde(&db.get_document_mut(document).add_chapter_autoname()).unwrap()
}

#[wasm_bindgen]
pub fn get_chapter(document:&str,i:usize) -> JsValue {
    web_sys::console::log_1(&JsValue::from_str(&format!("get_chapter A {} {}",document,i)));
    let db:&mut Database = &mut *DB.lock().unwrap();

    if let Some(chapter)=db.get_document_mut(document).chapters.get(i){
        JsValue::from_serde(chapter).unwrap()
    }
    else{
        JsValue::null()
    }
}

#[wasm_bindgen]
pub fn set_chapter(document:&str,i:usize,chapter:JsValue) -> String {
    let db:&mut Database = &mut *DB.lock().unwrap();
    match chapter.into_serde::<Chapter>(){
       Ok(ch) => {db.get_document_mut(document).chapters[i]=ch; "OK".to_string()},
       Err(e) => format!("Error in set_chapter: {}",e)
    }
}

#[wasm_bindgen]
pub fn get_chapter_text(document:&str,i:usize, id_prefix:&str, id_postfix:&str) -> String {
    let db:&mut Database = &mut *DB.lock().unwrap();
    db.tidy().get_chapter_text(document,i,id_prefix,id_postfix)
}

#[wasm_bindgen]
pub fn set_chapter_text(document:&str,i:usize, id_prefix:&str, id_postfix:&str, text:&str) {
    let db:&mut Database = &mut *DB.lock().unwrap();
    db.tidy().set_chapter_text(document,i,id_prefix,id_postfix,text);
}
