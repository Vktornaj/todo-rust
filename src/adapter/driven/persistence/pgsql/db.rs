use rocket_sync_db_pools::database;


#[database("my_db")]
pub struct Db(diesel::PgConnection);