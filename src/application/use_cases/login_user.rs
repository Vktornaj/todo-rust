use super::super::port::driven::user_repository::UserRepository;
use crate::domain::auth::Auth;


#[derive(Debug)]
pub enum LoginError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

// TODO: Set correct passowrd
pub async fn execute<T>(conn: &T, repo: &impl UserRepository<T>, username: &String, password: &String) -> Result<String, LoginError> {
    let user = if let Ok(user) = repo.find_one(conn, username).await {
        user
    } else {
        return Err(LoginError::InvalidData("User not found".to_string()));
    };

    if user.verify_password(password).is_ok() {
        Ok(Auth::new(&user.username).token(&vec![0, 0]))
    } else  {
        Err(LoginError::InvalidData("Invalid password".to_string()))
    }
}