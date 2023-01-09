extern crate dotenv;

use dotenv::dotenv;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::Request;
use rocket::request::{self, FromRequest};
use rocket::State;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket_sync_db_pools::diesel::PgConnection;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<diesel::PgConnection>>;

#[database("postgres")]
pub struct PostgresDatabaseConnection(diesel::PgConnection);

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

pub fn establish_connection() -> PostgresPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::new(manager).expect(&format!("Error connecting to {}", database_url))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<&State<PostgresPool>>().await.unwrap();
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
