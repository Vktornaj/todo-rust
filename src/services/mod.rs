extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use std::env;
use std::time::SystemTime;
use uuid::Uuid;

use crate::models;
use crate::schema;
// use rocket_dyn_templates::{context, Template};

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]

pub struct NewTodo {
    user_id: Uuid,
    title: String,
    description: String,
    status: i32,
    create_date: SystemTime,
    done_date: Option<SystemTime>,
    deadline: Option<SystemTime>,
}


fn is_available_username(connection: &mut PgConnection, username_: &String) -> bool {
    use models::user::User;
    use self::schema::_user::dsl::*;
    let result = _user
        .filter(username.like(username_))
        .limit(1)
        .load::<User>(connection)
        .expect("Error loading user");
    result.len() == 0
}

// User
#[post("/user", format = "json", data = "<user>")]
pub fn create_user(
    user: Json<NewUser>
) -> Result<Json<models::user::User>, Status> {
    use models::user::User;
    use self::schema::_user::dsl::*;
    let connection = &mut establish_connection_pg();

    let new_user = User {
        id: Uuid::new_v4(),
        username: user.username.to_string(),
        first_name: Some(user.first_name.to_string()),
        last_name: Some(user.last_name.to_string()),
        password: user.password.to_string(),
    };

    if !is_available_username(connection, &new_user.username) {
        return Err(Status::NotAcceptable);
    }

    diesel::insert_into(_user)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");
    Ok(Json(new_user))
}

#[get("/users")]
pub fn list_users() -> Result<Json<Vec<models::user::User>>, Status> {
    use models::user::User;
    let connection = &mut establish_connection_pg();
    let results = self::schema::_user::dsl::_user
        .load::<User>(connection)
        .expect("Error loading users");
    Ok(Json(results))
}

// Todo
#[post("/todo", format = "json", data = "<todo>")]
pub fn create_todo(
    todo: Json<NewTodo>
) -> Result<Json<models::todo::Todo>, Status> {
    use models::todo::Todo;
    use self::schema::_todo::dsl::*;
    let connection = &mut establish_connection_pg();

    let new_todo = Todo {
        id: Uuid::new_v4(),
        user_id: todo.user_id,
        title: todo.title.clone(),
        description: Some(todo.description.clone()),
        status: todo.status,
        create_date: todo.create_date,
        done_date: todo.done_date,
        deadline: todo.deadline,
    };

    let result = _todo
        .filter(title.like(&new_todo.title))
        .limit(1)
        .load::<Todo>(connection)
        .expect("Error loading user");

    if result.len() != 0 {
        return Err(Status::NotAcceptable);
    }

    diesel::insert_into(_todo)
        .values(&new_todo)
        .execute(connection)
        .expect("Error saving new user");
    Ok(Json(new_todo))
}

#[get("/todos")]
pub fn list_todos() -> Result<Json<Vec<models::user::User>>, Status> {
    use models::user::User;
    let connection = &mut establish_connection_pg();
    let results = self::schema::_user::dsl::_user
        .load::<User>(connection)
        .expect("Error loading users");
    Ok(Json(results))
}