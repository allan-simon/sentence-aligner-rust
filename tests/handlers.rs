extern crate reqwest;
extern crate uuid;
extern crate interface_tests_helpers;

use reqwest::{
    Client,
    Response,
};

use std::collections::HashMap;

use interface_tests_helpers::ClientHandler;

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

    fn get_all_sentences_with_last_uuid(&self, uuid: &uuid::Uuid) -> Response;

    fn get_sentence(&self, uuid: &uuid::Uuid) -> Response;

    fn get_sentences_by_language(&self, iso_639_3: &str) -> Response;

    fn update_sentence_structure(&self, uuid: &uuid::Uuid, structure: &str) -> Response;

    fn update_sentence_text(&self, uuid: &uuid::Uuid, text: &str) -> Response;

    fn update_sentence_language(&self, uuid: &uuid::Uuid, iso_639_3: &str) -> Response;
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

    /// Handles GET all sentences with last_uuid requests.
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn get_all_sentences_with_last_uuid(&self, uuid: &uuid::Uuid) -> Response {

        self.get_url(
            &format!(
                "{}/sentences?starting_after_id={}",
                self.get_base_url(),
                uuid.to_string(),
            )
        )
    }

    /// Handles GET one sentence per UUID request.
    ///
    /// # Args:
    ///
    /// `uuid` - the UUID of the sentence to get
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn get_sentence(&self, uuid: &uuid::Uuid) -> Response {

        self.get_url(
            &format!(
                "{}/sentences/{}",
                self.get_base_url(),
                uuid.to_string(),
            )
        )
    }

    /// Handles GET sentences per language.
    ///
    /// # Args:
    ///
    /// `iso_639_3` - the language to use
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn get_sentences_by_language(&self, iso_639_3: &str) -> Response {

        self.get_url(
            &format!(
                "{}/languages/{}/sentences",
                self.get_base_url(),
                iso_639_3.to_string(),
            )
        )
    }

    /// Handles PUT structure per sentence.
    ///
    /// # Args:
    ///
    /// `uuid` - the UUID of the sentence to update
    /// `structure` - the new structure to apply
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn update_sentence_structure(
        &self,
        uuid: &uuid::Uuid,
        structure: &str,
    ) -> Response {

        self.put_xml(
            &format!(
                "{}/sentences/{}/structure",
                self.get_base_url(),
                uuid.to_string(),
            ),
            structure,
        )
    }

    /// Handles PUT text per sentence.
    ///
    /// # Args:
    ///
    /// `uuid` - the UUID of the sentence to update
    /// `text` - the text to upload
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn update_sentence_text(
        &self,
        uuid: &uuid::Uuid,
        text: &str,
    ) -> Response {

        self.put_text(
            &format!(
                "{}/sentences/{}/text",
                self.get_base_url(),
                uuid.to_string(),
            ),
            text,
        )
    }

    /// Handles PUT language per sentence.
    ///
    /// # Args:
    ///
    /// `uuid` - the UUID of the sentence to update
    /// `iso_639_3` - the language to upload
    ///
    /// # Returns:
    ///
    /// reqwest response
    fn update_sentence_language(
        &self,
        uuid: &uuid::Uuid,
        iso_639_3: &str,
    ) -> Response {

        self.put_text(
            &format!(
                "{}/sentences/{}/language",
                self.get_base_url(),
                uuid.to_string(),
            ),
            iso_639_3,
        )
    }
}
