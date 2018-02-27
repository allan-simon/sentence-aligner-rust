extern crate postgres;
extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;
use reqwest::header::ContentType;

use postgres::Connection;

mod db;

use db::DatabaseHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_put_sentence_text_returns_204() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/text",
        tests_commons::SERVICE_URL,
        uuid,
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

    connection.assert_sentence_content_equals(
        &uuid,
        &modified_sentence,
    );
}

#[test]
fn test_put_sentence_text_if_text_already_used_returns_409() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let iso_639_3 = "eng";
    connection.insert_language(&iso_639_3);

    let first_text = "This is the first sentence content.";
    let first_uuid = connection.insert_sentence(&first_text, &iso_639_3);

    let second_text = "This is the second sentence content.";
    connection.insert_sentence(&second_text, &iso_639_3);

    let client = reqwest::Client::new();

    /* modifies the first sentence with the second sentence
       content in order to trigger a conflict */
    let url = format!(
        "{}/sentences/{}/text",
        tests_commons::SERVICE_URL,
        first_uuid,
    );
    let response = client.put(&url)
        .body(second_text)
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Conflict,
    );
}

#[test]
fn test_put_sentence_text_that_does_not_exist_returns_404() {

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
fn test_put_sentence_language_returns_204() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let first_iso639_3 = "eng";
    connection.insert_language(&first_iso639_3);

    let text = "This is one sentence.";
    let uuid = connection.insert_sentence(&text, &first_iso639_3);

    let second_iso639_3 = "fra";
    connection.insert_language(&second_iso639_3);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/language",
        tests_commons::SERVICE_URL,
        uuid,
    );
    let response = client.put(&url)
        .body(second_iso639_3)
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NoContent,
    );

    connection.assert_sentence_language_equals(
        &uuid,
        &second_iso639_3,
    );
}

#[test]
fn test_put_sentence_language_that_does_not_exist_returns_404() {

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
fn test_put_sentence_structure_returns_204() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";

    let uuid = connection.insert_sentence(&text, &iso639_3);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        uuid,
    );
    let modified_structure = "<sentence><subject>This</subject> <verb>is</verb> <complement>one</complement> <complement>sentence.</complement></sentence>";
    let response = client.put(&url)
        .body(modified_structure)
        .header(ContentType::xml())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NoContent,
    );

    connection.assert_sentence_structure_equals(
        &uuid,
        &modified_structure,
    );
}

#[test]
fn test_put_sentence_structure_that_does_not_exist_returns_404() {

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        uuid::Uuid::new_v4(),
    );
    let response = client.put(&url)
        .body("<sentence><subject>This</subject> <verb>is</verb> <complement>one</complement> <complement>sentence</complement></sentence>")
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

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        &uuid,
    );
    let response = client.put(&url)
        .body("<sentence><subject>I</subject> <verb>eat</verb> <complement>apple</complement></sentence>")
        .header(ContentType::xml())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BadRequest,
    );
}

#[test]
fn test_put_sentence_structure_with_untagged_words_returns_204() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        &uuid,
    );
    let modified_structure = "<sentence><subject>This</subject> <verb>is</verb> one sentence.</sentence>";
    let response = client.put(&url)
        .body(modified_structure)
        .header(ContentType::xml())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NoContent,
    );

    connection.assert_sentence_structure_equals(
        &uuid,
        &modified_structure,
    );
}

#[test]
fn test_put_sentence_structure_with_spaces_that_do_not_match_content() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let client = reqwest::Client::new();

    let url = format!(
        "{}/sentences/{}/structure",
        tests_commons::SERVICE_URL,
        &uuid,
    );
    let response = client.put(&url)
        .body("<sentence><subject>This</subject><verb>is</verb><complement>one</complement><complement>sentence</complement></sentence>")
        .header(ContentType::xml())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BadRequest,
    );
}

#[test]
fn test_put_sentence_language_if_language_already_used_returns_409() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let first_language = "eng";
    connection.insert_language(&first_language);

    let second_language = "fra";
    connection.insert_language(&second_language);

    let common_text = "This is a common text.";

    let first_sentence_uuid = connection.insert_sentence(
        &common_text,
        &first_language,
    );

    connection.insert_sentence(
        &common_text,
        &second_language,
    );

    let client = reqwest::Client::new();

    /* modifies the first sentence with the second sentence
       language in order to trigger a conflict */
    let url = format!(
        "{}/sentences/{}/language",
        tests_commons::SERVICE_URL,
        first_sentence_uuid,
    );
    let response = client.put(&url)
        .body(second_language)
        .header(ContentType::plaintext())
        .send()
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Conflict,
    );
}
