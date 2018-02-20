extern crate postgres;
extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use reqwest::StatusCode;

use postgres::Connection;

mod db;

use db::DatabaseHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_post_sentence_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let mut json = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", &iso639_3);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Created,
    );
}

#[test]
fn test_post_sentence_without_structure_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let language = "eng";
    connection.insert_language(&language);

    let mut json = HashMap::new();
    let uuid = uuid::Uuid::new_v4();
    json.insert("id", uuid.to_string());
    json.insert("text", "This is a sentence.".to_string());
    json.insert("iso639_3", language.to_string());

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Created,
    );

    assert_eq!(
        db::get_structure(
            &connection,
            &uuid,
        ),
        None,
    );
}

#[test]
fn test_post_with_structure_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let language = "eng";
    connection.insert_language(&language);

    let mut json = HashMap::new();
    let uuid = uuid::Uuid::new_v4();
    let structure = "<sentence><subject>This</subject> <verb>is</verb> <complement>a</complement> <complement>sentence</complement>.</sentence>";
    json.insert("id", uuid.to_string());
    json.insert("text", "This is a sentence.".to_string());
    json.insert("iso639_3", language.to_string());
    json.insert("structure", structure.to_string());

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Created,
    );

    assert_eq!(
        db::get_structure(
            &connection,
            &uuid,
        ),
        Some(structure.to_string())
    );
}

#[test]
fn test_post_sentence_with_used_uuid_returns_409() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let text = "This is one sentence.";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let mut json = HashMap::new();
    json.insert("id", uuid.to_string());
    json.insert("text", "Une autre phrase.".to_string());
    json.insert("iso639_3", iso639_3.to_string());

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Conflict,
    );
}

#[test]
fn test_post_sentence_with_non_existing_language_returns_400() {

    let _: Connection = DatabaseHandler::connect_and_clean();

    let mut json = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", "eng");

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BadRequest,
    );
}

#[test]
fn test_post_sentence_structure_that_does_not_match_content_returns_400() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let mut json = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", &iso639_3);
    json.insert("structure", "<sentence>Not matching structure.</sentence>");

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BadRequest,
    );
}

#[test]
fn test_post_sentence_with_used_content_and_language_returns_409() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso639_3 = "eng";
    connection.insert_language(&iso639_3);

    let text = "This is one sentence.";
    connection.insert_sentence(&text, &iso639_3);

    let mut json = HashMap::new();
    json.insert("text", text.to_string());
    json.insert("iso639_3", iso639_3.to_string());

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Conflict,
    );
}

#[test]
fn test_post_sentence_with_used_content_and_different_language_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let first_iso639_3 = "eng";
    connection.insert_language(&first_iso639_3);

    let second_iso639_3 = "fra";
    connection.insert_language(&second_iso639_3);

    let text = "This is one sentence.";
    connection.insert_sentence(&text, &first_iso639_3);

    let mut json = HashMap::new();
    json.insert("text", text.to_string());
    json.insert("iso639_3", second_iso639_3.to_string());

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );
    let response = client.post(&url)
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Created,
    );
}
