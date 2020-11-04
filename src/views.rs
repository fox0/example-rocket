use rocket::response::Redirect;
use rocket_contrib::templates::Template;


#[derive(serde::Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>,
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!(get: name = "Unknown"))
}

#[get("/world")]
pub fn world() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
pub fn get(name: String) -> Template {
    let context = TemplateContext { name, items: vec!["One", "Two", "Three"] };
    Template::render("index", &context)
}
