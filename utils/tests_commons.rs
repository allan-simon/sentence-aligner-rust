//! Common items for interface tests only.

extern crate uuid;

pub const SERVICE_URL: &str = "http://localhost:8000";

pub type Sentences = Vec<Sentence>;

#[derive(Deserialize)]
pub struct Sentence {
    pub id: Option<uuid::Uuid>,
    pub text: String,
    pub iso639_3: String,
    pub structure: Option<String>,
}
