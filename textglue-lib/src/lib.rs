#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::collections::{HashMap,HashSet, BTreeSet};

use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct GenericError {
    pub details: String
}

impl GenericError {
    pub fn new(msg: &str) -> GenericError {
        GenericError{details: msg.to_string()}
    }
}

pub fn generic_error(msg: &str) -> GenericError{
    GenericError::new(msg)
}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chapter{
    pub name:String,
    pub snippets:Vec<String>
}

impl Chapter{
    pub fn new(name:&str) -> Chapter{
        Chapter{
            name: name.to_string(),
            snippets:Vec::new()
        }
    }
    pub fn keep_snippets<'a>(&'a mut self, snippets:&Vec<String>) -> &'a mut Chapter{
        self.snippets = self.snippets
            .iter()
            .filter(|snippet| snippets.contains(snippet))
            .map(|snippet| snippet.to_string())
            .collect();
        self
    }
    pub fn add_snippet<'a>(&'a mut self, id:&str)->&'a mut Chapter{
        self.snippets.push(id.to_string());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document{
    pub name:String,
    pub chapters:Vec<Chapter>
}

impl Document{
    pub fn new(name:&str) -> Document{
        Document{
            name:name.to_string(),
            chapters:Vec::new()
        }
    }

    pub fn keep_snippets<'a>(&'a mut self, snippets:&Vec<String>) -> &'a mut Document{
        for chapter in self.chapters.iter_mut(){
            chapter.keep_snippets(snippets);
        }
        self
    }

    pub fn add_chapter<'a>(&'a mut self, name:&str) -> &'a mut Chapter{
        self.chapters.push(Chapter::new(name));
        let index = self.chapters.len()-1;
        &mut self.chapters[index]
    }

    pub fn add_chapter_autoname(&mut self) -> &mut Chapter{
        self.add_chapter(&format!("Chapter {}",self.chapters.len()+1))
    }

    pub fn len(&self) -> usize{
        self.chapters.len()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata{
    pub name:String,
    pub summary:String,
    pub tags:Vec<String>
}

impl Metadata{
    pub fn new(name:&str) -> Metadata{
        Metadata{
            name: name.to_string(),
            summary: "".to_string(),
            tags: vec![]
        }
    }

    pub fn rename_tag(&mut self, old_tag:&str, new_tag:&str){
        self.tags = self.tags.iter().map(
            |x| {                
                if x==old_tag {
                    new_tag.to_string()
                }
                else{
                    x.to_string()
                }
            }
        ).collect();
    }
}
/*
enum ChapterTextSnippet<'a>{
    Snippet{id:&'a str,text:&'a str,remainder:Option<&'a str>},
    End,
    Remainder(&'a str),
    EndMissing(&'a str)
}

fn next_snippet<'a>(text:&'a str, id_prefix:&str, id_postfix:&str) -> ChapterTextSnippet<'a>{
    if let Some(start) = text.find(id_prefix){
        let id_start = start + id_prefix.len();
        if let Some(end) = text[id_start..].find(id_postfix){
            let id:&str = text[id_start..][..end]
            let text = text[id_start..][()]
            ChapterTextSnippet::End
        }
        else{
            ChapterTextSnippet::EndMissing(&text[id_start..])
        }
    }
    else{
        if text.len() == 0{
            ChapterTextSnippet::End
        }
        else{
            ChapterTextSnippet::Remainder(text)
        }
    }
}
*/

pub fn all_tags(metadata:&HashMap<String,Metadata>) -> BTreeSet<String>{
    let mut set = BTreeSet::new();
    for (_,v) in metadata.iter(){
        for tag in v.tags.iter(){
            set.insert(tag.to_string());
        }
    }
    set
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database{
    pub snippets:HashMap<String,String>,
    pub metadata:HashMap<String,Metadata>,
    pub documents:HashMap<String,Document>
}

impl Database{
    pub fn new() -> Database {
        Database{
            snippets:HashMap::new(),
            metadata:HashMap::new(),
            documents:HashMap::new()
        }
    }

    pub fn new_document_id(&self,name:&str) -> String{
        let id = name.to_string()
                    .replace(" ","_")
                    .replace(",","_")
                    .replace(".","")
                    .replace("+","_")
                    .replace("&","_")
                    .replace("?","")
                    .replace("!","")
                    .to_lowercase();
        if self.documents.contains_key(&id){
            for i in 1..{
                let new_id = format!("{}_{}",id,i);
                if !self.documents.contains_key(&new_id){
                    return new_id;
                }
            }
            panic!("This should not happen!");
        }
        else{
            id
        }
    }

    pub fn all_tags(&self) -> BTreeSet<String>{
        all_tags(&self.metadata)
    }

    pub fn rename_tag(&mut self, old_tag:&str, new_tag:&str){
       for (_,m) in self.metadata.iter_mut(){
           m.rename_tag(old_tag, new_tag);
       }
    }

    pub fn new_snippet_id(&mut self,name:&str)->String{
        let id = name.to_string()
                    .replace(" ","_")
                    .replace(",","_")
                    .replace(".","")
                    .replace("+","_")
                    .replace("&","_")
                    .replace("?","")
                    .replace("!","")
                    .to_lowercase();
        self.tidy();
        if self.snippets.contains_key(&id){
            for i in 1..{
                let new_id = format!("{}_{}",id,i);
                if !self.snippets.contains_key(&new_id){
                    return new_id;
                }
            }
            panic!("This should not happen!");
        }
        else{
            id
        }
    }

    pub fn new_snippet(&mut self, id:&str) -> String{
        let id = self.new_snippet_id(id);
        self.snippets.insert(id.clone(), "".to_string());
        self.fill();
        id
    } 

    pub fn keys_chain(&self) -> impl Iterator<Item = &String>{
        self.snippets.keys().chain(self.metadata.keys())
    }

    pub fn keys(&self) -> impl Iterator<Item = String>{
        let mut set = HashSet::new();
        for key in self.keys_chain(){
            set.insert(key.clone());
        }
        set.into_iter()
    }

    pub fn fill(&mut self)->&mut Database{
        self.document();
        let keys = self.keys().collect::<Vec<String>>();
        for key in keys.iter(){
            if !self.snippets.contains_key(key){
                self.snippets.insert(key.clone(), "".to_string());
            }
            if !self.metadata.contains_key(key){
                self.metadata.insert(key.clone(), Metadata::new(key));
            }
        }
        self
    }

    pub fn remove_undefined_snippets_from_documents(&mut self) -> &mut Database{
        let keys = self.keys().collect::<Vec<String>>();
        self.documents.values_mut().map(
            |doc| doc.keep_snippets(&keys)
        );
        self
    }

    pub fn tidy(&mut self) -> &mut Database{
        self.fill().remove_undefined_snippets_from_documents()
    }

    pub fn new_document<'a>(&'a mut self, id:&str, name:&str) -> &'a mut Document{
        self.documents.insert(id.to_string(), Document::new(name));
        self.documents.get_mut(id).unwrap()
    }
    pub fn new_document_autoid<'a>(&'a mut self, name:&str) -> String{
        let id = self.new_document_id(name);
        self.new_document(&id, name);
        id
    }

    pub fn document<'a>(&'a mut self) -> &'a mut Document{
        if self.documents.contains_key("document"){
            self.documents.get_mut("document").unwrap()
        }
        else{
            self.new_document("document", "Document")
        }
    }

    pub fn get_document_mut<'a>(&'a mut self,id:&str) -> &'a mut Document{
        if self.documents.contains_key(id){
            self.documents.get_mut(id).unwrap()
        }
        else{
            self.new_document(id,id)
        }
    }

    pub fn get_document<'a>(&'a mut self,id:&str) -> &'a Document{
        if self.documents.contains_key(id){
            self.documents.get(id).unwrap()
        }
        else{
            self.new_document(id,id)
        }
    }

    pub fn to_pretty_json(&self) -> serde_json::Result<String>{
        serde_json::to_string_pretty(self)
    }
    pub fn to_json(&self) -> serde_json::Result<String>{
        serde_json::to_string(self)
    }
    pub fn from_json(json:&str) -> serde_json::Result<Database>{
        serde_json::from_str(json)
    }
    pub fn clear(&mut self) -> &mut Database{
        self.snippets.clear();
        self.metadata.clear();
        self.documents.clear();
        self
    }

    pub fn update_with_draining(&mut self, db:& mut Database) -> &mut Database{
        db.snippets.drain().map(
            |(key,value)|{
                self.snippets.insert(key,value);
            }
        );
        db.metadata.drain().map(
            |(key,value)|{
                self.metadata.insert(key,value);
            }
        );
        db.documents.drain().map(
            |(key,value)|{
                self.documents.insert(key,value);
            }
        );
        self
    }
    pub fn update_with(&mut self, db:&Database) -> &mut Database{
        db.snippets.iter().map(
            |(key,value)|{
                self.snippets.insert(key.clone(),value.clone());
            }
        );
        db.metadata.iter().map(
            |(key,value)|{
                self.metadata.insert(key.clone(),value.clone());
            }
        );
        db.documents.iter().map(
            |(key,value)|{
                self.documents.insert(key.clone(),value.clone());
            }
        );
        self
    }

    pub fn get_chapter_text(&self, document_id:&str, chapter_number:usize, id_prefix:&str, id_postfix:&str) -> String {
        if let Some(doc) = self.documents.get(document_id){
            if let Some(ch) = &doc.chapters.get(chapter_number){
                let snippets = &self.snippets;
                ch.snippets.iter().map(
                    |id| format!("{}{}{}\n{}",id_prefix,id,id_postfix,snippets.get(id).unwrap_or(&"".to_string()))
                ).collect::<Vec<String>>().join(", ")
            }
            else{
                "Error 1".to_string()
            }
        }
        else{
            "Error 2".to_string()
        }
    }

    pub fn set_chapter_text(&mut self, document_id:&str, chapter_number:usize, id_prefix:&str, id_postfix:&str, text:&str) -> Result<()> {
        let mut snippet_ids:Vec<String> = Vec::new();
        {
            let doc:&mut Document = self.get_document_mut(document_id);
            if doc.chapters.len() <= chapter_number{
                return Err(Box::new(generic_error(&format!("Chapter {} not in '{}' containing {} chapters.",chapter_number,document_id,doc.chapters.len()))));
            }
            for fragment in text.split_terminator(id_prefix).skip(1){
                let v = fragment.split_terminator(id_postfix).collect::<Vec<&str>>();
                if v.len() >= 2{
                    snippet_ids.push(v[0].to_string());
                    self.snippets.insert(v[0].to_string(), v[1..].join("\n").to_string());
                }
            }
        }
        let doc:&mut Document = self.tidy().get_document_mut(document_id);
        let chapter:&mut Chapter = &mut doc.chapters[chapter_number];
        chapter.snippets = snippet_ids;
        Ok(())
    }

}


