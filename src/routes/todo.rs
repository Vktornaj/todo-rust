extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::{get, post};
use rocket::serde::{json::Json};
use dotenvy::dotenv;
use std::env;

use crate::models::todo::{NewTodoJson, TodoJson};
use crate::db;


pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[post("/todo", format = "json", data = "<todo>")]
pub fn create_todo(
    todo: Json<NewTodoJson>
) -> Json<&'static str> {
    let connection = &mut establish_connection_pg();
    if !db::todo::is_available_title(connection, &todo.title) {
        return Json("{ 'msg': 'fail' }");
    }
    // TODO: extract username from jwt
    let id = 1;

    let new_todo = todo.attach(id);
    db::todo::write_todo(connection, new_todo);
    Json("{ 'msg': 'done' }")
}

#[get("/todos/<from>/<to>")]
pub fn list_todos(from: i64, to: i64) -> Result<Json<Vec<TodoJson>>, Status> {

    if to - from > 10 {
        return Err(Status::BadRequest);
    }

    let connection = &mut establish_connection_pg();

    // TODO: extract username from jwt
    let username = "vktornaj".to_string();

    let results: Vec<TodoJson> = match db::user::read_user(connection, &username) {
        Some(user) => {
            let results = db::todo::read_todos(connection, &user, from, to);
            results.into_iter().map(|x| x.attach()).collect()
        },
        None => Vec::<TodoJson>::new()
    };

    Ok(Json(results))
}