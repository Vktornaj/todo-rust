use diesel::{PgConnection, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use dotenvy::dotenv;
use todo_rust;
use rocket::{self};


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let conn = &mut establish_connection_pg();
    run_migration(conn);
    let _rocket = todo_rust::rocket()
        .ignite().await?
        .launch().await?;
    Ok(())
}
