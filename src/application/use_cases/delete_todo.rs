use crate::domain::auth::Auth;

use super::{super::port::driven::todo_repository::TodoRepository};


#[derive(Debug)]
pub enum DeleteError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

fn execute(repo: &impl TodoRepository, token: &String, id: i64) -> Result<(), DeleteError> {
    let user_id = if let Ok(auth) = Auth::from_token(token, &vec![0, 0]) {
        auth.id
    } else {
        return Err(DeleteError::Unautorized("Invalid token".to_string()));
    };
    if repo.find_one(id).is_ok() {
        match repo.delete(user_id, id) {
            Ok(_) => Ok(()),
            Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
        }
    } else {
        return Err(DeleteError::Conflict("Todo does not exist".to_string()));
    }
}