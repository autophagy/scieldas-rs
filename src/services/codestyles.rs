use crate::shields::{Shield, ShieldRequest, StateShield};
use phf::phf_map;

const PYTHON_STYLE_SHIELD: StateShield = StateShield {
    prefix: Some("Style"),
    suffix: None,
    states: phf_map! {
        "black" => "Black",
        "yapf" => "Yapf",
        "autopep8" => "AutoPEP8",
    },
};

#[get("/python/<codestyle>")]
pub async fn get_python_style(codestyle: ShieldRequest) -> Shield<StateShield> {
    Shield {
        shield: PYTHON_STYLE_SHIELD,
        value: codestyle.body.to_lowercase(),
        filetype: codestyle.filetype,
    }
}
