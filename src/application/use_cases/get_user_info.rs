use crate::domain::{user::User, auth::Auth};

use super::super::port::driven::user_repository::UserRepository;

#[derive(Debug)]
pub enum FindError {
    Unknown(String),
    Unautorized(String)
}

pub fn execute(repo: &impl UserRepository, token: &String) -> Result<User, FindError> {
    let username = if let Ok(auth) = Auth::from_token(token, &vec![0, 0]) {
        auth.username
    } else {
        return Err(FindError::Unautorized("Invalid token".to_string()));
    };
    match repo.find_one(&username) {
        Ok(user) => Ok(user),
        Err(error) => Err(FindError::Unknown("user not found".to_string())),
    }
}