extern crate postgres;
extern crate reqwest;
extern crate uuid;
extern crate interface_tests_helpers;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;
use reqwest::header::ContentType;

use postgres::Connection;

use interface_tests_helpers::ResponseHandler;

mod db;
mod handlers;

use db::DatabaseHandler;
use handlers::SentenceHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_put_sentence_text_returns_204() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let modified_sentence = "This is a modified sentence.";

    let client = reqwest::Client::new();
    let response = client.update_sentence_text(
        &uuid,
        &modified_sentence,
    );

    response.assert_204();

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
    let mut response = client.update_sentence_text(
        &first_uuid,
        &second_text,
    );

    let sentence = response.json::<tests_commons::Sentence>().unwrap();

    assert_eq!(sentence.text, second_text);
    assert_eq!(sentence.iso639_3, iso_639_3);

    response.assert_409();
}

#[test]
fn test_put_sentence_text_that_does_not_exist_returns_404() {

    let client = reqwest::Client::new();
    let response = client.update_sentence_text(
        &uuid::Uuid::new_v4(),
        "This is a sentence",
    );

    response.assert_404();
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
    let response = client.update_sentence_language(
        &uuid,
        &second_iso639_3,
    );

    response.assert_204();

    connection.assert_sentence_language_equals(
        &uuid,
        &second_iso639_3,
    );
}

#[test]
fn test_put_sentence_language_that_does_not_exist_returns_404() {

    let client = reqwest::Client::new();
    let response = client.update_sentence_language(
        &uuid::Uuid::new_v4(),
        "fra",
    );

    response.assert_404();
}

#[test]
fn test_put_sentence_structure_returns_204() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";

    let uuid = connection.insert_sentence(&text, &iso639_3);

    let modified_structure = "<sentence><subject>This</subject> <verb>is</verb> <complement>one</complement> <complement>sentence.</complement></sentence>";

    let client = reqwest::Client::new();
    let response = client.update_sentence_structure(
        &uuid,
        &modified_structure,
    );

    response.assert_204();

    connection.assert_sentence_structure_equals(
        &uuid,
        &modified_structure,
    );
}

#[test]
fn test_put_sentence_structure_that_does_not_exist_returns_404() {

    let client = reqwest::Client::new();
    let response = client.update_sentence_structure(
        &uuid::Uuid::new_v4(),
        "<sentence><subject>This</subject> <verb>is</verb> <complement>one</complement> <complement>sentence</complement></sentence>",
    );

    response.assert_404();
}

#[test]
fn test_put_sentence_structure_that_does_not_match_content_returns_400() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";
    let uuid = connection.insert_sentence(&text, &iso639_3);

    let client = reqwest::Client::new();
    let response = client.update_sentence_structure(
        &uuid,
        "<sentence><subject>I</subject> <verb>eat</verb> <complement>apple</complement></sentence>",
    );

    response.assert_400();
}

#[test]
fn test_put_sentence_structure_with_untagged_words_returns_204() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let text = "This is one sentence.";
    let iso639_3 = "eng";
    let uuid = connection.insert_sentence(&text, &iso639_3);
    let modified_structure = "<sentence><subject>This</subject> <verb>is</verb> one sentence.</sentence>";

    let client = reqwest::Client::new();
    let response = client.update_sentence_structure(
        &uuid,
        &modified_structure,
    );

    response.assert_204();

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
    let response = client.update_sentence_structure(
        &uuid,
        "<sentence><subject>This</subject><verb>is</verb><complement>one</complement><complement>sentence</complement></sentence>",
    );

    response.assert_400();
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
    let response = client.update_sentence_language(
        &first_sentence_uuid,
        &second_language,
    );

    response.assert_409();
}
