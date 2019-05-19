#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate textglue_lib;
extern crate clap;


use clap::{clap_app};
use textglue_lib::*;
use actix_web::{
    dev, error, http, multipart, server, App, Error, FutureResponse,
    HttpMessage, HttpRequest, HttpResponse, Json
};

use actix_web::fs::StaticFiles;

use futures::{Future, Stream};
use std::io::Write;
use std::fs::{self, File};
use std::path::{Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileRepository{
    snippet_file_postfix:String,
    metadata_file_postfix:String,
    document_file_postfix:String,
    directory:String
}

impl FileRepository{
    pub fn new() -> FileRepository{
        FileRepository{
            snippet_file_postfix:".txt".to_string(),
            metadata_file_postfix:".meta.yaml".to_string(),
            document_file_postfix:".doc.yaml".to_string(),
            directory:".".to_string()        }
    }

    pub fn new_with_directory(dir:&str) -> FileRepository{
        FileRepository{
            snippet_file_postfix:".txt".to_string(),
            metadata_file_postfix:".meta.yaml".to_string(),
            document_file_postfix:".doc.yaml".to_string(),
            directory:dir.to_string()        }
    }

    pub fn load(&self) -> textglue_lib::Result<Database>{
        let all_files = files(&self.directory);
        let mut db = Database::new();
        assert!(!self.metadata_file_postfix.ends_with(&self.snippet_file_postfix));
        assert!(!self.document_file_postfix.ends_with(&self.snippet_file_postfix));
        for path in all_files{
            let filename = Path::new(&path)
                .file_name()
                .ok_or(generic_error(&format!("Path error (F): {}",path)))?
                .to_str().ok_or(generic_error(&format!("Path error (S): {}",path)))?;
            if filename.ends_with(&self.snippet_file_postfix){
                let id = &filename[..(&filename.len()-self.snippet_file_postfix.len())];
                db.snippets.insert(id.to_string(),fs::read_to_string(&path)?);
            }
            else if filename.ends_with(&self.metadata_file_postfix){
                let id = &filename[..(&filename.len()-self.metadata_file_postfix.len())];
                let yaml = fs::read_to_string(&path)?;
                db.metadata.insert(id.to_string(),serde_yaml::from_str(&yaml)?);
            }
            else if filename.ends_with(&self.document_file_postfix){
                let id = &filename[..(&filename.len()-self.document_file_postfix.len())];
                let yaml = fs::read_to_string(&path)?;
                    db.documents.insert(id.to_string(),serde_yaml::from_str(&yaml)?);
            }
        }
        Ok(db)
    }

    pub fn load_json(&self) -> textglue_lib::Result<String>{
        Ok(serde_json::to_string(&self.load()?)?)
    }

    pub fn save(&self,db:&Database) -> textglue_lib::Result<()>{
        self.save_to_directory(db, &self.directory)
    }

    pub fn save_json(&self,json:&str) -> textglue_lib::Result<()>{
        let db:Database = serde_json::from_str(json)?;
        self.save(&db)?;
        Ok(())
    }

    pub fn save_to_directory(&self,db:&Database,directory:&str) -> textglue_lib::Result<()>{
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
            file.write_all(serde_yaml::to_string(&value)?.as_bytes())?;
        }
        for (key,value) in db.documents.iter(){
            let path = dirpath.join(format!("{}{}",key,self.document_file_postfix));
            let mut file = File::create(&path)?;
            file.write_all(serde_yaml::to_string(&value)?.as_bytes())?;
        }
        Ok(())
    }

    pub fn remove_unused(&self,db:&Database) -> textglue_lib::Result<()>{
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
    "<head><meta http-equiv=\"refresh\" content=\"0; URL=/textglue/index.html\"/></head>"
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
                .concat2()
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

pub fn upload(req: HttpRequest<FileRepository>) -> FutureResponse<HttpResponse> {
    println!("Upload json");
    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(move |content| {
                let b = content.concat();
                if let Ok(s) = String::from_utf8(b){
                    fs::write("uploaded.json",&s).unwrap();
                    req.state().save_json(&s);
                    HttpResponse::Ok().body("Saved")
                }
                else{                    
                    println!("Upload json - ERR");
                    HttpResponse::InternalServerError().reason("UTF8 decoding error").body("UTF8 decoding error")
                }
            }
        )
    )
}

fn upload_json(db:Json<Database>) -> HttpResponse{
    match FileRepository::new().save(&db){
        Ok(()) =>{
            HttpResponse::Ok()
            .content_type("application/json")
            .body(&format!("{{\"status\":\"OK\", message:\"OK\"}}"))
        },
        Err(e) => {
            HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(&format!("{{\"status\":\"Error\", message:\"{}\"}}",e))
        }
    }

}

fn serve_database(request:&HttpRequest<FileRepository>) -> HttpResponse{
    let repo:&FileRepository = request.state();
    match repo.load_json(){
        Ok(json) => {
            HttpResponse::Ok()
            .content_type("application/json")
            .body(json)
        },
        Err(e) =>{
            HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(&format!("{{\"status\":\"Error\", message:\"{}\"}}",e))
        }
    }
}

fn textglue_resources(info: actix_web::Path<(String,)>) -> HttpResponse{
    println!("TEXTGLUE {}",info.0);
    /*
    match info.0.as_str(){
       "abc" => HttpResponse::Ok()
            .content_type("text/plain")
            .body("ABC".to_string()),
        _ => HttpResponse::Ok()
        .content_type("text/plain")
        .body(info.0.to_string())

    }*/
    include!("../../textglue-app/resources.rs")
}

fn main() {
    let matches = clap::clap_app!(myapp =>
        (version: "1.0")
        (author: "Orest Dubay <orest3.dubay@gmail.com>")
        (about: "Repository: https://github.com/orest-d/textglue")
        (@arg path: -p --path +takes_value "Path to the text repository")
        (@subcommand ui =>
            (about: "Start TextGlue UI server")
            (@arg port: -p --port +takes_value "Port number")
        )
        (@subcommand mv =>
            (about: "move old_snippet.txt new_snippet.txt")
            (@arg src: +takes_value "source")
            (@arg dst: +takes_value "destination")
        )
        (@subcommand archive =>
            (about: "archive text repository to a json file")
            (@arg dst: +takes_value "destination")
        )
        (@subcommand unarchive =>
            (about: "reconstruct text repository from a json archive")
            (@arg src: +takes_value "source")
        )
    ).get_matches();    
    let file_repo = FileRepository::new_with_directory(matches.value_of("path").unwrap_or("."));
    println!("TextGlue text repository at {}",file_repo.directory);
    if let Some(uimatches) = matches.subcommand_matches("mv") {
        let src = uimatches.value_of("src").unwrap();
        let dst = uimatches.value_of("dst").unwrap();
        println!("TextGlue move {} {}",src,dst);
    }
    if let Some(uimatches) = matches.subcommand_matches("archive") {
        let dst = uimatches.value_of("dst").unwrap();
        println!("TextGlue archive {}",dst);
        let db = file_repo.load().expect("Failed to load text repository").to_json().expect("Failed to create json archive");
        let mut file = File::create(dst).expect(&format!("Failed to create archive file {}",dst));
        file.write_all(db.as_bytes()).expect(&format!("Failed to write archive file {}",dst));
    }
    if let Some(uimatches) = matches.subcommand_matches("ui") {
        let port = uimatches.value_of("port").unwrap_or("5000");
        println!("Run TextGlue server on port {}",port);
        server::new(move || App::with_state(file_repo.clone())
            //.resource("/", |r| r.f(index))
            .resource("/api/db.json", |r| r.f(serve_database))
            .resource("/api/dbnow.json", |r| r.f(|_r| {
                HttpResponse::Ok()
                .content_type("application/json")
                .body(FileRepository::new().load().unwrap().to_json().unwrap())
            }))
            .resource("/app-static.html", |r| r.f(|_r| {
                const CONTENT: &'static [u8] = include_bytes!("../../textglue-wasm/www/app.html");
                HttpResponse::Ok()
                .content_type("text/html")
                .body(CONTENT)
            }))
            .resource("/app.html", |r| r.f(|_r| {
                let content = fs::read_to_string("/home/orest/zlos/rust/textglue/textglue-wasm/www/app.html").expect("Read error");
                HttpResponse::Ok()
                .content_type("text/html")
                .body(content)
            }))
            .resource("/textglue_wasm.js", |r| r.f(|_r| {
                const CONTENT: &'static [u8] = include_bytes!("../../textglue-wasm/pkg/textglue_wasm.js");
                HttpResponse::Ok()
                .content_type("application/javascript")
                .body(CONTENT)
            }))
            .resource("/textglue_wasm_bg.wasm", |r| r.f(|_r| {
                const CONTENT: &'static [u8] = include_bytes!("../../textglue-wasm/pkg/textglue_wasm_bg.wasm");
                HttpResponse::Ok()
                .content_type("application/wasm")
                .body(CONTENT)
            }))
            .resource("/api/upload", |r| {
                    r.method(http::Method::GET).f(|_r| {"Upload"});
                    r.method(http::Method::POST).with(upload);
            })
            .resource("/api/upload-json", |r|
                    r.method(http::Method::POST).with(upload_json)
            )
            .route("/textglue/{path:.*}", http::Method::GET, textglue_resources)
            .handler(
                "/js",
                StaticFiles::new("/home/orest/zlos/rust/textglue/textglue-app/dist/js")
                    .unwrap()
                    .show_files_listing())
            .handler(
                "/css",
                StaticFiles::new("/home/orest/zlos/rust/textglue/textglue-app/dist/css")
                    .unwrap()
                    .show_files_listing())
            .resource("/index.html", |r| r.f(|_r| {
                let content = "<head><meta http-equiv=\"refresh\" content=\"0; URL=/textglue/index.html\"/></head>";
                HttpResponse::Ok()
                .content_type("text/html")
                .body(content)
            }))
            .resource("/", |r| r.f(|_r| {
                let content = "<head><meta http-equiv=\"refresh\" content=\"0; URL=/textglue/index.html\"/></head>";
                HttpResponse::Ok()
                .content_type("text/html")
                .body(content)
            }))
            .resource("/logo.png", |r| r.f(|_r| {
                const CONTENT: &'static [u8] = include_bytes!("../../textglue-app/public/logo.png");
                HttpResponse::Ok()
                .content_type("image/png")
                .body(CONTENT)
            }))                
        )
        .bind(&format!("127.0.0.1:{}",port))
        .unwrap()
        .run();
    }
}
