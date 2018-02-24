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

pub trait DatabaseHandler {

    fn connect_and_clean() -> Self;

    fn insert_language(&self, iso639_3: &str);

    fn insert_sentence(&self, content: &str, iso639_3: &str) -> uuid::Uuid;

    fn assert_language_exists(&self, iso639_3: &str);

    fn assert_sentence_structure_equals(&self, uuid: &uuid::Uuid, structure: &str);
}

impl DatabaseHandler for Connection {

    /// Creates a new connection instance and clean the whole database content
    fn connect_and_clean() -> Connection {

        let connection = Connection::connect(
            create_connection_params_from_env(),
            TlsMode::None,
        ).unwrap();

        connection.execute("TRUNCATE TABLE sentence;", &[]).unwrap();
        connection.execute("TRUNCATE TABLE language CASCADE;", &[]).unwrap();

        connection
    }

    /// Inserts a language with a given iso639_3 name
    ///
    /// Args:
    ///
    /// `iso639_3` - the iso639_3 name of the language to insert
    fn insert_language(
        &self,
        iso639_3: &str,
    ) {

        let _ = self.execute(
            r#"
            INSERT INTO language(iso639_3)
            VALUES ($1)
            "#,
            &[&iso639_3]
        )
        .expect("problem while inserting language");
    }

    /// Inserts a sentence with the given id, content and language
    ///
    /// Args:
    ///
    /// `content` - the content of the language to insert
    /// `iso639_3` - the iso639_3 name of the language to insert
    ///
    /// Returns:
    ///
    /// the generated uuid of the inserted sentence
    fn insert_sentence(
        &self,
        content: &str,
        iso639_3: &str,
    ) -> uuid::Uuid {

        let result = self.query(
            r#"
            INSERT INTO sentence(
                content,
                language_id
            ) VALUES (
                $1,
                (SELECT id FROM language WHERE iso639_3 = $2)
            )
            RETURNING id
            "#,
            &[
                &content,
                &iso639_3,
            ]
        );

        let rows = result.expect("problem while inserting sentence");

        let sentence_uuid: uuid::Uuid = rows
            .iter()
            .next()
            .expect("0 result, expected one")
            .get(0);

        sentence_uuid
    }

    /// Assertion to check if a given language exists from its iso639_3 name
    ///
    /// Args:
    ///
    /// `iso639_3` - the iso639_3 name of the language that must exists
    fn assert_language_exists(
        &self,
        iso639_3: &str,
    ) {

        let rows = self.query(
            r#"
                SELECT 1
                FROM language
                WHERE iso639_3 = $1
            "#,
            &[&iso639_3]
        )
        .expect("problem while getting language");

        assert_eq!(rows.len(), 1, "The language does not exist.");
    }

    /// Assertion to check if the structure for the sentence at the given id is equals to the expected one
    ///
    /// Args:
    ///
    /// `uuid` - the sentence uuid of the sentence to check
    /// `structure` - the expected structure for the sentence to check
    fn assert_sentence_structure_equals(
        &self,
        uuid: &uuid::Uuid,
        structure: &str,
    ) {

        let result = self.query(
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

        /* additional step: inference cannot be performed automatically here */
        let current_structure: Option<String> = row.get(0);
        let expected_structure = current_structure.unwrap();

        assert_eq!(
            structure,
            expected_structure,
            "The sentence structure is not the expected one.",
        );
    }
}

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
    connection.execute("TRUNCATE TABLE language CASCADE;", &[]).unwrap();
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
            language_id
        ) VALUES (
            $1,
            $2,
            (SELECT id FROM language WHERE iso639_3 = $3)
        )
        "#,
        &[
            &uuid,
            &content,
            &iso639_3,
        ]
    );
}

/// Inserts one language into the table 'language'
///
/// # Arguments:
///
/// `connection` - The PostgreSQL connection object
/// `iso639_3` - The language
///
/// NOTE: allow dead_code to prevent cargo test incorrect warnings
/// (https://github.com/rust-lang/rust/issues/46379)
#[allow(dead_code)]
pub fn insert_language(
    connection: &Connection,
    iso639_3: &str,
) {

    let _ = connection.execute(
        r#"
        INSERT INTO language(iso639_3)
        VALUES ($1)
        "#,
        &[&iso639_3]
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
pub fn get_language_by_sentence(
    connection: &Connection,
    uuid: &uuid::Uuid,
) -> String {

    let result = connection.query(
        r#"
            SELECT language.iso639_3
            FROM sentence
            JOIN language ON (sentence.language_id = language.id)
            WHERE sentence.id = $1
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
) -> Option<String> {

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
///
/// NOTE: allow dead_code to prevent cargo test incorrect warnings
/// (https://github.com/rust-lang/rust/issues/46379)
#[allow(dead_code)]
pub fn language_exists(
    connection: &Connection,
    iso639_3: &str,
) -> bool {

    let rows = connection.query(
        r#"
            SELECT 1
            FROM language
            WHERE iso639_3 = $1
        "#,
        &[&iso639_3]
    )
    .expect("problem while getting language");

    rows.len() == 1
}
