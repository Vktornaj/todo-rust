extern crate rocket;

use rocket::http::{Status, ContentType};
use rocket::State;
use rocket::response::status;
use rocket::{get, post};
use rocket::serde::{json::Json, Serialize, Deserialize};

use crate::adapter::driven::persistence::user_repository_pgsql;
use crate::adapter::driving::web::token::Token;
use crate::adapter::driving::web::models::user::{NewUserJson, UserJson};
use crate::config::AppState;
use crate::application::use_cases;
use crate::domain::user::User;


pub fn create_user(user: Json<NewUserJson>) -> (Status, String)  {
    let mut _user: User = User {
        id: None,
        username: user.0.username,
        first_name: Some(user.first_name),
        last_name: Some(user.last_name),
        password: user.password,
    };
    match use_cases::create_user::execute(&user_repository_pgsql::UserRepository::new(), &_user) {
        Ok(_) => (Status::Ok, "".to_string()),
        Err(error) => match error {
            use_cases::create_user::CreateError::InvalidData(s) => (Status::BadRequest, s),
            use_cases::create_user::CreateError::Unknown(s) => (Status::InternalServerError, s),
            use_cases::create_user::CreateError::Conflict(s) => (Status::Conflict, s),
        }
    }
}

#[get("/get-username-availability/<username>")]
pub fn username_available(username: String) -> (Status, (ContentType, String)) {
    let is_available = use_cases::is_user_exist::execute(
        &user_repository_pgsql::UserRepository::new(), 
        &username
    );
    (
        Status::Ok,
        (ContentType::JSON, format!("{{ \"isAvailable\": \"{is_available}\" }}"))
    )
}


pub fn get_user_info(token: Token) -> (Status, Option<Json<UserJson>>) {
    match use_cases::get_user_info::execute(
        &user_repository_pgsql::UserRepository::new(),
        &token.value
    ) {
        Ok(user) => (Status::Ok, Some(Json(UserJson::from_user(user)))),
        Err(_) => (Status::Gone, None),
    }   
}

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonToken {
    pub authorization_token: String,
    pub token_type: String,
}
#[post("/login", format = "json", data = "<credentials>")]
pub fn login(
    credentials: Json<Credentials>,
    state: &State<AppState>,
) -> Result<Json<JsonToken>, status::Unauthorized<String>> {
    let invalid_msg = "invalid credentials".to_string();

    match use_cases::login_user::execute(
        &user_repository_pgsql::UserRepository::new(), 
        &credentials.username, 
        &credentials.password
    ) {
        Ok(token) => Ok(Json(JsonToken { 
            authorization_token: token, 
            token_type: "Bearer".to_string() 
        })),
        Err(error) => Err(status::Unauthorized(Some(invalid_msg))),
    }
}