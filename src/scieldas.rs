use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::{FromParam, Request};
use rocket::response::{self, Responder, Response};

use std::cmp;
use std::path::PathBuf;

/// Scield Request
/// ==============

pub enum SupportedFiletype {
    Png,
    Svg,
    Txt,
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

/// Scield Traits
/// =============

/// Trait for a renderable scield. A scield must implement this trait to turn
/// a given value into a scieldic representation, usually with some prefix and
/// transformation of the input value.
pub trait RenderableScield<T> {
    fn render(&self, value: &T) -> String;
}

/// Scield
/// ======

pub struct Scield<A, T: RenderableScield<A>> {
    pub scield: T,
    pub value: A,
    pub filetype: SupportedFiletype,
}

impl<A, T: RenderableScield<A>> Scield<A, T> {
    fn to_svg(&self) -> String {
        let value = self.scield.render(&self.value);
        let mut svg: String = "".to_string();
        let width = (&value.len() * 7) + 32;

        let head = format!(
            r#"<svg baseProfile="full" height="41px" version="1.1" width="{}px" xmlns="http://www.w3.org/2000/svg" xmlns:ev="http://www.w3.org/2001/xml-events" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
            width
        );
        svg.push_str(&head);

        svg.push_str(r##"<rect fill="#282828" height="100%" width="100%" x="0" y="0" />"##);
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
impl<'r, A, T: RenderableScield<A>> Responder<'r, 'static> for Scield<A, T> {
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

/// Text Scield
/// ===========
///
/// A simple scield for returning an arbitrary string value.

pub struct TextScield {
    pub prefix: &'static str,
    pub suffix: Option<&'static str>,
}

impl RenderableScield<String> for TextScield {
    fn render(&self, value: &String) -> String {
        let prefix = format!("{} :: ", &self.prefix);
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => String::from(""),
        };
        format!("{}{}{}", prefix, &value, suffix)
    }
}

impl RenderableScield<f64> for TextScield {
    fn render(&self, value: &f64) -> String {
        let prefix = format!("{} :: ", &self.prefix);
        let suffix = match &self.suffix {
            Some(s) => format!(" {}", s),
            None => String::from(""),
        };
        format!("{}{}{}", prefix, readable_number(*value), suffix)
    }
}

fn readable_number(number: f64) -> String {
    if number == 0.0 {
        number.to_string()
    } else {
        let units = vec!["", "k", "m", "bn"];
        let magnitude: usize = cmp::min(number.abs().log(1000.0).floor() as usize, units.len() - 1);
        let amnt = number.abs() / (1000.0_f64.powf(magnitude as f64));

        if magnitude == 0 {
            format!("{}", number.floor())
        } else {
            let sign = if number > 0.0 { "" } else { "-" };
            let magnitude_symbol = match units.get(magnitude) {
                Some(x) => x,
                None => units[units.len() - 1],
            };
            format!(
                "{sign}{amnt}{unit}",
                sign = sign,
                amnt = amnt.floor(),
                unit = magnitude_symbol
            )
        }
    }
}

/// State Scield
///
/// ============
///
/// A scield for returning a value from a predefined set of possible values.

pub struct StateScield {
    pub prefix: Option<&'static str>,
    pub suffix: Option<&'static str>,
    pub states: phf::Map<&'static str, &'static str>,
}

impl RenderableScield<String> for StateScield {
    fn render(&self, value: &String) -> String {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_readable_number() {
        assert_eq!(readable_number(12.0), "12");
        assert_eq!(readable_number(123.0), "123");
        assert_eq!(readable_number(1234.0), "1k");
        assert_eq!(readable_number(12_345.0), "12k");
        assert_eq!(readable_number(123_456.0), "123k");
        assert_eq!(readable_number(1_234_567.0), "1m");
        assert_eq!(readable_number(12_345_678.0), "12m");
        assert_eq!(readable_number(123_456_789.0), "123m");
        assert_eq!(readable_number(1_234_567_891.0), "1bn");
        assert_eq!(readable_number(-12.0), "-12");
        assert_eq!(readable_number(-123.0), "-123");
        assert_eq!(readable_number(-1234.0), "-1k");
        assert_eq!(readable_number(-12_345.0), "-12k");
        assert_eq!(readable_number(-123_456.0), "-123k");
        assert_eq!(readable_number(-1_234_567.0), "-1m");
        assert_eq!(readable_number(-12_345_678.0), "-12m");
        assert_eq!(readable_number(-123_456_789.0), "-123m");
        assert_eq!(readable_number(-1_234_567_891.0), "-1bn");
    }
}
