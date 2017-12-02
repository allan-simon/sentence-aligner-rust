extern crate reqwest;
extern crate uuid;

use std::collections::HashMap;

use reqwest::StatusCode;

mod db;

#[test]
fn test_post_sentence_returns_200() {

    let connection = db::get_connection();
    db::clear(&connection);

    let mut json = HashMap::new();
    json.insert("text", "This is a sentence.");
    json.insert("iso639_3", "eng");

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8000/sentences")
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Created,
    );
}

#[test]
fn test_post_sentence_with_used_uuid_returns_409() {

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_uuid = uuid::Uuid::new_v4();
    let sentence_text = "This is one sentence";
    let sentence_iso639_3 = "eng";
    db::insert_sentence(
        &connection,
        &sentence_uuid,
        &sentence_text,
        &sentence_iso639_3,
    );

    let mut json = HashMap::new();
    json.insert("id", sentence_uuid.to_string());
    json.insert("text", sentence_text.to_string());
    json.insert("iso639_3", sentence_iso639_3.to_string());

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8000/sentences")
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Conflict,
    );
}
