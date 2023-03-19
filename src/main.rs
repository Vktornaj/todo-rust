use todo_rust;
use rocket::{self};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = todo_rust::rocket()
        .ignite().await?
        .launch().await?;
    Ok(())
}
