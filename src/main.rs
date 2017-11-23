#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate postgres;

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
        .attach(cors::CORS())
        .manage(pool)
        .mount(
            "/",
            routes![
                sentences::create_sentence,
                sentences::get_all_sentences,
                one_sentence::get_sentence,
                one_sentence::edit_sentence_text,
                one_sentence::edit_sentence_structure,
                one_sentence::edit_sentence_language,
                languages::get_all_sentences_of_language,
            ]
        )
        .launch()
    ;
}
