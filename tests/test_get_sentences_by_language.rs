extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;

mod db;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_get_sentence_by_language_returns_200() {

    let connection = db::get_connection();
    db::clear(&connection);

    let first_english_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &first_english_iso639_3,
    );

    let second_english_iso639_3 = "eng";
    db::insert_language(
        &connection,
        &second_english_iso639_3,
    );

    let other_iso639_3 = "fra";
    db::insert_language(
        &connection,
        &other_iso639_3,
    );

    let first_english_uuid = uuid::Uuid::new_v4();
    let first_english_text = "This is one sentence";
    db::insert_sentence(
        &connection,
        &first_english_uuid,
        &first_english_text,
        &first_english_iso639_3,
    );

    let second_english_uuid = uuid::Uuid::new_v4();
    let second_english_text = "This is a second sentence";
    db::insert_sentence(
        &connection,
        &second_english_uuid,
        &second_english_text,
        &second_english_iso639_3,
    );

    let other_uuid = uuid::Uuid::new_v4();
    let other_text = "Ceci est une phrase";
    db::insert_sentence(
        &connection,
        &other_uuid,
        &other_text,
        &other_iso639_3,
    );

    let url = format!(
        "{}/languages/{}/sentences",
        tests_commons::SERVICE_URL,
        first_english_iso639_3.to_string(),
    );
    let mut response = reqwest::get(&url).unwrap();

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
