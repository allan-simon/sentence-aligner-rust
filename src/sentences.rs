extern crate uuid;
extern crate xml;

use rocket::Response;
use rocket_contrib::Json;
use rocket::http::{
    Status,
    ContentType,
};
use postgres::error::{
    UNIQUE_VIOLATION,
    FOREIGN_KEY_VIOLATION,
};
use self::xml::reader::EventReader;
use self::xml::reader::XmlEvent::{Characters, Whitespace};

use std::io::Cursor;

use db;

#[derive(Deserialize, Serialize)]
pub struct Sentence {
    pub id: Option<uuid::Uuid>,
    pub text: String,
    pub iso639_3: String,
    pub structure: Option<String>,
}

#[post("/sentences", format="application/json", data="<sentence>")]
fn create_sentence<'r>(
    connection: db::DbConnection,
    sentence: Json<Sentence>
) -> Response<'r> {

    if let Some(ref content) = sentence.structure {

        let parser = EventReader::from_str(&content);
        let mut structure = String::new();

        for word in parser {
            match word {
                Ok(Characters(value)) | Ok(Whitespace(value)) => {
                    structure += &value;
                },
                _ => {}
            }
        }

        if sentence.text != structure {
            return Response::build()
                .status(Status::BadRequest)
                .finalize();
        }
    }

    let result = connection.query(
        r#"
        INSERT INTO sentence(
            id,
            content,
            language_id,
            structure
        ) VALUES (
            $1,
            $2,
            -- the language id is found using coalesce()
            -- in order to force a relation error
            -- if no language is found
            -- (it prevents NULL to be inserted as the sentence language) */
            COALESCE((SELECT id FROM language WHERE iso639_3 = $3), 0),
            $4::TEXT::XML
        )
        RETURNING id
        "#,
        &[
            &sentence.id.or_else(|| Some(uuid::Uuid::new_v4())),
            &sentence.text,
            &sentence.iso639_3,
            &sentence.structure,
        ],
    );

    let rows = match result {
        Ok(rows) => rows,
        Err(ref e) => {

            let error = e.code();
            if error == Some(&UNIQUE_VIOLATION) {
                return Response::build()
                    .status(Status::Conflict)
                    .finalize();
            }
            if error == Some(&FOREIGN_KEY_VIOLATION) {
                return Response::build()
                    .status(Status::BadRequest)
                    .finalize();
            }

            panic!(format!("{}", e));
        }
    };

    let sentence_uuid : uuid::Uuid = rows
        .iter()
        .next() // there's only 1 result
        .expect("0 results, expected one...")
        .get(0)
    ;

    Response::build()
        .status(Status::Created)
        .raw_header("Location", format!("/sentences/{}", sentence_uuid))
        .finalize()
}


#[get("/sentences")]
fn get_all_sentences<'r>(
    connection: db::DbConnection,
) -> Response<'r> {

    let result = connection.query(
        r#"
            SELECT
                sentence.id,
                content,
                language.iso639_3,
                structure::text
            FROM sentence
            JOIN language ON (sentence.language_id = language.id)
            ORDER BY
                added_at,
                sentence.id
            LIMIT 100
        "#,
        &[],
    );

    let rows = result.expect("problem while getting sentence");

    let mut sentences : Vec<Sentence> = Vec::with_capacity(100);


    for row in rows.iter() {
        let sentence = Sentence {
            id: row.get(0),
            text: row.get(1),
            iso639_3: row.get(2),
            structure: row.get(3),
        };
        sentences.push(sentence);
    }

    Response::build()
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json!(sentences).to_string()))
        .finalize()
}

