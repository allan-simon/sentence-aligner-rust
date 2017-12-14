extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use reqwest::StatusCode;
use reqwest::header::ContentType;

mod db;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_post_language_returns_200() {

    let connection = db::get_connection();
    db::clear(&connection);

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

    assert!(
        db::language_exists(
            &connection,
            &created_language,
        ),
        "The language has not been inserted."
    );
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
