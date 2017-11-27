extern crate postgres;

use std::env;

use self::postgres::{
    Connection,
    TlsMode,
};
use self::postgres::params::{
    ConnectParams,
    Host,
};

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

/// Wrapper that returns a PostgreSQL connection to the tests environment db
pub fn get_connection() -> Connection {

    return Connection::connect(
        create_connection_params_from_env(),
        TlsMode::None,
    ).unwrap();
}

/// Truncate the content of the unique table 'sentence'
///
/// # Arguments:
///
/// `connection` - The PostgreSQL connection object
pub fn clear(connection: &Connection) {

    connection.execute("TRUNCATE TABLE sentence;", &[]).unwrap();
}
