use crate::scieldas::{Scield, ScieldRequest, StateScield};
use phf::phf_map;

const LICENCE_SCIELD: StateScield = StateScield {
    prefix: None,
    suffix: None,
    states: phf_map! {
        "mit" => "MIT",
        "apache" => "Apache 2",
        "gpl" => "GPL 3",
    },
};

pub fn routes() -> Vec<rocket::Route> {
    routes![license]
}

#[get("/<license>")]
async fn license(license: ScieldRequest) -> Scield<String, StateScield> {
    Scield {
        scield: LICENCE_SCIELD,
        value: license.body.to_lowercase(),
        filetype: license.filetype,
    }
}
