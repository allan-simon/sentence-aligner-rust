extern crate uuid;

use rocket::Response;
use rocket::http::Status;
use rocket::http::ContentType;
use rocket_contrib::UUID;
use postgres::error::UNIQUE_VIOLATION;

use self::uuid::Uuid;
use std::io::Cursor;

use db;
use sentences::Sentence;

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
                language.iso639_3,
                structure::text
            FROM sentence
            JOIN language USING (language_id)
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
        structure: row.get(3),
    };


    Response::build()
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json!(sentence).to_string()))
        .finalize()
}

#[put("/sentences/<sentence_uuid>/text", format="text/plain", data="<text>")]
fn edit_sentence_text<'r>(
    connection: db::DbConnection,
    sentence_uuid: UUID,
    text: String,
) -> Response<'r> {

    let real_uuid : Uuid = *sentence_uuid;

    let result = connection.execute(
        r#"
            UPDATE sentence
            SET content = $1
            WHERE id = $2
        "#,
        &[
            &text,
            &real_uuid,
        ],
    );

    let not_found = match result {
        Ok(nbr_row_updated) => nbr_row_updated == 0,
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

    if not_found {
        return  Response::build()
            .status(Status::NotFound)
            .finalize()
        ;
    }

    Response::build()
        .status(Status::NoContent)
        .finalize()
}

#[put("/sentences/<sentence_uuid>/structure", format="text/xml", data="<text>")]
fn edit_sentence_structure<'r>(
    connection: db::DbConnection,
    sentence_uuid: UUID,
    text: String,
) -> Response<'r> {

    let real_uuid : Uuid = *sentence_uuid;

    /* we add ::TEXT::XML because Postgresql query parameters need explicit cast:
       https://github.com/sfackler/rust-postgres/issues/309#issuecomment-351063887 */
    let result = connection.execute(
        r#"
            UPDATE sentence
            SET structure = $1::TEXT::XML
            WHERE id = $2
        "#,
        &[
            &text,
            &real_uuid,
        ],
    );

    let not_found = match result {
        Ok(nbr_row_updated) => nbr_row_updated == 0,
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

    if not_found {
        return  Response::build()
            .status(Status::NotFound)
            .finalize()
        ;
    }

    Response::build()
        .status(Status::NoContent)
        .finalize()
}

#[put("/sentences/<sentence_uuid>/language", format="text/plain", data="<text>")]
fn edit_sentence_language<'r>(
    connection: db::DbConnection,
    sentence_uuid: UUID,
    text: String,
) -> Response<'r> {

    let real_uuid : Uuid = *sentence_uuid;

    let result = connection.execute(
        r#"
            UPDATE sentence
            SET language_id = (
                SELECT language_id
                FROM language
                WHERE iso639_3 = $1
            )
            WHERE id = $2
        "#,
        &[
            &text,
            &real_uuid,
        ],
    );

    let status = match result {
        Ok(nbr_row_updated) if nbr_row_updated == 1 => {
            Status::NoContent
        },
        Ok(_) => {
            Status::NotFound
        },
        Err(ref e) => {
            panic!(format!("{}", e));
        }
    };

    return Response::build()
        .status(status)
        .finalize()
}
