use crate::shields::{Shield, ShieldRequest, StateShield};
use std::collections::HashMap;

#[get("/python/<codestyle>")]
pub async fn get_python_style(codestyle: ShieldRequest) -> Shield<StateShield> {
    Shield {
        shield: StateShield {
            prefix: Some("Style".to_string()),
            value: codestyle.body,
            states: HashMap::from([
                ("black".to_string(), "Black".to_string()),
                ("yapf".to_string(), "Yapf".to_string()),
                ("autopep8".to_string(), "AutoPEP8".to_string()),
            ]),
            ..Default::default()
        },
        filetype: codestyle.filetype,
    }
}
