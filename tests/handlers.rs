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

    fn get_all_sentences(&self) -> Response;
}

impl LanguageHandler for Client {

    /// Handles POST language requests.
    ///
    /// # Args:
    ///
    /// `iso_639_3` - the language to post
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn post_language(&self, iso_639_3: &str) -> Response {

        self.post_body(
            &format!("{}/languages", self.get_base_url()),
            iso_639_3,
        )
    }
}

impl SentenceHandler for Client {

    /// Handles POST sentence requests.
    ///
    /// # Args:
    ///
    /// `json` - sentence structure to post
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn post_sentence(&self, json: &HashMap<&str, &str>) -> Response {

        self.post_json(
            &format!("{}/sentences", self.get_base_url()),
            json,
        )
    }

    /// Handles GET all sentences requests.
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn get_all_sentences(&self) -> Response {

        self.get_url(&format!("{}/sentences", self.get_base_url()))
    }
}
