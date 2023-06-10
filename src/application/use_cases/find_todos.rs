use crate::domain::{todo::Todo, auth::Auth};

use super::super::port::driven::todo_repository::TodoRepository;


#[derive(Debug)]
pub enum FindAllError {
    Unknown(String),
    Unautorized(String),
}

pub fn execute(
    repo: &impl TodoRepository, 
    secret: &[u8],
    token: &String, 
    from: i64, 
    to: i64
) -> Result<Vec<Todo>, FindAllError> {
    let username = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(FindAllError::Unautorized("Invalid token".to_string()));
    };
    match repo.find_all(&username, from, to).ok() {
        Some(todo) => Ok(todo),
        None => Err(FindAllError::Unknown("not found".to_string())),
    }
}