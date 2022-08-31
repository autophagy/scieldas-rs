use crate::scieldas::{Scield, ScieldRequest, StateScield};
use std::str::FromStr;

enum Licence {
    Mit,
    Apache,
    Gpl,
}

struct ParseLicenceError;

impl FromStr for Licence {
    type Err = ParseLicenceError;

    fn from_str(s: &str) -> Result<Licence, ParseLicenceError> {
        match s {
            "mit" => Ok(Licence::Mit),
            "apache" => Ok(Licence::Apache),
            "gpl" => Ok(Licence::Gpl),
            _ => Err(ParseLicenceError),
        }
    }
}

impl ToString for Licence {
    fn to_string(&self) -> String {
        match &self {
            Licence::Mit => "MIT".to_string(),
            Licence::Apache => "Apache 2".to_string(),
            Licence::Gpl => "GPL 3".to_string(),
        }
    }
}

const LICENCE_SCIELD: StateScield = StateScield {
    prefix: None,
    suffix: None,
};

pub fn routes() -> Vec<rocket::Route> {
    routes![license]
}

#[get("/<license>")]
async fn license(license: ScieldRequest<Licence>) -> Scield<Licence, StateScield> {
    Scield {
        scield: LICENCE_SCIELD,
        value: license.body,
        filetype: license.filetype,
    }
}
