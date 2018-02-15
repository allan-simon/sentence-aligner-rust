extern crate postgres;
extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;
use reqwest::header::ContentType;

use postgres::Connection;

use db::DatabaseHandler;

/* TODO: #52 apply the Connection encapsulation to the whole file,
   so we can then simply get rid of this direct module inclusion */
mod db;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_post_language_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let client = reqwest::Client::new();

    let url = format!(
        "{}/languages",
        tests_commons::SERVICE_URL,
    );
    let created_language = "eng";
    let response = client.post(&url)
        .body(created_language)
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Created,
    );

    connection.assert_language_exists(&created_language);
}

#[test]
fn test_post_language_that_already_exists_returns_409() {

    let connection = db::get_connection();
    db::clear(&connection);

    let created_language = "eng";
    db::insert_language(
        &connection,
        &created_language,
    );

    let client = reqwest::Client::new();

    let url = format!(
        "{}/languages",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .body(created_language)
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Conflict,
    );
}

#[test]
fn test_post_language_with_incorrect_iso639_3_length() {

    let connection = db::get_connection();
    db::clear(&connection);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/languages",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .body("fr") // two characters given, three expected
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::InternalServerError,
    );
}
