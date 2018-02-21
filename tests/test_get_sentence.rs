extern crate postgres;
extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;

use postgres::Connection;

mod db;

use db::DatabaseHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_get_sentence_if_exists_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let text = "This is one sentence";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let url = format!(
        "{}/sentences/{}",
        tests_commons::SERVICE_URL,
        uuid.to_string(),
    );
    let mut response = reqwest::get(&url).unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Ok,
    );

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

    let url = format!(
        "{}/sentences/{}",
        tests_commons::SERVICE_URL,
        uuid::Uuid::new_v4().to_string(),
    );
    let response = reqwest::get(&url).unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NotFound,
    );
}
