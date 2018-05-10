extern crate reqwest;
extern crate uuid;
extern crate postgres;
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
fn test_get_all_sentences_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let first_iso639_3 = "eng";
    connection.insert_language(&first_iso639_3);

    let second_iso639_3 = "fra";
    connection.insert_language(&second_iso639_3);

    let first_text = "This is one sentence";
    connection.insert_sentence(&first_text, &first_iso639_3);

    let second_text = "This is a second sentence";
    connection.insert_sentence(&second_text, &second_iso639_3);

    let client = reqwest::Client::new();
    let mut response = client.get_all_sentences();
    response.assert_200();

    let sentences = response.json::<tests_commons::Sentences>().unwrap();
    assert_eq!(sentences.len(), 2);
}

#[test]
fn test_get_all_sentences_with_last_id_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let first_iso639_3 = "eng";
    connection.insert_language(&first_iso639_3);

    let first_uuid = uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d10").unwrap();
    connection.insert_sentence_with_uuid(
        &first_uuid,
        "first sentence",
        &first_iso639_3,
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d11").unwrap(),
        "second sentence",
        &first_iso639_3
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d12").unwrap(),
        "third sentence",
        &first_iso639_3
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d13").unwrap(),
        "fourth sentence",
        &first_iso639_3
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d14").unwrap(),
        "fifth sentence",
        &first_iso639_3
    );

    let sixth_uuid = uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d15").unwrap();
    connection.insert_sentence_with_uuid(
        &sixth_uuid,
        "sixth sentence",
        &first_iso639_3
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d16").unwrap(),
        "seventh sentence",
        &first_iso639_3
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d17").unwrap(),
        "eighth sentence",
        &first_iso639_3
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d18").unwrap(),
        "ninth sentence",
        &first_iso639_3
    );
    connection.insert_sentence_with_uuid(
        &uuid::Uuid::parse_str("d5c4807e-0904-4a2a-b50b-9da6b3f83d19").unwrap(),
        "tenth sentence",
        &first_iso639_3
    );

    let client = reqwest::Client::new();
    let mut response = client.get_all_sentences_with_last_uuid(&first_uuid);
    response.assert_200();
    let sentences = response.json::<tests_commons::Sentences>().unwrap();
    assert_eq!(sentences.len(), 10);

    let mut response = client.get_all_sentences_with_last_uuid(&sixth_uuid);
    response.assert_200();
    let sentences = response.json::<tests_commons::Sentences>().unwrap();
    assert_eq!(sentences.len(), 5);
}
