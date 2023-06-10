use crate::domain::{todo::Todo, auth::Auth};

use super::super::port::driven::todo_repository::TodoRepository;


#[derive(Debug)]
pub enum FindError {
    Unknown(String),
    Unauthorized(String)
}

fn execute(
    repo: &impl TodoRepository,
    secret: &[u8],
    token: &String,
    id: i64
) -> Result<Todo, FindError> {
    if Auth::from_token(token, secret).is_err() {
        return Err(FindError::Unauthorized("Invalid token".to_string()));
    };
    match repo.find_one(id).ok() {
        Some(todo) => Ok(todo),
        None => Err(FindError::Unknown("not found".to_string())),
    }
}