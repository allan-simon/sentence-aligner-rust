extern crate postgres;
extern crate reqwest;
extern crate uuid;
extern crate interface_tests_helpers;

#[macro_use] extern crate serde_derive;

use postgres::Connection;

use interface_tests_helpers::ResponseHandler;

mod db;
mod handlers;

use db::DatabaseHandler;
use handlers::LanguageHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_post_language_returns_201() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let created_language = "eng";

    let client = reqwest::Client::new();
    let response = client.post_language(&created_language);

    response.assert_201();

    connection.assert_language_exists(&created_language);
}

#[test]
fn test_post_language_that_already_exists_returns_409() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let created_language = "eng";
    connection.insert_language(&created_language);

    let client = reqwest::Client::new();
    let response = client.post_language(&created_language);

    response.assert_409();
}

#[test]
fn test_post_language_with_incorrect_iso639_3_length() {

    let client = reqwest::Client::new();
    let response = client.post_language("fr"); // two characters given, three expected

    response.assert_500();
}
