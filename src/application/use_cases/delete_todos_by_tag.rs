use crate::domain::auth::Auth;

use super::super::port::driven::todo_repository::{TodoRepository, FindTodo};


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
    tag: &String
) -> Result<(), DeleteError> {
    let username = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(DeleteError::Unautorized("Invalid token".to_string()));
    };
    let find_todo = FindTodo {
        title: None,
        description: None,
        status: None,
        tags: Some(vec![tag.to_owned()]),
    };
    match repo.delete_all_criteria(&username, &find_todo) {
        Ok(_) => Ok(()),
        Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
    }
}