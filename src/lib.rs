use rocket::{Request, catch, catchers, options};
use cors::CORS;
use rocket::{launch, routes};
use dotenv::dotenv;

mod db;
mod cors;
mod auth;
mod config;
pub mod routes;
pub mod models;
mod schema;


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
        .attach(CORS)
        .mount(
            "/api", 
            routes![
                routes::user::username_available,
                routes::user::create_user,
                routes::user::list_users,
                routes::user::login,
                routes::user::get_user_info,
                routes::todo::post_todo,
                routes::todo::update_todo,
                routes::todo::delete_todo,
                routes::todo::get_todos,
                routes::todo::put_add_tag,
                routes::todo::put_remove_tag,
                all_options,
            ],
        )
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
