#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::collections::{HashMap,HashSet};

use std::str::FromStr;


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

}


