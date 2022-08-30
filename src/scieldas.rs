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

pub struct Scield<T: RenderableScield> {
    pub scield: T,
    pub value: String,
    pub filetype: SupportedFiletype,
}

impl<T: RenderableScield> Scield<T> {
    fn to_svg(&self) -> String {
        let value = self.scield.render(&self.value);
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

    fn to_png(&self, opt: &usvg::Options) -> Vec<u8> {
        let svg = self.to_svg();
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
impl<'r, T: RenderableScield> Responder<'r, 'static> for Scield<T> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self.filetype {
            SupportedFiletype::Png => {
                let opt: &usvg::Options = request.rocket().state().unwrap();
                let png = self.to_png(opt);
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
                let value = self.scield.render(&self.value);
                Response::build()
                    .header(ContentType::Plain)
                    .sized_body(value.len(), Cursor::new(value))
                    .ok()
            }
        }
    }
}

pub struct ScieldRequest {
    pub body: String,
    pub filetype: SupportedFiletype,
}

#[derive(Debug)]
pub enum ScieldRequestError {
    InvalidBody,
    InvalidFiletype,
}

impl<'r> FromParam<'r> for ScieldRequest {
    type Error = ScieldRequestError;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let path: PathBuf = PathBuf::from(param);
        let stem = path.file_stem();

        if let Some(b) = stem {
            let body = String::from(b.to_str().unwrap());
            if param.ends_with(".png") {
                Ok(ScieldRequest {
                    body,
                    filetype: SupportedFiletype::Png,
                })
            } else if param.ends_with(".svg") {
                Ok(ScieldRequest {
                    body,
                    filetype: SupportedFiletype::Svg,
                })
            } else if param.ends_with(".txt") {
                Ok(ScieldRequest {
                    body,
                    filetype: SupportedFiletype::Txt,
                })
            } else {
                Err(ScieldRequestError::InvalidFiletype)
            }
        } else {
            Err(ScieldRequestError::InvalidBody)
        }
    }
}

pub trait RenderableScield {
    fn render(&self, value: &str) -> String;
}

pub struct TextScield {
    pub prefix: &'static str,
    pub suffix: Option<&'static str>,
}

impl RenderableScield for TextScield {
    fn render(&self, value: &str) -> String {
        let prefix = format!("{} :: ", &self.prefix);
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => String::from(""),
        };
        format!("{}{}{}", prefix, &value, suffix)
    }
}

pub struct StateScield {
    pub prefix: Option<&'static str>,
    pub suffix: Option<&'static str>,
    pub states: phf::Map<&'static str, &'static str>,
}

impl RenderableScield for StateScield {
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
