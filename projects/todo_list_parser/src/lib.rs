//! This crate provides an API to parse a list of todos

use std::fs::read_to_string;
use std::path::Path;

mod error;
use error::{ParseError, ReadError};

use std::error::Error;

#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<String>,
}

impl TodoList {
    pub fn get_todos<P>(path: P) -> Result<TodoList, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let read_todos: Result<String, Box<dyn Error>> = read_todos(path);
        let parsed_todos = parse_todos(&read_todos?)?;
        Ok(parsed_todos)
    }
}

pub fn read_todos<P>(path: P) -> Result<String, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let raw_todos = read_to_string(path).map_err(|e| ReadError {
        child_error: Box::new(e),
    })?;
    Ok(raw_todos)
}

pub fn parse_todos(todo_str: &str) -> Result<TodoList, Box<dyn Error>> {
    let mut tasks: Vec<String> = vec![];

    for line in todo_str.lines() {
        tasks.push(line.to_string());
    }

    if tasks.is_empty() {
        Err(ParseError::Empty.into())
    } else {
        Ok(TodoList { tasks: tasks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
