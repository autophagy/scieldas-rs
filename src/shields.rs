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
        let value = svgify(format!("{}{}{}", prefix, self.value, suffix));
        Response::build()
            .header(ContentType::SVG)
            .sized_body(value.len(), Cursor::new(value))
            .ok()
    }
}

fn svgify(s: String) -> String {
    let mut svg: String = "".to_string();
    let width = (s.len() * 7) + 32;

    let head = format!(
        r#"<svg baseProfile="full" height="41px" version="1.1" width="{}px" xmlns="http://www.w3.org/2000/svg" xmlns:ev="http://www.w3.org/2001/xml-events" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
        width
    );
    svg.push_str(&head);

    svg.push_str(r##"<rect fill="#2D2D2D" height="100%" width="100%" x="0" y="0" />"##);
    let b = format!(
        r##"<text fill="#F2F2F2" font-family="Inconsolata, Courier, monospace" font-size="140" textLength="{}" transform="scale(.1)" x="160" y="240">{}</text>"##,
        (width * 10) - 320,
        &s
    );
    svg.push_str(&b);
    svg.push_str("</svg>");
    svg
}
