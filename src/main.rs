#![feature(proc_macro_hygiene, decl_macro)]

mod views;
mod catchers;

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;


fn main() {
    rocket::ignite()
        .mount("/", routes![views::index, views::get])
        .attach(Template::fairing())
        .register(catchers![catchers::not_found])
        .launch();
}
