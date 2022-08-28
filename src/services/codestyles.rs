use crate::scieldas::{Scield, ScieldRequest, StateScield};
use phf::phf_map;

const PYTHON_STYLE_SCIELD: StateScield = StateScield {
    prefix: Some("Style"),
    suffix: None,
    states: phf_map! {
        "black" => "Black",
        "yapf" => "Yapf",
        "autopep8" => "AutoPEP8",
    },
};

#[get("/python/<codestyle>")]
pub async fn get_python_style(codestyle: ScieldRequest) -> Scield<StateScield> {
    Scield {
        scield: PYTHON_STYLE_SCIELD,
        value: codestyle.body.to_lowercase(),
        filetype: codestyle.filetype,
    }
}
