use std::collections::HashMap;
use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::{FromParam, Request};
use rocket::response::{self, Responder, Response};

use cached::proc_macro::cached;
use cached::SizedCache;

use std::path::PathBuf;

pub enum SupportedFiletype {
    Png,
    Svg,
    Txt,
}

pub struct ShieldRequest {
    pub body: String,
    pub filetype: SupportedFiletype,
}

#[derive(Debug)]
pub enum ShieldRequestError {
    InvalidBody,
    InvalidFiletype,
}

impl<'r> FromParam<'r> for ShieldRequest {
    type Error = ShieldRequestError;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let path: PathBuf = PathBuf::from(param);
        let stem = path.file_stem();

        if let Some(b) = stem {
            let body = String::from(b.to_str().unwrap());
            if param.ends_with(".png") {
                Ok(ShieldRequest {
                    body,
                    filetype: SupportedFiletype::Png,
                })
            } else if param.ends_with(".svg") {
                Ok(ShieldRequest {
                    body,
                    filetype: SupportedFiletype::Svg,
                })
            } else if param.ends_with(".txt") {
                Ok(ShieldRequest {
                    body,
                    filetype: SupportedFiletype::Txt,
                })
            } else {
                Err(ShieldRequestError::InvalidFiletype)
            }
        } else {
            Err(ShieldRequestError::InvalidBody)
        }
    }
}

pub trait RenderableShield {
    fn render(&self) -> String;
}

pub struct TextShield {
    pub prefix: String,
    pub suffix: Option<String>,
    pub value: String,
    pub filetype: SupportedFiletype,
}

impl Default for TextShield {
    fn default() -> TextShield {
        TextShield {
            prefix: String::from("!"),
            suffix: None,
            value: String::from("N/A"),
            filetype: SupportedFiletype::Txt,
        }
    }
}

impl RenderableShield for TextShield {
    fn render(&self) -> String {
        let prefix = format!("{} :: ", &self.prefix);
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => String::from(""),
        };
        format!("{}{}{}", prefix, self.value, suffix)
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for TextShield {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let value = self.render();
        render_for_filetype(value, self.filetype)
    }
}

pub struct StateShield {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub value: String,
    pub states: HashMap<String, String>,
    pub filetype: SupportedFiletype,
}

impl Default for StateShield {
    fn default() -> StateShield {
        StateShield {
            prefix: None,
            suffix: None,
            value: "".to_string(),
            states: HashMap::from([("".to_string(), "N/A".to_string())]),
            filetype: SupportedFiletype::Txt,
        }
    }
}

impl RenderableShield for StateShield {
    fn render(&self) -> String {
        let prefix = match &self.prefix {
            Some(s) => format!("{} :: ", &s),
            None => "".to_string(),
        };
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => "".to_string(),
        };
        let value = match self.states.get(&self.value) {
            Some(v) => v,
            None => "N/A",
        };
        format!("{}{}{}", prefix, value, suffix)
    }
}
#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for StateShield {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let value = self.render();
        render_for_filetype(value, self.filetype)
    }
}

fn render_for_filetype(value: String, filetype: SupportedFiletype) -> response::Result<'static> {
    match filetype {
        SupportedFiletype::Png => {
            let png = pngify(&svgify(value));
            Response::build()
                .header(ContentType::PNG)
                .sized_body(png.len(), Cursor::new(png))
                .ok()
        }
        SupportedFiletype::Svg => {
            let svg = svgify(value);
            Response::build()
                .header(ContentType::SVG)
                .sized_body(svg.len(), Cursor::new(svg))
                .ok()
        }
        SupportedFiletype::Txt => Response::build()
            .header(ContentType::Plain)
            .sized_body(value.len(), Cursor::new(value))
            .ok(),
    }
}

#[cached(
    type = "SizedCache<String, Vec<u8>>",
    create = "{ SizedCache::with_size(1000) }",
    convert = "{ svg.to_string() }"
)]
fn pngify(svg: &str) -> Vec<u8> {
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.fontdb.set_monospace_family("Inconsolata");

    let rtree = usvg::Tree::from_str(svg, &opt.to_ref()).unwrap();

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.encode_png().unwrap()
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
        r##"<text fill="#F2F2F2" font-family="monospace" font-size="140" textLength="{}" transform="scale(.1)" x="160" y="240">{}</text>"##,
        (width * 10) - 320,
        &s
    );
    svg.push_str(&b);
    svg.push_str("</svg>");
    svg
}
