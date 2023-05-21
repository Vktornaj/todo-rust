use super::super::port::driven::todo_repository::{TodoRepository, FindTodo};
use crate::domain::{todo::Status, auth::Auth};


#[derive(Debug)]
pub enum DeleteError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

fn execute(repo: &impl TodoRepository, token: &String) -> Result<(), DeleteError> {
    let user_id = if let Ok(auth) = Auth::from_token(token, &vec![0, 0]) {
        auth.id
    } else {
        return Err(DeleteError::Unautorized("Invalid token".to_string()));
    };
    let find_todo = FindTodo { 
        title: None, 
        description: None, 
        status: Some(Status::DONE), 
        tags: None 
    };
    match repo.delete_all_criteria(user_id, &find_todo) {
        Ok(_) => Ok(()),
        Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
    }
}