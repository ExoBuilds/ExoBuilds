#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use std::collections::HashMap;
use rocket::fs::FileServer;

use std::thread;

mod models;
mod database;
use database::*;

mod settings;
use settings::Settings;

mod matches_parser;
use matches_parser::initialize_matches;

mod champion_parser;
use champion_parser::initialize_champions;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/champions/<name>")]
fn champions(name: &str) -> Template {
    Template::render("champions", context! {
        name: name,
        title: "The darkin blade",
        spellmax1: name.to_owned() + "Q",
        spellname1: "A",
        spellmax2: name.to_owned() + "W",
        spellname2: "Z",
        spellmax3: name.to_owned() + "E",
        spellname3: "E",
        spellmax4: name.to_owned() + "R",
        spellname4: "R",
        item0: "3858",
        item1: "3078",
        item2: "3053",
        item3: "3053",
        item4: "3053",
        item5: "3053",
        item6: "3053",
        rune: "Electrocute",
        rune1: "Domination",
        rune2: "Domination",
        summoner1: "Flash",
        summoner2: "Dot",
        role: "jungle"
    })
}

#[launch]
fn rocket() -> _ {
    let mut settings = Settings::init();
    let database = Database::init(&settings);
    let second_database = database.clone();
    thread::spawn(move || {initialize_matches(&mut settings, second_database)});
    thread::spawn(move || {initialize_champions(database)});
    rocket::build()
        .mount("/", routes![index, champions])
        .mount("/", FileServer::from("public/"))
        .attach(Template::fairing())
}
