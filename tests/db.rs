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
    connection.execute("TRUNCATE TABLE language;", &[]).unwrap();
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
/// NOTE: allow dead_code to prevent cargo test incorrect warnings
/// (https://github.com/rust-lang/rust/issues/46379)
#[allow(dead_code)]
pub fn insert_sentence(
    connection: &Connection,
    uuid: &uuid::Uuid,
    content: &str,
    iso639_3: &str,
) {

    let _ = connection.execute(
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

/// Returns the sentence text for a given sentence UUID
///
/// # Arguments:
///
/// `connection` - The PostgreSQL connection object
/// `uuid` - The sentence UUID v4
///
/// NOTE: allow dead_code to prevent cargo test incorrect warnings
/// (https://github.com/rust-lang/rust/issues/46379)
#[allow(dead_code)]
pub fn get_sentence(
    connection: &Connection,
    uuid: &uuid::Uuid,
) -> String {

    let result = connection.query(
        r#"
            SELECT content
            FROM sentence
            WHERE id = $1
        "#,
        &[&uuid]
    );

    let rows = result.expect("problem while getting sentence");

    let row = rows
        .iter()
        .next() // there's only 1 result
        .expect("0 results, expected one...")
    ;

    row.get(0)
}

/// Returns the language text for a given sentence UUID
///
/// # Arguments:
///
/// `connection` - The PostgreSQL connection object
/// `uuid` - The sentence UUID v4
///
/// NOTE: allow dead_code to prevent cargo test incorrect warnings
/// (https://github.com/rust-lang/rust/issues/46379)
#[allow(dead_code)]
pub fn get_language(
    connection: &Connection,
    uuid: &uuid::Uuid,
) -> String {

    let result = connection.query(
        r#"
            SELECT iso639_3
            FROM sentence
            WHERE id = $1
        "#,
        &[&uuid]
    );

    let rows = result.expect("problem while getting sentence");

    let row = rows
        .iter()
        .next() // there's only 1 result
        .expect("0 results, expected one...")
    ;

    row.get(0)
}

/// Returns the structure text for a given sentence UUID
///
/// # Arguments:
///
/// `connection` - The PostgreSQL connection object
/// `uuid` - The sentence UUID v4
///
/// NOTE: allow dead_code to prevent cargo test incorrect warnings
/// (https://github.com/rust-lang/rust/issues/46379)
#[allow(dead_code)]
pub fn get_structure(
    connection: &Connection,
    uuid: &uuid::Uuid,
) -> String {

    let result = connection.query(
        r#"
            SELECT structure::TEXT
            FROM sentence
            WHERE id = $1
        "#,
        &[&uuid]
    );

    let rows = result.expect("problem while getting sentence");

    let row = rows
        .iter()
        .next() // there's only 1 result
        .expect("0 results, expected one...")
    ;

    row.get(0)
}

/// Indicates if a language exists in database (iso639_3)
///
/// # Arguments:
///
/// `connection` - The PostgreSQL connection object
/// `iso639_3` - The language
pub fn language_exists(
    connection: &Connection,
    iso639_3: &str,
) -> bool {

    let result = connection.query(
        r#"
            SELECT 1
            FROM language
            WHERE iso639_3 = $1
        "#,
        &[&iso639_3]
    );

    let rows = result.expect("problem while getting language");

    rows.len() == 1
}
