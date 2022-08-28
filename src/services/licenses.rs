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

#[get("/<license>")]
pub async fn get_license(license: ScieldRequest) -> Scield<StateScield> {
    Scield {
        scield: LICENCE_SCIELD,
        value: license.body.to_lowercase(),
        filetype: license.filetype,
    }
}
