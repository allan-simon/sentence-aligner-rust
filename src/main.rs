#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate postgres;
extern crate uuid;
extern crate xml;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;

mod db;
mod cors;
mod sentences;
mod one_sentence;
mod languages;




fn main() {
    let pool = db::init_pool();
    let connection = pool.get().unwrap();
    connection.execute("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"", &[]).unwrap();

    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS language (
            id SERIAL PRIMARY KEY,
            iso639_3 VARCHAR(3) UNIQUE NOT NULL
            CONSTRAINT iso639_3_length CHECK (CHAR_LENGTH(iso639_3) = 3)
        )
        "#,
        &[],
    )
    .expect("can't create table language");

    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS sentence (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            language_id INTEGER REFERENCES language (id) ON DELETE SET NULL,
            added_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            content TEXT NOT NULL,
            structure XML DEFAULT NULL,
            UNIQUE (language_id, content)
        )
        "#,
        &[],
    )
    .expect("can't create table sentence");

    rocket::ignite()
        .attach(cors::CORS())
        .manage(pool)
        .mount(
            "/",
            routes![
                sentences::create_sentence,
                sentences::get_all_sentences,
                sentences::get_all_sentences_with_last_id,
                one_sentence::get_sentence,
                one_sentence::edit_sentence_text,
                one_sentence::edit_sentence_structure,
                one_sentence::edit_sentence_language,
                languages::create_language,
                languages::get_all_sentences_of_language,
            ]
        )
        .launch()
    ;
}
