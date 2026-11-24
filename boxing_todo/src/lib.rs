mod err;

use std::{error::Error, fs, io};

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
        let contents = fs::read_to_string(path).map_err(|error| {
            Box::new(ReadErr {
                child_err: Box::new(error),
            }) as Box<dyn Error>
        })?;

        let value = json::parse(&contents)
            .map_err(|error| Box::new(ParseErr::Malformed(Box::new(error))) as Box<dyn Error>)?;

        parse_todo(&value).map_err(|error| Box::new(error) as Box<dyn Error>)
    }
}

fn parse_todo(value: &json::JsonValue) -> Result<TodoList, ParseErr> {
    let title = value["title"]
        .as_str()
        .ok_or_else(|| malformed("todo title must be a string"))?
        .to_owned();
    let tasks = &value["tasks"];

    if !tasks.is_array() {
        return Err(malformed("todo tasks must be an array"));
    }
    if tasks.is_empty() {
        return Err(ParseErr::Empty);
    }

    let tasks = tasks
        .members()
        .map(|task| {
            let id = task["id"]
                .as_u32()
                .ok_or_else(|| malformed("task id must be an unsigned integer"))?;
            let description = task["description"]
                .as_str()
                .ok_or_else(|| malformed("task description must be a string"))?
                .to_owned();
            let level = task["level"]
                .as_u32()
                .ok_or_else(|| malformed("task level must be an unsigned integer"))?;

            Ok(Task {
                id,
                description,
                level,
            })
        })
        .collect::<Result<Vec<_>, ParseErr>>()?;

    Ok(TodoList { title, tasks })
}

fn malformed(message: &'static str) -> ParseErr {
    ParseErr::Malformed(Box::new(io::Error::new(
        io::ErrorKind::InvalidData,
        message,
    )))
}
