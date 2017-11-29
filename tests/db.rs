extern crate postgres;
extern crate uuid;

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

/// Inserts one sentence into the table 'sentence'
///
/// # Arguments:
///
/// `connection` - The PostgreSQL connection object
/// `uuid` - The sentence UUID v4
/// `content` - The sentence itself
/// `iso639_3` - The sentence language
///
/// NOTE: allow `unused_must_use` because we don't need to get the result of execute(),
/// allow `dead_code` as this function is only used by the tests
#[allow(unused_must_use, dead_code)]
pub fn insert_sentence(
    connection: &Connection,
    uuid: &uuid::Uuid,
    content: &str,
    iso639_3: &str,
) {

    connection.execute(
        r#"
        INSERT INTO sentence(
            id,
            content,
            iso639_3
        ) VALUES (
            $1,
            $2,
            $3
        )
        "#,
        &[
            &uuid,
            &content,
            &iso639_3,
        ]
    );
}
