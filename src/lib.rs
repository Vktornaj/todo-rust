use rocket::{Request, catch, catchers, options, get};
use cors::CORS;
use rocket::{launch, routes};
use dotenv::dotenv;

mod config;
mod domain;
mod application;
mod adapter;
mod cors;

use adapter::driving::web;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[get("/")]
pub fn get_root() -> &'static str {
    "{ \"msg\": \"ok\" }"
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
        .attach(CORS)
        .mount(
            "/", 
            routes![
                get_root
            ]
        )
        .mount(
            "/api", 
            routes![
                web::user::username_available,
                web::user::create_user,
                web::user::list_users,
                web::user::login,
                web::user::get_user_info,
                web::todo::post_todo,
                web::todo::update_todo,
                web::todo::delete_todo,
                web::todo::get_todos,
                web::todo::put_add_tag,
                web::todo::put_remove_tag,
                all_options,
            ]
        )
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}