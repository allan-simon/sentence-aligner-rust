extern crate postgres;
extern crate reqwest;
extern crate uuid;
extern crate interface_tests_helpers;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;

use postgres::Connection;

use interface_tests_helpers::ResponseHandler;

mod db;
mod handlers;

use db::DatabaseHandler;
use handlers::SentenceHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_get_sentence_if_exists_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let text = "This is one sentence";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let client = reqwest::Client::new();
    let mut response = client.get_sentence(&uuid);

    response.assert_200();

    let sentence = response.json::<tests_commons::Sentence>().unwrap();

    assert_eq!(
        sentence.text,
        text,
        "Unexpected sentence text.",
    );

    assert_eq!(
        sentence.iso639_3,
        iso639_3,
        "Unexpected sentence language.",
    );
}

#[test]
fn test_get_sentence_if_does_not_exist_returns_404() {

    let uuid_not_in_database = uuid::Uuid::new_v4();

    let client = reqwest::Client::new();
    let response = client.get_sentence(&uuid_not_in_database);
    response.assert_404();
}
