extern crate reqwest;
extern crate uuid;
extern crate postgres;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;

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
