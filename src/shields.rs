use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::{FromParam, Request};
use rocket::response::{self, Responder, Response};

use std::path::PathBuf;

pub enum SupportedFiletype {
    Png,
    Svg,
    Txt,
}

pub struct Shield<T: RenderableShield> {
    pub shield: T,
    pub value: String,
    pub filetype: SupportedFiletype,
}

impl<T: RenderableShield> Shield<T> {
    fn to_svg(&self) -> String {
        let value = self.shield.render(&self.value);
        let mut svg: String = "".to_string();
        let width = (&value.len() * 7) + 32;

        let head = format!(
            r#"<svg baseProfile="full" height="41px" version="1.1" width="{}px" xmlns="http://www.w3.org/2000/svg" xmlns:ev="http://www.w3.org/2001/xml-events" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
            width
        );
        svg.push_str(&head);

        svg.push_str(r##"<rect fill="#2D2D2D" height="100%" width="100%" x="0" y="0" />"##);
        let b = format!(
            r##"<text fill="#F2F2F2" font-family="Inconsolata Nerd Font, Inconsolata, monospace" font-size="140" textLength="{}" transform="scale(.1)" x="160" y="240">{}</text>"##,
            (width * 10) - 320,
            &value
        );
        svg.push_str(&b);
        svg.push_str("</svg>");
        svg
    }

    fn to_png(&self) -> Vec<u8> {
        let svg = self.to_svg();
        let mut opt = usvg::Options::default();
        opt.fontdb.load_system_fonts();
        opt.fontdb.set_monospace_family("Inconsolata");

        let rtree = usvg::Tree::from_str(&svg, &opt.to_ref()).unwrap();

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
}

#[rocket::async_trait]
impl<'r, T: RenderableShield> Responder<'r, 'static> for Shield<T> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        match self.filetype {
            SupportedFiletype::Png => {
                let png = self.to_png();
                Response::build()
                    .header(ContentType::PNG)
                    .sized_body(png.len(), Cursor::new(png))
                    .ok()
            }
            SupportedFiletype::Svg => {
                let svg = self.to_svg();
                Response::build()
                    .header(ContentType::SVG)
                    .sized_body(svg.len(), Cursor::new(svg))
                    .ok()
            }
            SupportedFiletype::Txt => {
                let value = self.shield.render(&self.value);
                Response::build()
                    .header(ContentType::Plain)
                    .sized_body(value.len(), Cursor::new(value))
                    .ok()
            }
        }
    }
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
    fn render(&self, value: &str) -> String;
}

pub struct TextShield {
    pub prefix: &'static str,
    pub suffix: Option<&'static str>,
}

impl RenderableShield for TextShield {
    fn render(&self, value: &str) -> String {
        let prefix = format!("{} :: ", &self.prefix);
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => String::from(""),
        };
        format!("{}{}{}", prefix, &value, suffix)
    }
}

pub struct StateShield {
    pub prefix: Option<&'static str>,
    pub suffix: Option<&'static str>,
    pub states: phf::Map<&'static str, &'static str>,
}

impl RenderableShield for StateShield {
    fn render(&self, value: &str) -> String {
        let prefix = match &self.prefix {
            Some(s) => format!("{} :: ", &s),
            None => "".to_string(),
        };
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => "".to_string(),
        };
        let value = match self.states.get(value) {
            Some(v) => v,
            None => "N/A",
        };
        format!("{}{}{}", prefix, value, suffix)
    }
}
