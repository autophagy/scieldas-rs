use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

pub struct TextShield {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub value: String,
}

impl Default for TextShield {
    fn default() -> TextShield {
        TextShield {
            prefix: None,
            suffix: None,
            value: String::from("N/A"),
        }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for TextShield {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let prefix = match &self.prefix {
            Some(s) => format!("{} :: ", s),
            None => String::from(""),
        };
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => String::from(""),
        };
        let value = format!("{}{}{}", prefix, self.value, suffix);
        Response::build()
            .header(ContentType::Plain)
            .sized_body(value.len(), Cursor::new(value))
            .ok()
    }
}
