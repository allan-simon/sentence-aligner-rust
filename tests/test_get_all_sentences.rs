extern crate reqwest;
extern crate uuid;
extern crate postgres;

#[macro_use] extern crate serde_derive;

use reqwest::{
    StatusCode,
    Url,
};

use std::str::FromStr;

use postgres::Connection;

mod db;

use db::DatabaseHandler;

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

    let url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );

    let url = Url::parse_with_params(
        &url,
        &[("id", "00000000-0000-0000-0000-000000000000")],
    ).unwrap();

    let mut response = reqwest::get(url).unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Ok,
    );

    let sentences = response.json::<tests_commons::Sentences>().unwrap();

    assert_eq!(
        sentences.len(),
        2,
    );
}

#[test]
fn test_get_paginated_sentences() {

    let connection = db::get_connection();
    db::clear(&connection);

    let english_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &english_iso639_3,
    );

    let base_url = format!(
        "{}/sentences",
        tests_commons::SERVICE_URL,
    );

    /* insert multiple sentences with different
       content and "consecutives" uuids,
       from a full-zeros uuid to x-x-x-x-...13 */

    let uuid_common = "00000000-0000-0000-0000-0000000000";

    const SENTENCE_MAX_INDEX: usize = 15;
    for id in 1..SENTENCE_MAX_INDEX {

        /* ensure the uuids strings are all valid uuids */

        const UUIDS_PER_SET: usize = 10;
        let uuid = if id >= UUIDS_PER_SET {
            format!("{}{}0", uuid_common, id - UUIDS_PER_SET)
        } else {
            format!("{}0{}", uuid_common, id)
        };

        db::insert_sentence(
            &connection,
            &uuid::Uuid::from_str(&uuid).unwrap(),
            &format!("Sentence {}", id),
            &english_iso639_3,
        );
    }

    let url = Url::parse_with_params(
        &base_url,
        &[("id", "00000000-0000-0000-0000-000000000000")],
    ).unwrap();

    let mut response = reqwest::get(url).unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Ok,
    );

    let sentences = response.json::<tests_commons::Sentences>().unwrap();

    assert_eq!(
        sentences.len(),
        10,
    );

    let last_sentence = sentences.last().unwrap();
    let last_id = last_sentence.id.unwrap().to_string();

    let url = Url::parse_with_params(
        &base_url,
        &[("id", last_id)],
    ).unwrap();

    let mut response = reqwest::get(url).unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Ok,
    );

    let sentences = response.json::<tests_commons::Sentences>().unwrap();

    assert_eq!(
        sentences.len(),
        3,
    );
}
