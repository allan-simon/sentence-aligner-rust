extern crate uuid;

use rocket::Response;
use rocket::http::ContentType;
use rocket::http::Status;

use postgres::error::UNIQUE_VIOLATION;

use std::io::Cursor;

use db;
use sentences::Sentence;

#[post("/languages", format="text/plain", data="<iso639_3>")]
fn create_language<'r>(
    connection: db::DbConnection,
    iso639_3: String,
) -> Response<'r> {

    let result = connection.query(
        r#"
        INSERT INTO language(iso639_3)
        VALUES ($1)
        "#,
        &[&iso639_3],
    );

    if result.is_err() {

        let error = result.unwrap_err();

        if error.code() == Some(&UNIQUE_VIOLATION) {
            return Response::build()
                .status(Status::Conflict)
                .finalize();
        }

        panic!(format!("{}", error));
    }

    Response::build()
        .status(Status::Created)
        .finalize()
}

#[get("/languages/<language_code>/sentences")]
fn get_all_sentences_of_language<'r>(
    connection: db::DbConnection,
    language_code: String,
) -> Response<'r> {

    let result = connection.query(
        r#"
            SELECT
                sentence.id,
                sentence.content,
                iso639_3,
                sentence.structure::text
            FROM language
            JOIN sentence ON (sentence.language_id = language.id)
            WHERE iso639_3 = $1
            ORDER BY
                sentence.added_at,
                sentence.id
            LIMIT 100
        "#,
        &[&language_code],
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
