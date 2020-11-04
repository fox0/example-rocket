use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

use crate::forms::MyForm;


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
pub fn world(conn: State<i32>) -> String /*&'static str*/ {
    format!("Hello, world! {}", conn.inner())
}

#[get("/hello/<name>")]
pub fn get(name: String) -> Template {
    let context = TemplateContext { name, items: vec!["One", "Two", "Three"] };
    Template::render("index", &context)
}

#[get("/form")]
pub fn form_get() -> Template {
    let mut context = Context::new();
    context.insert("form", &MyForm::new());
    Template::render("form", &context)
}

#[post("/form", data = "<form>")]
pub fn form_post(form: Form<MyForm>) -> Template {
    // form.into_inner().field
    let mut context = Context::new();
    context.insert("form", &form.into_inner());
    Template::render("form", &context)
}
