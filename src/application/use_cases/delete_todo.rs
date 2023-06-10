use crate::domain::auth::Auth;

use super::{super::port::driven::todo_repository::TodoRepository};


#[derive(Debug)]
pub enum DeleteError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

fn execute(
    repo: &impl TodoRepository,
    secret: &[u8],
    token: &String, 
    id: i64
) -> Result<(), DeleteError> {
    let username = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(DeleteError::Unautorized("Invalid token".to_string()));
    };
    if repo.find_one(id).is_ok() {
        match repo.delete(&username, id) {
            Ok(_) => Ok(()),
            Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
        }
    } else {
        return Err(DeleteError::Conflict("Todo does not exist".to_string()));
    }
}