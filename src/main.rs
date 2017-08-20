#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate postgres;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;

mod db;
mod sentences;




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
                sentences::create_sentence,
                sentences::get_sentence,
                sentences::get_all_sentences,
                sentences::edit_sentence_text,
                sentences::edit_sentence_structure,
            ]
        )
        .launch()
    ;
}
