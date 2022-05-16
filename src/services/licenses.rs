use crate::shields::{ShieldRequest, StateShield};
use std::collections::HashMap;

#[get("/<license>")]
pub async fn get_license(license: ShieldRequest) -> StateShield {
    StateShield {
        value: license.body,
        states: HashMap::from([
            ("mit".to_string(), "MIT".to_string()),
            ("apache".to_string(), "Apache 2".to_string()),
            ("gpl".to_string(), "GPL 3".to_string()),
        ]),
        filetype: license.filetype,
        ..Default::default()
    }
}
