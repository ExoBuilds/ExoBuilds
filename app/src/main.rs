#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/tkt")]
fn tkt() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/<word>")]
fn test(word: &str) -> String {
    format!("J'aime les {}", word)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/arabe", routes![test, tkt])
        .mount("/", routes![index])
        .attach(Template::fairing())
}
