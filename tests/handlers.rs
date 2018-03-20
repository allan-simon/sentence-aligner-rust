extern crate reqwest;
extern crate rust_interface_tests_helper;

use reqwest::{
    Client,
    Response,
};

use std::collections::HashMap;

use rust_interface_tests_helper::ClientHandler;

pub trait HasBaseUrl {

    fn get_base_url(&self) -> &str;
}

impl HasBaseUrl for Client {

    /// Returns the service base URL.
    fn get_base_url(&self) -> &str {
        "http://localhost:8000"
    }
}

pub trait LanguageHandler {

    fn post_language(&self, iso_639_3: &str) -> Response;
}

pub trait SentenceHandler {

    fn post_sentence(&self, json: &HashMap<&str, &str>) -> Response;
}

impl LanguageHandler for Client {

    fn post_language(&self, iso_639_3: &str) -> Response {

        self.post_body(
            &format!("{}/languages", self.get_base_url()),
            iso_639_3,
        )
    }
}

impl SentenceHandler for Client {

    fn post_sentence(&self, json: &HashMap<&str, &str>) -> Response {

        self.post_json(
            &format!("{}/sentences", self.get_base_url()),
            json,
        )
    }
}
