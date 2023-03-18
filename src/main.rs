extern crate rocket;
use rocket::{launch, routes};

mod routes;

pub mod models;
pub mod schema;
pub mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            routes::user::username_available,
            routes::user::create_user,
            routes::user::list_users,
            routes::user::login,
            routes::todo::create_todo,
            routes::todo::list_todos,
        ])
}
