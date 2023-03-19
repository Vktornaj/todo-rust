extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::{get, post, put, delete};
use rocket::serde::{json::Json};
use dotenvy::dotenv;
use std::env;

use crate::models::todo::{NewTodoJson, TodoJson, TodoUpdateJson};
use crate::db;
use crate::auth::Auth;


pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[post("/todo", format = "json", data = "<todo>")]
pub fn post_todo(
    todo: Json<NewTodoJson>,
    auth: Auth,
) -> (Status, &'static str) {
    let connection = &mut establish_connection_pg();
    if !db::todo::is_available_title(connection, auth.id, &todo.title) {
        return (Status::Gone, "the title is in conflict");
    }

    let new_todo = todo.attach(auth.id);
    db::todo::write_todo(connection, new_todo);
    (Status::Accepted, "done")
}

// TODO: add update todo endpoint
#[put("/todo/<id>", format = "json", data = "<todo_update>")]
pub fn update_todo(
    id: i32,
    todo_update: Json<TodoUpdateJson>,
    auth: Auth,
) -> (Status, &'static str) {

    let todo_update = todo_update.0;
    let connection = &mut establish_connection_pg();

    if !db::todo::is_belonging_to(connection, auth.id, id) {
        return (Status::Gone, "todo not found");
    }

    if todo_update.title.is_some() 
        && !db::todo::is_available_title(connection, auth.id, &todo_update.title.as_ref().unwrap()) {
        return (Status::Gone, "the title is in conflict");
    }

    if db::todo::update_todo(connection, todo_update.attach(auth.id, id)).is_err() {
        return (Status::Gone, "error updating");
    }

    (Status::Accepted, "done")
}

// TODO: add delete todo endpoint
#[delete("/todo/<id>")]
pub fn delete_todo(
    id: i32,
    auth: Auth,
) -> Status {
    let connection = &mut establish_connection_pg();

    db::todo::is_belonging_to(connection, auth.id, id);
    if db::todo::delete_todo(connection, id).is_err() {
        return Status::Gone;
    }

    Status::Accepted
}

#[get("/todos/<from>/<to>")]
pub fn get_todos(from: i64, to: i64, auth: Auth) -> Result<Json<Vec<TodoJson>>, Status> {

    if to - from > 10 {
        return Err(Status::BadRequest);
    }

    let connection = &mut establish_connection_pg();

    let results: Vec<TodoJson> = match db::user::read_user(connection, &auth.id) {
        Some(user) => {
            let results = db::todo::read_todos(connection, &user, from, to);
            results.into_iter().map(|x| x.attach()).collect()
        },
        None => Vec::<TodoJson>::new()
    };

    Ok(Json(results))
}