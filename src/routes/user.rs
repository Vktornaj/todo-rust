extern crate rocket;
use std::env;

use diesel::pg::{PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::{get, post};
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::models::user::{NewUserJson, UserJson};
use crate::db::user as db_user;


#[derive(Serialize, Deserialize)]
pub struct NewUser {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
}

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[post("/user", format = "json", data = "<user>")]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Available {
    is_available: bool
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

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[post("/login", format = "json", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>) -> Option<Json<&'static str>>  {
    let connection = &mut establish_connection_pg();

    let user = db_user::read_user(connection, &credentials.username)?;

    user.verify_password(&credentials.password).ok()?;

    Some(Json("{ 'msg': 'done' }"))
}
