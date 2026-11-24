
pub mod err;

use std::error::Error;
use std::fs;
use std::io;

pub use err::{ParseErr, ReadErr};

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
        // 1. Read file
        let content = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(err) => {
                return Err(Box::new(ReadErr {
                    child_err: Box::new(err),
                }));
            }
        };

        // 2. Parse JSON
        let json = match json::parse(&content) {
            Ok(data) => data,
            Err(err) => {
                return Err(Box::new(ParseErr::Malformed(
                    Box::new(err)
                )));
            }
        };

        // 3. Read title
        let title = match json["title"].as_str() {
            Some(value) => value.to_string(),
            None => {
                return Err(Box::new(ParseErr::Malformed(
                    Box::new(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid title"
                    ))
                )));
            }
        };

        if !json["tasks"].is_array() {
            return Err(Box::new(ParseErr::Malformed(
                Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid tasks"
                ))
            )));
        }

        let task_values: Vec<_> = json["tasks"].members().collect();

        if task_values.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks = Vec::new();

        for task in task_values {
            let id = task["id"].as_u32();
            let description = task["description"].as_str();
            let level = task["level"].as_u32();

            match (id, description, level) {
                (Some(id), Some(description), Some(level)) => {
                    tasks.push(Task {
                        id,
                        description: description.to_string(),
                        level,
                    });
                }
                _ => {
                    return Err(Box::new(ParseErr::Malformed(
                        Box::new(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid task fields"
                        ))
                    )));
                }
            }
        }

        Ok(TodoList {
            title,
            tasks,
        })
    }
}
