use super::super::port::driven::todo_repository::{TodoRepository};
use crate::{domain::{todo::Todo, auth::Auth}, application::port::driven::todo_repository::UpdateTodo};


#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

fn execute(repo: &impl TodoRepository, token: &String, update_todo: UpdateTodo) -> Result<Todo, UpdateError> {
    let user_id = if let Ok(auth) = Auth::from_token(token, &vec![0, 0]) {
        auth.id
    } else {
        return Err(UpdateError::Unautorized("Invalid token".to_string()));
    };
    match repo.update(user_id, &update_todo) {
        Ok(todo) => Ok(todo),
        Err(error) => Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}