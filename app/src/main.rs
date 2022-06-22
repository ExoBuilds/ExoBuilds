#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use std::collections::HashMap;
use rocket::fs::FileServer;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/champions/<name>")]
fn champions(name: &str) -> Template {
    Template::render("champions", context! {
        name: name,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, champions])
        .mount("/", FileServer::from("public/"))
        .attach(Template::fairing())
}
