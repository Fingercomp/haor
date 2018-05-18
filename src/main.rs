#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;

use rocket_contrib::Template;

type TemplateContext = HashMap<String, String>;

#[get("/")]
fn index() -> Template {
    Template::render("index", &TemplateContext::new())
}

#[get("/setup/rust")]
fn setup_rust() -> Template {
    Template::render("setup-rust", &TemplateContext::new())
}

#[get("/setup/haor")]
fn setup_haor() -> Template {
    Template::render("setup-haor", &TemplateContext::new())
}

#[get("/copyright")]
fn copyright() -> Template {
    Template::render("copyright", &TemplateContext::new())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, setup_rust, setup_haor, copyright])
        .attach(Template::fairing())
        .launch();
}
