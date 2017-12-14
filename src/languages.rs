extern crate uuid;

use rocket::Response;
use rocket::http::ContentType;
use rocket::http::Status;

use std::io::Cursor;

use db;
use sentences::Sentence;

#[post("/languages", format="text/plain", data="<language>")]
fn create_language<'r>(
    connection: db::DbConnection,
    language: String,
) -> Response<'r> {

    return Response::build()
        .status(Status::Created)
        .finalize();
}

#[get("/languages/<language_code>/sentences")]
fn get_all_sentences_of_language<'r>(
    connection: db::DbConnection,
    language_code: String,
) -> Response<'r> {

    let result = connection.query(
        r#"
            SELECT
                id,
                content,
                iso639_3,
                structure::text
            FROM sentence
            WHERE iso639_3 = $1
            ORDER BY
                added_at,
                id
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
