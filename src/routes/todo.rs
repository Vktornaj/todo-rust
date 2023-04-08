extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::{get, post, put, delete};
use rocket::serde::{json::Json};
use dotenvy::dotenv;
use std::env;

use crate::models::tag::NewTag;
use crate::models::todo::{NewTodoJson, TodoJson, TodoUpdateJson};
use crate::database;
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
    if !database::todo::is_available_title(connection, auth.id, &todo.title) {
        return (Status::Conflict, "the title is in conflict");
    }

    let new_todo = todo.attach(auth.id);
    database::todo::write_todo(connection, new_todo);
    (Status::Ok, "done")
}

#[put("/todo/<id>", format = "json", data = "<todo_update>")]
pub fn update_todo(
    id: i32,
    todo_update: Json<TodoUpdateJson>,
    auth: Auth,
) -> (Status, &'static str) {

    let todo_update = todo_update.0;
    let connection = &mut establish_connection_pg();

    if !database::todo::is_belonging_to(connection, auth.id, id) {
        return (Status::Gone, "todo not found");
    }

    if todo_update.title.is_some() 
        && !database::todo::is_available_title(connection, auth.id, &todo_update.title.as_ref().unwrap()) {
        return (Status::Gone, "the title is in conflict");
    }

    if database::todo::update_todo(connection, todo_update.attach(auth.id, id)).is_err() {
        return (Status::Gone, "error updating");
    }

    (Status::Ok, "done")
}

#[delete("/todo/<id>")]
pub fn delete_todo(
    id: i32,
    auth: Auth,
) -> Status {
    let connection = &mut establish_connection_pg();

    if !database::todo::is_belonging_to(connection, auth.id, id) {
        return Status::Gone;
    }
    if database::todo::delete_todo(connection, id).is_err() {
        return Status::Gone;
    }

    Status::Ok
}

#[get("/todos/<from>/<to>")]
pub fn get_todos(from: i64, to: i64, auth: Auth) -> Result<Json<Vec<TodoJson>>, Status> {

    if to - from > 10 {
        return Err(Status::BadRequest);
    }

    let connection = &mut establish_connection_pg();

    let results: Vec<TodoJson> = match database::user::read_user(connection, &auth.id) {
        Some(user) => {
            let results = database::todo::read_todos(connection, &user, from, to);
            results.into_iter().map(|x| x.attach()).collect()
        },
        None => Vec::<TodoJson>::new()
    };

    Ok(Json(results))
}

// TODO: fix avoid repeated values
#[put("/todo/<id>/tag/<tag>")]
pub fn put_add_tag(id: i32, tag: String, auth: Auth) -> (Status, String) {
    let connection = &mut establish_connection_pg();
    if !database::todo::is_belonging_to(connection, auth.id, id) {
        return (Status::NoContent, "you don't have a todo with this id".to_owned());
    }
    let is_tag_existing = database::tag::is_user_tag_existing(connection, auth.id, &tag);
    if is_tag_existing.is_err() {
        return (Status::InternalServerError, "error querying the database".to_owned());
    }
    let tag_id = if is_tag_existing.unwrap() {
        database::tag::read_tag(connection, auth.id, &tag)
    } else {
        database::tag::write_tag(connection, &NewTag { tag_value: tag.clone() })
    };
    if tag_id.is_err() {
        return (Status::NotAcceptable, "database insert failed".to_owned());
    }
    if database::tag::associate_todo_tag(connection, id, tag_id.unwrap()).is_err() {
        return (
            Status::Conflict, 
            format!("todo id: {id} already has \"{}\" tag.", &tag)
        );
    }
    (Status::Ok, "".to_owned())
}

#[delete("/todo/<id>/tag/<tag>")]
pub fn put_remove_tag(id: i32, tag: String, auth: Auth) -> Status {
    let connection = &mut establish_connection_pg();
    if !database::todo::is_belonging_to(connection, auth.id, id) {
        return Status::Gone;
    }
    if database::tag::delete_todo_tag(connection, id, &tag).is_err() {
        return Status::Gone;
    }
    Status::Ok
}