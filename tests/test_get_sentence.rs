extern crate reqwest;

use reqwest::StatusCode;

#[test]
fn test_get_sentence_returns_200() {

    let response = reqwest::get("http://localhost:8000/sentences").unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Ok,
    );
}