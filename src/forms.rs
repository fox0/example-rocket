use serde::Serializer;
use serde::ser::Serialize;

/// тестовая форма
#[derive(FromForm)]
pub struct MyForm {
    pub field: String,
}

impl MyForm {
    pub fn new() -> MyForm {
        MyForm { field: String::new() }
    }
}

impl Serialize for MyForm {
    /// хитрим и рендерим форму для шаблона
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.field.as_str())  //todo + raw_template
        //https://tera.netlify.app/docs/#loading-templates-from-strings
    }
}
