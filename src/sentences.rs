extern crate uuid;

use rocket_contrib::UUID;
use rocket::Response;
use rocket_contrib::Json;
use rocket::http::Status;
use postgres::error::UNIQUE_VIOLATION;
use self::uuid::Uuid;
use std::io::Cursor;

use db;

#[derive(Deserialize, Serialize)]
struct Sentence {
    id: Option<uuid::Uuid>,
    text: String,
    iso639_3: String,
}

#[post("/sentences", format="application/json", data="<sentence>")] fn create_sentence<'r>( connection: db::DbConnection,
    sentence: Json<Sentence>
) -> Response<'r> {

    let result = connection.query(
        r#"
        INSERT INTO sentence(
            id,
            content,
            iso639_3
        ) VALUES (
            $1,
            $2,
            $3
        )
        RETURNING id
        "#,
        &[
            &sentence.id.or_else(|| Some(uuid::Uuid::new_v4())),
            &sentence.text,
            &sentence.iso639_3,
        ],
    );

    let rows = match result {
        Ok(rows) => rows,
        Err(ref e) => {
            if e.code() == Some(&UNIQUE_VIOLATION) {

                return  Response::build()
                    .status(Status::Conflict)
                    .finalize()
                ;
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

#[get("/sentences/<sentence_uuid>")]
fn get_sentence<'r>(
    connection: db::DbConnection,
    sentence_uuid: UUID
) -> Response<'r> {
    // little trick needed as we can't directly
    // convert from url's string param to 'standard' uuid
    // we need to go through an intermediate rocket's UUID
    let real_uuid : Uuid = *sentence_uuid;

    let result = connection.query(
        r#"
            SELECT
                id,
                content,
                iso639_3
            FROM sentence
            WHERE id = $1
        "#,
        &[&real_uuid]
    );

    let rows = result.expect("problem while getting sentence");

    if rows.is_empty() {
        return  Response::build()
            .status(Status::NotFound)
            .finalize()
        ;
    }

    let row = rows
        .iter()
        .next() // there's only 1 result
        .expect("0 results, expected one...")
    ;

    let sentence = Sentence {
        id: row.get(0),
        text: row.get(1),
        iso639_3: row.get(2),
    };


    Response::build()
        .sized_body(Cursor::new(json!(sentence).to_string()))
        .finalize()
}

#[get("/sentences")]
fn get_all_sentences<'r>(
    connection: db::DbConnection,
) -> Response<'r> {

    let result = connection.query(
        r#"
            SELECT
                id,
                content,
                iso639_3
            FROM sentence
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
        };
        sentences.push(sentence);
    }

    Response::build()
        .sized_body(Cursor::new(json!(sentences).to_string()))
        .finalize()
}
