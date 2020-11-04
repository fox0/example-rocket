use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;


#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut context = HashMap::new();
    context.insert("path", req.uri().path());
    Template::render("error/404", &context)
}
