#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rand;
#[macro_use]
extern crate serde_derive;

use rand::Rng;
use rocket::request::Form;
use rocket_contrib::{Template, Json};
// use serde_derive::Serialize;

#[derive(Serialize)]
struct PageContext {
    title: String,
    body: String,
    name: Option<String>
}

#[derive(FromForm)]
struct MyForm {
    name: String,
}

const QUOTES: &[&'static str] = &[
    "We're entering a new world in which data may be more important than software. -> Tim O'Rail",
    "Software comes from heaven when you have good hardware. -> Ken Olsen",
    "People who are really serious about software should make their own hardware -> Alan Kay",
    "Karma, memory, and desire are just the software of the soul. It's conditioning that the soul undergoes in order to create experience. And it's a cycle. In most people, the cycle is a conditioned response. They do the same things over and over again. -> Deepak Chopra",
    "If we want users to like our software, we should design it to behave like a likeable person -> Alan Cooper"
];

#[get("/")]
fn index() -> String {
    "Yes, Rockeeet!".to_string()
}

#[get("/quote")]
fn rand_quote() -> String {
    let quote = get_random_quote();
    quote.to_string()
}

#[get("/page")]
fn page() -> Template {
    let context = PageContext{
        title: "Random Quote".to_string(),
        body: get_random_quote().to_string(),
        name: None,
    };
    Template::render("index", &context)
}

#[post("/page", data = "<data>")]
fn page_post(data: Form<MyForm>) -> Template {
    let form = data.get();
    let name = form.name.clone();
    let context = PageContext{
        title: "Random Quote".to_string(),
        body: get_random_quote().to_string(),
        name: Some(name),
    };
    Template::render("index", &context)
}

#[derive(Serialize)]
struct APIResponse {
    status: String,
    data: &'static[&'static str],
}

#[get("/quotes")]
fn api_quotes() -> Json<APIResponse> {
    let response = APIResponse{
        status: "ok".to_string(),
        data: QUOTES
    };
    Json(response)
}

fn get_random_quote() -> &'static str {
    let mut rand_gen = rand::thread_rng();
    let selected = rand_gen.gen_range(0, QUOTES.len());

    QUOTES.get(selected).expect("No quote!")
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, rand_quote, page, page_post])
    .mount("/api/v1", routes![api_quotes])
    .attach(Template::fairing())
    .launch();
}
