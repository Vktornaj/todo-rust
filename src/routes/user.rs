extern crate rocket;
use std::env;

use diesel::pg::{PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::{get, post};
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::models::user::{User, NewUserJson};
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
pub fn create_user(
    user: Json<NewUserJson>
) -> Json<&'static str>  {
    let connection = &mut establish_connection_pg();

    let new_user = user.attach();

    if !db_user::is_available_username(connection, &new_user.username) {
        return Json("{ 'msg': 'fail' }");
    }

    db_user::write_user(connection, new_user);

    Json("{ 'msg': 'done' }")
}

#[get("/users")]
pub fn list_users() -> Result<Json<Vec<User>>, Status> {
    let connection = &mut establish_connection_pg();
    let results = db_user::read_users(connection);
    Ok(Json(results))
}