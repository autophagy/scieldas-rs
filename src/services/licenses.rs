use crate::shields::{Shield, ShieldRequest, StateShield};
use phf::phf_map;

const LICENCE_SHIELD: StateShield = StateShield {
    prefix: None,
    suffix: None,
    states: phf_map! {
        "mit" => "MIT",
        "apache" => "Apache 2",
        "gpl" => "GPL 3",
    },
};

#[get("/<license>")]
pub async fn get_license(license: ShieldRequest) -> Shield<StateShield> {
    Shield {
        shield: LICENCE_SHIELD,
        value: license.body.to_lowercase(),
        filetype: license.filetype,
    }
}
