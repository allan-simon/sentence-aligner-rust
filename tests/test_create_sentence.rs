extern crate postgres;
extern crate reqwest;
extern crate uuid;
extern crate interface_tests_helpers;

#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use postgres::Connection;

use interface_tests_helpers::ResponseHandler;

mod db;
mod handlers;

use db::DatabaseHandler;
use handlers::SentenceHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_post_sentence_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", &iso639_3);

    let client = reqwest::Client::new();
    let response = client.post_sentence(&json);

    response.assert_201();
}

#[test]
fn test_post_sentence_without_structure_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let language = "eng";
    connection.insert_language(&language);

    let uuid = uuid::Uuid::new_v4();
    let uuid_as_string = uuid.to_string();

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("id", &uuid_as_string);
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", language);

    let client = reqwest::Client::new();
    let response = client.post_sentence(&json);

    response.assert_201();

    connection.assert_sentence_structure_is_null(&uuid);
}

#[test]
fn test_post_with_structure_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let language = "eng";
    connection.insert_language(&language);

    let uuid = uuid::Uuid::new_v4();
    let uuid_as_string = uuid.to_string();

    let mut json: HashMap<&str, &str> = HashMap::new();

    let structure = "<sentence><subject>This</subject> <verb>is</verb> <complement>a</complement> <complement>sentence</complement>.</sentence>";
    json.insert("id", &uuid_as_string);
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", language);
    json.insert("structure", structure);

    let client = reqwest::Client::new();
    let response = client.post_sentence(&json);

    response.assert_201();

    connection.assert_sentence_structure_equals(
        &uuid,
        &structure,
    );
}

#[test]
fn test_post_sentence_with_used_uuid_returns_409() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let text = "This is one sentence.";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let uuid_string = uuid.to_string();

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("id", &uuid_string);
    json.insert("text", "Une autre phrase.");
    json.insert("iso639_3", iso639_3);

    let client = reqwest::Client::new();
    let mut response = client.post_sentence(&json);

    let sentence = response.json::<tests_commons::Sentence>().unwrap();

    assert_eq!(sentence.text, text);
    assert_eq!(sentence.iso639_3, iso639_3);

    response.assert_409();
}

#[test]
fn test_post_sentence_with_non_existing_language_returns_400() {

    let _: Connection = DatabaseHandler::connect_and_clean();

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", "eng");

    let client = reqwest::Client::new();
    let response = client.post_sentence(&json);

    response.assert_400();
}

#[test]
fn test_post_sentence_structure_that_does_not_match_content_returns_400() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", &iso639_3);
    json.insert("structure", "<sentence>Not matching structure.</sentence>");

    let client = reqwest::Client::new();
    let response = client.post_sentence(&json);

    response.assert_400();
}

#[test]
fn test_post_sentence_with_used_content_and_language_returns_409() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let text = "This is one sentence.";
    connection.insert_sentence(&text, &iso639_3);

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("text", text);
    json.insert("iso639_3", iso639_3);

    let client = reqwest::Client::new();
    let mut response = client.post_sentence(&json);

    let sentence = response.json::<tests_commons::Sentence>().unwrap();

    assert_eq!(sentence.text, text);
    assert_eq!(sentence.iso639_3, iso639_3);

    response.assert_409();
}

#[test]
fn test_post_sentence_with_used_content_and_different_language_returns_201() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let first_iso639_3 = "eng";
    connection.insert_language(&first_iso639_3);

    let second_iso639_3 = "fra";
    connection.insert_language(&second_iso639_3);

    let text = "This is one sentence.";
    connection.insert_sentence(&text, &first_iso639_3);

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("text", text);
    json.insert("iso639_3", second_iso639_3);

    let client = reqwest::Client::new();
    let response = client.post_sentence(&json);

    response.assert_201();
}
