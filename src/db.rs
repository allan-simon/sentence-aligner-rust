
use std::ops::Deref;
use std::env;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use postgres;
use postgres::params::{ConnectParams, Host};

use r2d2;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};


type Pool = r2d2::Pool<PostgresConnectionManager>;


/// Create the connection parameters from environment variables
/// DB_USER
/// DB_PASSWORD
/// DB_HOST
/// DB_NAME
/// we fail if any is missing
fn create_connection_params_from_env() -> ConnectParams {
    ConnectParams::builder()
        .user(
            &env::var("DB_USER").expect("missing DB_USER"),
            Some(&env::var("DB_PASSWORD").expect("missing DB_PASSWORD")),
        )
        .database(&env::var("DB_NAME").expect("missing DB_NAME"))
        .build(Host::Tcp(env::var("DB_HOST").expect("missing DB_HOST")))
}

/// Create the r2d2 connection pool of Postgresql Connection
pub fn init_pool() -> Pool {

    let manager = PostgresConnectionManager::new(
        create_connection_params_from_env(),
        TlsMode::None
    )
    .expect("db manager");

    let config = r2d2::Config::default();

    Pool::new(
        config,
        manager
    )
    .expect("db pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConnection(pub r2d2::PooledConnection<PostgresConnectionManager>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConnection {

    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConnection, ()> {

        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }

    }
}

// For the convenience of using an &DbConn as an &postgres::Connection.
impl Deref for DbConnection {
    type Target = postgres::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
