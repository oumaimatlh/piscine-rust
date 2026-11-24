mod err;

pub use err::{ParseErr, ReadErr};

use json::{self, JsonValue};
use std::{error::Error, fs};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // read file
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(ReadErr { child_err: Box::new(e) })),
        };
        // parse json !!
        let parsed_json = match json::parse(&content) {
            Ok(p) => p,
            Err(e) => return Err(Box::new(ParseErr::Malformed(Box::new(e)))),
        };

        let title = match parsed_json["title"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "missing title"))))),
        };

        // members(): iterer sur les elements d'un objet Json  
        let tasks_json = parsed_json["tasks"].members().collect::<Vec<&JsonValue>>();
        if tasks_json.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks: Vec<Task> = Vec::new();
        for i in tasks_json {
            let id = match i["id"].as_u32() {
                Some(i) => i,
                None => return Err(Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "missing or malformed task id"))))),
            };
            let description = match i["description"].as_str() {
                Some(d) => d.to_string(),
                None => return Err(Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "missing or malformed task description"))))),
            };
            let level = match i["level"].as_u32() {
                Some(l) => l,
                None => return Err(Box::new(ParseErr::Malformed(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "missing or malformed task level"))))),
            }; 

            tasks.push(Task { id, description, level });
        }
       Ok(TodoList { title, tasks }) 
    }
}