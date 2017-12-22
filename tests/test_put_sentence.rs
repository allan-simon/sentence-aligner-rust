extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;
use reqwest::header::ContentType;

mod db;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_put_sentence_text_returns_200() {

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

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/text",
        tests_commons::SERVICE_URL,
        sentence_uuid,
    );
    let modified_sentence = "This is a modified sentence.";
    let response = client.put(&url)
        .body(modified_sentence)
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NoContent,
    );

    assert_eq!(
        db::get_sentence(
            &connection,
            &sentence_uuid,
        ),
        modified_sentence,
    );
}

#[test]
fn test_put_sentence_text_that_does_not_exist_returns_404() {

    let connection = db::get_connection();
    db::clear(&connection);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/text",
        tests_commons::SERVICE_URL,
        uuid::Uuid::new_v4(),
    );
    let response = client.put(&url)
        .body("This is a sentence.")
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NotFound,
    );
}

#[test]
fn test_put_sentence_language_returns_200() {

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &sentence_iso639_3,
    );

    let modified_language = "fra";
    db::insert_language(
        &connection,
        &modified_language,
    );

    let sentence_uuid = uuid::Uuid::new_v4();
    let sentence_text = "This is one sentence";
    db::insert_sentence(
        &connection,
        &sentence_uuid,
        &sentence_text,
        &sentence_iso639_3,
    );

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/language",
        tests_commons::SERVICE_URL,
        sentence_uuid,
    );
    let response = client.put(&url)
        .body(modified_language)
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NoContent,
    );

    assert_eq!(
        db::get_language_by_sentence(
            &connection,
            &sentence_uuid,
        ),
        modified_language,
    );
}

#[test]
fn test_put_sentence_language_that_does_not_exist_returns_404() {

    let connection = db::get_connection();
    db::clear(&connection);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/language",
        tests_commons::SERVICE_URL,
        uuid::Uuid::new_v4(),
    );
    let response = client.put(&url)
        .body("fra")
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NotFound,
    );
}

#[test]
fn test_put_sentence_structure_returns_200() {

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

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        sentence_uuid,
    );
    let modified_structure = "<sentence>This is one sentence</sentence>";
    let response = client.put(&url)
        .body(modified_structure)
        .header(ContentType::xml())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NoContent,
    );

    assert_eq!(
        db::get_structure(
            &connection,
            &sentence_uuid,
        ),
        modified_structure,
    );
}

#[test]
fn test_put_sentence_structure_that_does_not_exist_returns_404() {

    let connection = db::get_connection();
    db::clear(&connection);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        uuid::Uuid::new_v4(),
    );
    let response = client.put(&url)
        .body("<sentence>This is one sentence</sentence>")
        .header(ContentType::xml())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NotFound,
    );
}

#[test]
fn test_put_sentence_structure_that_does_not_match_content_returns_400() {

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

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        &sentence_uuid,
    );
    let response = client.put(&url)
        .body("<sentence>Contains different words.</sentence>")
        .header(ContentType::xml())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BadRequest,
    );
}
