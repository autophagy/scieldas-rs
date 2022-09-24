use crate::scieldas::{Scield, ScieldRequest, StateScield};
use std::str::FromStr;

enum PythonStyle {
    Black,
    Yapf,
    AutoPEP8,
}

struct ParseStyleError;

impl FromStr for PythonStyle {
    type Err = ParseStyleError;
    fn from_str(s: &str) -> Result<PythonStyle, ParseStyleError> {
        match &s.to_lowercase()[..] {
            "black" => Ok(PythonStyle::Black),
            "yapf" => Ok(PythonStyle::Yapf),
            "autopep8" => Ok(PythonStyle::AutoPEP8),
            _ => Err(ParseStyleError),
        }
    }
}

impl ToString for PythonStyle {
    fn to_string(&self) -> String {
        match &self {
            PythonStyle::Black => "Black".to_string(),
            PythonStyle::Yapf => "YAPF".to_string(),
            PythonStyle::AutoPEP8 => "AutoPEP8".to_string(),
        }
    }
}

const PYTHON_STYLE_SCIELD: StateScield = StateScield {
    prefix: Some("Style"),
    suffix: None,
};

pub fn routes() -> Vec<rocket::Route> {
    routes![python_style]
}

#[get("/python/<codestyle>")]
async fn python_style(codestyle: ScieldRequest<PythonStyle>) -> Scield<PythonStyle, StateScield> {
    Scield {
        scield: PYTHON_STYLE_SCIELD,
        value: codestyle.body,
        filetype: codestyle.filetype,
    }
}
