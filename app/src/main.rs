#[macro_use]
extern crate rocket;

use models::match_history_model::MatchHistory;
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

use std::thread;

mod utils;
use utils::*;

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
fn champions(db: &State<Database>, name: &str) -> Template {
    let champion = db.get_recommended_champion(name);

    let mut title = "The darkin blade".into();
    let mut spellname1 = "A".into();
    let mut spellname2 = "Z".into();
    let mut spellname3 = "E".into();
    let mut spellname4 = "R".into();
    let mut item0 = "7050".into();
    let mut item1 = "7050".into();
    let mut item2 = "7050".into();
    let mut item3 = "7050".into();
    let mut item4 = "7050".into();
    let mut item5 = "7050".into();
    let mut item6 = "7050".into();
    let mut rune = "Electrocute".into();
    let mut rune1 = "Domination".into();
    let mut rune2 = "Domination".into();
    let mut summoner1 = "Flash".into();
    let mut summoner2 = "Dot".into();
    let mut role = "JUNGLE".into();

    if champion.is_ok() {
        let champion = champion.unwrap();

        title = champion.title;
        spellname1 = champion.spellmax1;
        spellname2 = champion.spellmax2;
        spellname3 = champion.spellmax3;
        spellname4 = champion.spellmax4;
        if champion.item0 != 0 {
            item0 = champion.item0.to_string();
        }
        if champion.item1 != 0 {
            item1 = champion.item1.to_string();
        }
        if champion.item2 != 0 {
            item2 = champion.item2.to_string();
        }
        if champion.item3 != 0 {
            item3 = champion.item3.to_string();
        }
        if champion.item4 != 0 {
            item4 = champion.item4.to_string();
        }
        if champion.item5 != 0 {
            item5 = champion.item5.to_string();
        }
        if champion.item6 != 0 {
            item6 = champion.item6.to_string();
        }
        rune = champion.rune;
        rune1 = champion.rune1;
        rune2 = champion.rune2;
        summoner1 = champion.summoner1;
        summoner2 = champion.summoner2;
        role = champion.role;
    }

    Template::render(
        "champions",
        context! {
            name,
            title,
            spellmax1: name.to_owned() + &spellname1,
            spellname1,
            spellmax2: name.to_owned() + &spellname2,
            spellname2,
            spellmax3: name.to_owned() + &spellname3,
            spellname3,
            spellmax4: name.to_owned() + &spellname4,
            spellname4,
            item0,
            item1,
            item2,
            item3,
            item4,
            item5,
            item6,
            rune,
            rune1,
            rune2,
            summoner1,
            summoner2,
            role
        },
    )
}

#[get("/profile/<name>")]
fn profile(db: &State<Database>, name: &str) -> Template {
    let puuid = "Y22N0dvmtG6NsF5GTpPJ4yhxI2t3zMvP5solMwWSqj1Ld-YAijBqMG5bDP9xYZ9EgVkyxiyifsMC_Q";

    let player_matches = db.get_player_matches(puuid.clone());

    let mut icon: String = "4603".into();

    let mut arrays: Vec<MatchHistory> = Vec::new();
    let mut champs: Vec<(String, String, String)> = vec![("Zyra".into(), "10".into(), "50".into())];

    if player_matches.is_ok() {
        arrays = player_matches.unwrap();
        champs = get_most_played_champs(&arrays);
        icon = get_latest_icon(&arrays);
    }

    Template::render(
        "profile",
        context! {
            name,
            arrays,
            icon,
            champs,
        },
    )
}

#[launch]
fn rocket() -> _ {
    let mut settings = Settings::init();
    let database = Database::init(&settings);
    let tmp_db1 = database.clone();
    let tmp_db2 = database.clone();
    thread::spawn(move || initialize_matches(&mut settings, tmp_db1));
    thread::spawn(move || initialize_champions(tmp_db2));
    rocket::build()
        .mount("/", routes![index, champions, profile])
        .mount("/", FileServer::from("public/"))
        .manage(database)
        .attach(Template::fairing())
}
