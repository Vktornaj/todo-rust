use rocket::{Request, catch, catchers};
use cors::CORS;
use rocket::{launch, routes};
use dotenv::dotenv;

mod db;
mod cors;
mod auth;
mod config;
mod routes;
mod models;
mod schema;


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api", 
            routes![
                routes::user::username_available,
                routes::user::create_user,
                routes::user::list_users,
                routes::user::login,
                routes::todo::create_todo,
                routes::todo::list_todos,
            ],
        )    
        .attach(CORS)
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
