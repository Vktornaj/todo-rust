use crate::domain::todo::Todo;

use super::super::port::driven::todo_repository::TodoRepository;


#[derive(Debug)]
pub enum FindError {
    Unknown(String)
}

fn execute(repo: &impl TodoRepository, id: i64) -> Result<Todo, FindError> {
    match repo.find_one(id).ok() {
        Some(todo) => Ok(todo),
        None => Err(FindError::Unknown("not found".to_string())),
    }
}