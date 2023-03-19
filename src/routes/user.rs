extern crate rocket;
use std::env;

use diesel::pg::{PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::State;
use rocket::{get, post};
use rocket::serde::{json::Json, Deserialize, Serialize};
use chrono::{Duration, Utc};

use crate::config::AppState;
use crate::models::user::{NewUserJson, UserJson};
use crate::db::user as db_user;
use crate::auth::Auth;


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Available {
    is_available: bool
}

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthToken {
    authorization_token: String,
    token_type: String,
}

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[post("/register", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUserJson>) -> Json<&'static str>  {
    let connection = &mut establish_connection_pg();

    let err_value = Json("{ 'msg': 'fail' }");

    if !db_user::is_available_username(connection, &user.0.username) {
        return err_value;
    }
    let mut new_user = user.0.attach();
    
    if new_user.hash_password().is_err() {
        return err_value;
    }
    
    db_user::write_user(connection, &new_user);
    Json("{ 'msg': 'done' }")
}

#[get("/get-username-availability/<username>")]
pub fn username_available(username: String) -> Json<Available> {
    let connection = &mut establish_connection_pg();

    let is_available = db_user::is_available_username(connection, &username);
    Json(Available{ is_available })
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
    (Status::Accepted, Some(Json(user.unwrap().attach())))
}

#[post("/login", format = "json", data = "<credentials>")]
pub fn login(
    credentials: Json<Credentials>,
    state: &State<AppState>,
) -> (Status, Option<Json<AuthToken>>)  {
    let connection = &mut establish_connection_pg();

    let user = db_user::read_user_username(connection, &credentials.username);

    if user.is_none() {
        return (Status::Unauthorized, None);
    }

    if user.as_ref().unwrap().verify_password(&credentials.password).is_err() {
        return (Status::Unauthorized, None);
    }

    let token = Auth { 
        exp: (Utc::now() + Duration::days(60)).timestamp(), 
        id: user.unwrap().id
    }.token(&state.secret);

    (
        Status::Accepted,
        Some(Json(
            AuthToken { 
                authorization_token: token, 
                token_type: "barer".to_string()
            }
        ))
    )
}
