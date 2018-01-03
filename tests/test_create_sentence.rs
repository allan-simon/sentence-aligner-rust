extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use reqwest::StatusCode;

mod db;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_post_sentence_returns_200() {

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &sentence_iso639_3,
    );

    let mut json = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", &sentence_iso639_3);

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

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_language = "eng";
    db::insert_language(
        &connection,
        &sentence_language,
    );

    let mut json = HashMap::new();
    let sentence_uuid = uuid::Uuid::new_v4();
    json.insert("id", sentence_uuid.to_string());
    json.insert("text", "This is a sentence.".to_string());
    json.insert("iso639_3", sentence_language.to_string());

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
            &sentence_uuid,
        ),
        None,
    );
}

#[test]
fn test_post_sentence_with_structure_returns_200() {

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_language = "eng";
    db::insert_language(
        &connection,
        &sentence_language,
    );

    let mut json = HashMap::new();
    let sentence_uuid = uuid::Uuid::new_v4();
    let sentence_structure = "<sentence><subject>This</subject> <verb>is</verb> <complement>a</complement> <complement>sentence</complement>.</sentence>";
    json.insert("id", sentence_uuid.to_string());
    json.insert("text", "This is a sentence.".to_string());
    json.insert("iso639_3", sentence_language.to_string());
    json.insert("structure", sentence_structure.to_string());

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
            &sentence_uuid,
        ),
        Some(sentence_structure.to_string())
    );
}

#[test]
fn test_post_sentence_with_used_uuid_returns_409() {

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_iso639_3 = "fra";
    db::insert_language(
        &connection,
        &sentence_iso639_3,
    );

    let sentence_uuid = uuid::Uuid::new_v4();
    let sentence_text = "This is one sentence.";
    db::insert_sentence(
        &connection,
        &sentence_uuid,
        &sentence_text,
        &sentence_iso639_3,
    );

    let mut json = HashMap::new();
    json.insert("id", sentence_uuid.to_string());
    json.insert("text", "Une autre phrase.".to_string());
    json.insert("iso639_3", sentence_iso639_3.to_string());

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

    let connection = db::get_connection();
    db::clear(&connection);

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

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &sentence_iso639_3,
    );

    let mut json = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", &sentence_iso639_3);
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

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &sentence_iso639_3,
    );

    let sentence_text = "This is one sentence.";
    db::insert_sentence(
        &connection,
        &uuid::Uuid::new_v4(),
        &sentence_text,
        &sentence_iso639_3,
    );

    let mut json = HashMap::new();
    json.insert("text", sentence_text.to_string());
    json.insert("iso639_3", sentence_iso639_3.to_string());

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

    let connection = db::get_connection();
    db::clear(&connection);

    let first_sentence_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &first_sentence_iso639_3,
    );

    let second_sentence_iso639_3 = "fra";
    db::insert_language(
        &connection,
        &second_sentence_iso639_3,
    );

    let sentence_text = "This is one sentence.";
    db::insert_sentence(
        &connection,
        &uuid::Uuid::new_v4(),
        &sentence_text,
        &first_sentence_iso639_3,
    );

    let mut json = HashMap::new();
    json.insert("text", sentence_text.to_string());
    json.insert("iso639_3", second_sentence_iso639_3.to_string());

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
