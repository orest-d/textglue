#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate textglue_lib;

use textglue_lib::*;
//use actix_web::{
//    dev, server, multipart, error, http, App, Error, HttpRequest, HttpResponse, FutureResponse, http::ContentEncoding};
use actix_web::{
    dev, error, http, middleware, multipart, server, App, Error, FutureResponse,
    HttpMessage, HttpRequest, HttpResponse, Json
};

use std::collections::{HashMap,HashSet};

use futures::future;
use futures::{Future, Stream};

//use std::cell::Cell;


use std::str::FromStr;
use std::io::prelude::*;
use std::io;
use std::io::Write;
use std::fs::{self,DirEntry,File};
use std::path::{Path,PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRepository{
    snippet_file_postfix:String,
    metadata_file_postfix:String,
    document_file_postfix:String,
    directory:String,
    path:String
}

impl FileRepository{
    pub fn new() -> FileRepository{
        FileRepository{
            snippet_file_postfix:".txt".to_string(),
            metadata_file_postfix:".meta.yaml".to_string(),
            document_file_postfix:".doc.yaml".to_string(),
            directory:"md2".to_string(),
            path:"textglue.json".to_string(),
        }
    }

    pub fn load(&self) -> std::io::Result<Database>{
        let all_files = files(&self.directory);
        let mut db = Database::new();
        assert!(!self.metadata_file_postfix.ends_with(&self.snippet_file_postfix));
        assert!(!self.document_file_postfix.ends_with(&self.snippet_file_postfix));
        for path in all_files{
            let filename = Path::new(&path).file_name().expect("File name expected").to_str().expect("Filename err");
            if filename.ends_with(&self.snippet_file_postfix){
                let id = &filename[..(&filename.len()-self.snippet_file_postfix.len())];
                db.snippets.insert(id.to_string(),fs::read_to_string(&path)?);
            }
            else if filename.ends_with(&self.metadata_file_postfix){
                let id = &filename[..(&filename.len()-self.metadata_file_postfix.len())];
                let yaml = fs::read_to_string(&path)?;
                db.metadata.insert(id.to_string(),serde_yaml::from_str(&yaml).expect("Metadata error"));
            }
            else if filename.ends_with(&self.document_file_postfix){
                let id = &filename[..(&filename.len()-self.document_file_postfix.len())];
                let yaml = fs::read_to_string(&path)?;
                    db.documents.insert(id.to_string(),serde_yaml::from_str(&yaml).expect("Document error"));
            }
        }
        Ok(db)
    }

    pub fn save(&self,db:&Database) -> std::io::Result<()>{
        self.save_to_directory(db, &self.directory)
    }

    pub fn save_json(&self,json:&str) {
        let db:Database = serde_json::from_str(json).unwrap();
        self.save(&db).unwrap();
    }

    pub fn save_to_directory(&self,db:&Database,directory:&str) -> std::io::Result<()>{
        let dirpath = Path::new(directory);
        fs::create_dir_all(dirpath)?;
        for (key,value) in db.snippets.iter(){
            let path = dirpath.join(format!("{}{}",key,self.snippet_file_postfix));
            let mut file = File::create(&path)?;
            file.write_all(value.as_bytes())?;
        }
        for (key,value) in db.metadata.iter(){
            let path = dirpath.join(format!("{}{}",key,self.metadata_file_postfix));
            let mut file = File::create(&path)?;
            file.write_all(serde_yaml::to_string(&value).expect("Yaml error").as_bytes())?;
        }
        for (key,value) in db.documents.iter(){
            let path = dirpath.join(format!("{}{}",key,self.document_file_postfix));
            let mut file = File::create(&path)?;
            file.write_all(serde_yaml::to_string(&value).expect("Yaml error").as_bytes())?;
        }
        Ok(())
    }

    pub fn remove_unused(&self,db:&Database) -> std::io::Result<()>{
        let all_files = files(&self.directory);
        let keys:Vec<String> = db.keys().collect();
        for path in all_files{
            let filename = Path::new(&path).file_name().expect("File name expected").to_str().expect("Filename err");
            if filename.ends_with(&self.snippet_file_postfix){
                let id = &filename[..(&filename.len()-self.snippet_file_postfix.len())];
                if !keys.contains(&id.to_string()){
                    fs::remove_file(path)?;
                }
            }
            else if filename.ends_with(&self.metadata_file_postfix){
                let id = &filename[..(&filename.len()-self.metadata_file_postfix.len())];
                if !keys.contains(&id.to_string()){
                    fs::remove_file(path)?;
                }
            }
            else if filename.ends_with(&self.document_file_postfix){
                let id = &filename[..(&filename.len()-self.document_file_postfix.len())];
                if !db.documents.contains_key(&id.to_string()){
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }
}

fn index<T>(_req: &HttpRequest<T>) -> &'static str {
    "Hello world!"
}


pub fn files(dir: &str) -> Vec<String>{
    fs::read_dir(dir)
    .expect("Directory unaccessible")
    .filter_map(|x| {
        x.map(
            |entry| {
                let path = entry.path();
                if path.is_dir(){
                    None
                }
                else{
                    path.to_str().map(|x| x.to_string())
                }
            }
        ).ok()
    }).flatten().collect()
}

pub fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = Vec<u8>, Error = Error>> {
    
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(field
                .map_err(error::ErrorInternalServerError)
                .concat()
                .map(|bytes| bytes.to_vec()).into_stream()
            )
        },
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten(),
        ),
    }
}

pub fn upload<T>(req: HttpRequest<T>) -> FutureResponse<HttpResponse> {
    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(|content| {
                let b = content.concat();
                if let Ok(s) = String::from_utf8(b){
                    fs::write("uploaded.json",&s).unwrap();
                    FileRepository::new().save_json(&s);
                    HttpResponse::Ok().body("Saved")
                }
                else{
                    HttpResponse::InternalServerError().reason("UTF8 decoding error").body("UTF8 decoding error")
                }
            }
        )
    )
}

fn upload_json(db:Json<Database>) -> HttpResponse{
    FileRepository::new().save(&db);

    HttpResponse::Ok()
            .content_type("text/plain")
            .body("ok")    
}
fn main() {
//    println!("Hello, world!");
//    let directory = files(".");
//    for (i,p) in directory.iter().enumerate(){
//        println!("{} {:?}",i,p);
//    }
    let mut db:Database = FileRepository::new().load().unwrap();
    db.fill().remove_undefined_snippets_from_documents();
    db.document().add_chapter("Introduction").add_snippet("Intro");
    FileRepository::new().save_to_directory(&db, "md1");
    
    let s = fs::read_to_string("dbnow.json").unwrap();
    let db:Database = serde_json::from_str(&s).unwrap();
//    println!("{}",serde_json::to_string_pretty(&db).unwrap());
    
    server::new(|| App::with_state(FileRepository::new().load().unwrap())
        .resource("/", |r| r.f(index))
        .resource("/api/db.json", |r| r.f(|r| {
            HttpResponse::Ok()
            .content_type("application/json")
            .body(r.state().to_json().unwrap())
        }))
        .resource("/api/dbnow.json", |r| r.f(|r| {
            HttpResponse::Ok()
            .content_type("application/json")
            .body(FileRepository::new().load().unwrap().to_json().unwrap())
        }))
        .resource("/index.html", |r| r.f(|r| {
            const content: &'static [u8] = include_bytes!("../../textglue-wasm/www/app.html");
            HttpResponse::Ok()
            .content_type("text/html")
            .body(content)
        }))
        .resource("/textglue_wasm.js", |r| r.f(|r| {
            const content: &'static [u8] = include_bytes!("../../textglue-wasm/pkg/textglue_wasm.js");
            HttpResponse::Ok()
            .content_type("application/javascript")
            .body(content)
        }))
        .resource("/textglue_wasm_bg.wasm", |r| r.f(|r| {
            const content: &'static [u8] = include_bytes!("../../textglue-wasm/pkg/textglue_wasm_bg.wasm");
            HttpResponse::Ok()
            .content_type("application/wasm")
            .body(content)
        }))
        .resource("/api/upload", |r| {
                r.method(http::Method::GET).f(|r| {"Upload"});
                r.method(http::Method::POST).with(upload);
        })
        .resource("/api/upload-json", |r|
                r.method(http::Method::POST).with(upload_json)
        )

    )
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
