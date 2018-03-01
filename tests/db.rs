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

    fn assert_sentence_content_equals(&self, uuid: &uuid::Uuid, content: &str);

    fn assert_sentence_language_equals(&self, uuid: &uuid::Uuid, iso639_3: &str);

    fn assert_sentence_structure_is_null(&self, uuid: &uuid::Uuid);
}

impl DatabaseHandler for Connection {

    /// Creates a new connection instance and clean the whole database content
    fn connect_and_clean() -> Connection {

        let builder = ConnectParams::builder()
            .user(
                &env::var("DB_USER").expect("missing DB_USER"),
                Some(&env::var("DB_PASSWORD").expect("missing DB_PASSWORD")),
            )
            .database(&env::var("DB_NAME").expect("missing DB_NAME"))
            .build(Host::Tcp(env::var("DB_HOST").expect("missing DB_HOST")));

        let connection = Connection::connect(
            builder,
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
            "the sentence structure is not the expected one.",
        );
    }

    /// Assertion to check if the content for the sentence at the given id is equals to the expected one
    ///
    /// Args:
    ///
    /// `uuid` - the sentence uuid of the sentence to check
    /// `content` - the expected content for the sentence to check
    fn assert_sentence_content_equals(
        &self,
        uuid: &uuid::Uuid,
        content: &str,
    ) {

        let result = self.query(
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

        /* additional step: inference cannot be performed automatically here */
        let current_content: Option<String> = row.get(0);
        let expected_content = current_content.unwrap();

        assert_eq!(
            content,
            expected_content,
            "the sentence content is not the expected one.",
        );
    }

    /// Assertion to check if the language for the sentence at the given id is equals to the expected one
    ///
    /// Args:
    ///
    /// `uuid` - the sentence uuid of the sentence to check
    /// `iso639_3` - the expected language for the sentence to check
    fn assert_sentence_language_equals(
        &self,
        uuid: &uuid::Uuid,
        iso639_3: &str,
    ) {

        let result = self.query(
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

        /* additional step: inference cannot be performed automatically here */
        let current_iso639_3: Option<String> = row.get(0);
        let expected_iso639_3 = current_iso639_3.unwrap();

        assert_eq!(
            iso639_3,
            expected_iso639_3,
            "the sentence language is not the expected one.",
        );
    }

    /// Assertion that checks if a sentence structure is NULL
    ///
    /// Args:
    ///
    /// `uuid` - the UUID of the sentence to check
    fn assert_sentence_structure_is_null(
        &self,
        uuid: &uuid::Uuid,
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

        assert_eq!(
            current_structure,
            None,
        );
    }
}
