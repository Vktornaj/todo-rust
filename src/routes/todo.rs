extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::{get, post};
use rocket::serde::{json::Json};
use dotenvy::dotenv;
use std::env;

use crate::models::todo::{Todo, NewTodoJson};
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
    if db::todo::is_available_title(connection, &todo.title) {
        return Json("{ 'msg': 'fail' }");
    }
    let new_todo = todo.attach();
    db::todo::write_todo(connection, new_todo);
    Json("{ 'msg': 'done' }")
}

#[get("/todos")]
pub fn list_todos() -> Result<Json<Vec<Todo>>, Status> {
    let connection = &mut establish_connection_pg();
    let results = db::todo::read_todos(connection);
    Ok(Json(results))
}