#![allow(dead_code)]
extern crate curl;
extern crate serde;
extern crate serde_json;

use dictionaries::traits::{Definition, Dictionary};

#[derive(Deserialize)]
struct UrbanDefinition {
    defid: u32,
    word: String,
    author: String,
    permalink: String,
    definition: String,
    example: Option<String>,
    thumbs_up: Option<i32>,
    thumbs_down: Option<i32>,
    current_vote: Option<String>
}
#[derive(Deserialize)]
struct Response {
    tags: Option<Vec<String>>,
    result_type: Option<String>,
    sounds: Option<Vec<String>>,
    list: Vec<UrbanDefinition>
}

pub struct Urban {
    session: curl::http::Handle,
    pub key: String
}

impl Urban {
    pub fn new(key: &str) -> Urban {
        Urban {key: key.to_owned(), session: curl::http::handle()}
    }
}

impl Dictionary for Urban {
    fn get_definitions(&mut self, word: &str) -> Result<Vec<Definition>, &str> {
        let mut session = &mut self.session;
        let url = format!("https://mashape-community-urban-dictionary.p.mashape.com/define?term={}", word);
        let request = session.get(url)
            .header("X-Mashape-Key", &self.key)
            .exec().unwrap();
        let response_string = String::from_utf8_lossy(request.get_body());
        let response: Response = serde_json::from_str(&response_string).unwrap();
        let mut definitions: Vec<Definition> = Vec::new();
        for definition in response.list {
            definitions.push(Definition { word: definition.word.to_owned(), text: definition.definition.to_owned() });
        }
        if definitions.len()>0 {
            Ok(definitions)
        }
        else {
            Err("Couldn't find any definitions")
        }
    }

    fn clone_to_box(&self) -> Box<Dictionary> { Box::new(self.clone()) }
}

impl Clone for Urban {
    fn clone(&self) -> Self {
        Urban {key: self.key.clone(), session: curl::http::handle()}
    }
}

unsafe impl Send for Urban {}
unsafe impl Sync for Urban {}
