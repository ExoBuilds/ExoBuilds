#[macro_use]
extern crate rocket;

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
    let mut spellname1 = "Q".into();
    let mut spellname2 = "W".into();
    let mut spellname3 = "E".into();
    let mut spellname4 = "R".into();
    let mut spellmax1 = "ZoeQ".into();
    let mut spellmax2 = "ZoeW".into();
    let mut spellmax3 = "ZoeE".into();
    let mut spellmax4 = "ZoeR".into();
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
    let mut winrate = 50;

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
        winrate = ((champion.wins as f32 / (champion.wins + champion.loses) as f32) * 100.0) as i32;
        spellmax1 = champion.spellpath1;
        spellmax2 = champion.spellpath2;
        spellmax3 = champion.spellpath3;
        spellmax4 = champion.spellpath4;
    }

    Template::render(
        "champions",
        context! {
            name,
            title,
            spellmax1,
            spellname1,
            spellmax2,
            spellname2,
            spellmax3,
            spellname3,
            spellmax4,
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
            role,
            winrate
        },
    )
}

#[get("/profile/<name>")]
fn profile(db: &State<Database>, settings: &State<Settings>, name: &str) -> Template {
    let profile = get_player_profile(settings, &name.to_string());
    if profile.is_err() {
        Template::render("errorprofile", context! {})
    } else {
        let profile = profile.unwrap();

        let summoner_name: String = profile.name;

        let player_matches = db.get_player_matches(&profile.puuid);

        let arrays = player_matches.unwrap();
        let champs = get_most_played_champs(&arrays);
        let icon = get_latest_icon(&arrays);

        Template::render(
            "profile",
            context! {
                summoner_name,
                arrays,
                icon,
                champs,
            },
        )
    }
}

#[post("/profile/<name>")]
fn submit<'r>(db: &State<Database>, settings: &State<Settings>, name: &str) -> Template {
    profile(db, settings, name)
}

#[launch]
fn rocket() -> _ {
    let settings = Settings::init();
    let database = Database::init(&settings);
    thread::spawn({
        let database = database.clone();
        let settings = settings.clone();
        move || initialize_matches(settings, database)
    });
    thread::spawn({
        let database = database.clone();
        move || initialize_champions(database)
    });
    rocket::build()
        .mount("/", routes![index, champions, profile, submit])
        .mount("/", FileServer::from("public/"))
        .manage(database)
        .manage(settings)
        .attach(Template::fairing())
}
