extern crate reqwest;

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
