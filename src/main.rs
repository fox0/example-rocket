#![feature(proc_macro_hygiene, decl_macro)]

mod views;
mod catchers;

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;


fn main() {
    rocket::ignite()
        .mount("/", routes![views::index, views::get, views::world])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .attach(SpaceHelmet::default())
        // .register(catchers![catchers::not_found])
        .launch();
}

// todo кешировать статику
// todo проверить чтение/запись блобов
// todo генератор/метод для форм
