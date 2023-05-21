use super::super::port::driven::user_repository::UserRepository;
use crate::domain::user::User;
use super::find_user;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

fn execute(repo: &impl UserRepository, user: &mut User) -> Result<User, CreateError> {
    if user.hash_password().is_err() {
        return Err(CreateError::InvalidData("Invalid password".to_string()));
    }
    if find_user::execute(repo, &user.username).is_some() {
        Err(CreateError::Conflict("Username is already in use".to_string()))
    } else {
        match repo.create(&user) {
            Ok(user) => Ok(user),
            Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
        }
    }
}