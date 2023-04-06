extern crate rocket;
use std::env;

use diesel::pg::{PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::{Status, ContentType};
use rocket::State;
use rocket::response::status;
use rocket::{get, post};
use rocket::serde::{json::Json, Serialize, Deserialize};
use chrono::{Duration, Utc};

use crate::config::AppState;
use crate::models::user::{NewUserJson, UserJson};
use crate::db::user as db_user;
use crate::auth::Auth;


pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[post("/register", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUserJson>) -> (Status, String)  {
    let connection = &mut establish_connection_pg();

    if !db_user::is_available_username(connection, &user.0.username) {
        return (Status::NotAcceptable, "username already exist".to_string());
    }
    let mut new_user = user.0.attach();
    
    if new_user.hash_password().is_err() {
        return (Status::NotAcceptable, "password error".to_string());
    }
    
    db_user::write_user(connection, &new_user);
    (Status::Ok, "".to_string())
}

#[get("/get-username-availability/<username>")]
pub fn username_available(username: String) -> (Status, (ContentType, String)) {
    let connection = &mut establish_connection_pg();

    let is_available = db_user::is_available_username(connection, &username);
    (
        Status::Ok,
        (ContentType::JSON, format!("{{ \"isAvailable\": \"{is_available}\" }}"))
    )
}

#[get("/users")]
pub fn list_users() -> Result<Json<Vec<UserJson>>, Status> {
    let connection = &mut establish_connection_pg();
    let results = db_user::read_users(connection);
    let results = results.into_iter()
        .map(|x| x.attach()).collect();
    Ok(Json(results))
}

#[get("/user/info")]
pub fn get_user_info(auth: Auth) -> (Status, Option<Json<UserJson>>) {
    let connection = &mut establish_connection_pg();
    let user = db_user::read_user(connection, &auth.id);
    if user.is_none() {
        return (Status::Gone, None)
    }
    (Status::Ok, Some(Json(user.unwrap().attach())))
}

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub authorization_token: String,
    pub token_type: String,
}
#[post("/login", format = "json", data = "<credentials>")]
pub fn login(
    credentials: Json<Credentials>,
    state: &State<AppState>,
) -> Result<Json<Token>, status::Unauthorized<String>> {
    let connection = &mut establish_connection_pg();
    let user = db_user::read_user_username(connection, &credentials.username);

    let invalid_msg = "invalid credentials".to_string();

    if user.is_none() {
        return Err(status::Unauthorized(Some(invalid_msg)));
    }
    if user.as_ref().unwrap().verify_password(&credentials.password).is_err() {
        return Err(status::Unauthorized(Some(invalid_msg)));
    }
    let token = Auth { 
        exp: (Utc::now() + Duration::days(60)).timestamp(), 
        id: user.unwrap().id
    }.token(&state.secret);

    Ok(Json(Token { authorization_token: token, token_type: "Bearer".to_string() }))
}