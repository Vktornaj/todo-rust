use super::super::port::driven::todo_repository::{TodoRepository, FindTodo};
use crate::domain::{todo::Todo, auth::Auth};


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

// TODO: Set secret correctly
fn execute(repo: &impl TodoRepository, token: &String, todo: &Todo) -> Result<Todo, CreateError> {
    let username = if let Ok(auth) = Auth::from_token(token, &vec![0, 0]) {
        auth.username
    } else {
        return Err(CreateError::Unautorized("Invalid token".to_string()));
    };
    let find_todo = FindTodo {
        title: Some(todo.title.to_owned()),
        description: None,
        status: None,
        tags: None,
    };
    if repo.find_one_criteria(&username, &find_todo).is_ok() {
        return Err(CreateError::Conflict("Title already exist".to_string()));
    }
    match repo.create(&username, todo) {
        Ok(todo) => Ok(todo),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}