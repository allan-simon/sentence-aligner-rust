extern crate reqwest;
extern crate uuid;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;

mod db;

#[derive(Deserialize)]
pub struct Sentence {
    pub id: Option<uuid::Uuid>,
    pub text: String,
    pub iso639_3: String,
    pub structure: Option<String>,
}

static SERVICE_URL: &'static str = "http://localhost:8000";

#[test]
fn test_get_sentence_if_exists_returns_200() {

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

    let url = format!(
        "{}/sentences/{}",
        SERVICE_URL,
        sentence_uuid.to_string(),
    );
    let mut response = reqwest::get(&url).unwrap();

    assert_eq!(
        response.status(),
        StatusCode::Ok,
    );

    let sentence = response.json::<Sentence>().unwrap();

    assert_eq!(
        sentence.text,
        sentence_text,
        "Unexpected sentence text.",
    );

    assert_eq!(
        sentence.iso639_3,
        sentence_iso639_3,
        "Unexpected sentence language.",
    );
}

#[test]
fn test_get_sentence_if_does_not_exist_returns_404() {

    let connection = db::get_connection();
    db::clear(&connection);

    let sentence_uuid = uuid::Uuid::new_v4();

    let url = format!(
        "{}/sentences/{}",
        SERVICE_URL,
        sentence_uuid.to_string(),
    );
    let response = reqwest::get(&url).unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NotFound,
    );
}
