pub mod user;
use rocket_contrib::databases::diesel;

#[database("database_url")]
pub struct Conn(diesel::PgConnection);


