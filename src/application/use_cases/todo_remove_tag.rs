use crate::domain::{todo::Todo, auth::Auth};

use super::super::port::driven::todo_repository::TodoRepository;


#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

fn execute(repo: &impl TodoRepository, token: &String, todo_id: i64, tag: &String) -> Result<Todo, UpdateError> {
    let user_id = if let Ok(auth) = Auth::from_token(token, &vec![0, 0]) {
        auth.id
    } else {
        return Err(UpdateError::Unautorized("Invalid token".to_string()));
    };
    let todo = if let Ok(todo) = repo.find_one(todo_id) {
        todo
    } else {
        return Err(UpdateError::Unknown(format!("Unknown error")));
    };

    if !todo.tags.contains(tag) {
        return Err(UpdateError::Conflict(format!("Tag not found")));
    }

    match repo.remove_tag(user_id, todo_id, &tag) {
        Ok(todo) => Ok(todo),
        Err(error) => Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}