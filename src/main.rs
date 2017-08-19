#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate uuid;
extern crate rocket_contrib;
extern crate postgres;

extern crate r2d2;
extern crate r2d2_postgres;

mod db;

#[macro_use] extern crate serde_derive;


use rocket_contrib::Json;
use rocket::Response;
use rocket::http::Status;
use postgres::error::UNIQUE_VIOLATION;

#[derive(Deserialize)]
struct Sentence {
    id: Option<uuid::Uuid>,
    text: String,
    iso639_3: String,
}

#[post("/sentences", format="application/json", data="<sentence>" )]
fn create_sentence<'r>(
    connection: db::DbConnection,
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
        .raw_header("ContentType", "application/json".to_string())
        .finalize()
}


fn main() {
    let pool = db::init_pool();
    let connection = pool.get().unwrap();
    connection.execute("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"", &[]).unwrap();
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS sentence (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            added_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            content TEXT NOT NULL,
            iso639_3 VARCHAR(3) NOT NULL,
            structure XML DEFAULT NULL,
            UNIQUE(content, iso639_3)
        )
        "#,
        &[],

    )
    .expect("can't create table sentence");

    rocket::ignite()
        .manage(pool)
        .mount(
            "/",
            routes![
                create_sentence
            ]
        )
        .launch()
    ;
}
