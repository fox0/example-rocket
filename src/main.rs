#![feature(proc_macro_hygiene, decl_macro)]

mod views_main;
mod forms;
// mod catchers;
// mod cache;

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
// use rocket_contrib::helmet::SpaceHelmet;


fn main() {
    let conn = 42;
    rocket::ignite()
        .manage(conn)
        .mount("/", routes![
            views_main::index,
            views_main::get,
            views_main::world,
            views_main::form_get,
            views_main::form_post,
            ])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        // .attach(SpaceHelmet::default())
        // .attach(cache::HTTPCache::new())
        // .register(catchers![catchers::not_found])
        .launch();
}

// todo проверить чтение/запись блобов
// todo генератор/метод для форм
