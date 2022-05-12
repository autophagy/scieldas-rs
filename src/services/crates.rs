use crate::shields::TextShield;
use crate::utils::{get_payload, readable_number};

static CRATE_DOWNLOADS_URL: &'static str = "https://crates.io/api/v1/crates/";

#[get("/downloads/<crate_name>")]
pub async fn get_crate_downloads(crate_name: &str) -> TextShield {
    let request_url = format!("{}/{}", CRATE_DOWNLOADS_URL, crate_name);
    let prefix = Some(String::from("Downloads"));

    if let Some(v) = get_payload(&request_url).await {
        let downloads = v
            .get("crate")
            .and_then(|v| v.get("downloads"))
            .and_then(|v| v.as_f64());

        if let Some(x) = downloads {
            TextShield {
                prefix,
                value: readable_number(x),
                ..Default::default()
            }
        } else {
            TextShield {
                prefix,
                ..Default::default()
            }
        }
    } else {
        TextShield {
            prefix,
            ..Default::default()
        }
    }
}

#[get("/downloads/<crate_name>/<version>")]
pub async fn get_crate_version_downloads(crate_name: &str, version: &str) -> TextShield {
    let request_url = format!("{}/{}/{}", CRATE_DOWNLOADS_URL, crate_name, version);
    let prefix = Some(format!("Downloads (v{})", version));

    if let Some(v) = get_payload(&request_url).await {
        let downloads = v
            .get("version")
            .and_then(|v| v.get("downloads"))
            .and_then(|v| v.as_f64());

        match downloads {
            Some(x) => TextShield {
                prefix,
                value: readable_number(x),
                ..Default::default()
            },
            None => TextShield {
                prefix,
                ..Default::default()
            },
        }
    } else {
        TextShield {
            prefix,
            ..Default::default()
        }
    }
}

#[get("/version/<crate_name>")]
pub async fn get_crate_version(crate_name: &str) -> TextShield {
    let request_url = format!("{}/{}", CRATE_DOWNLOADS_URL, crate_name);
    let prefix = Some(String::from("Version"));

    if let Some(v) = get_payload(&request_url).await {
        let version = v
            .get("crate")
            .and_then(|v| v.get("max_version"))
            .and_then(|v| v.as_str());

        match version {
            Some(x) => TextShield {
                prefix,
                value: String::from(x),
                ..Default::default()
            },
            None => TextShield {
                prefix,
                ..Default::default()
            },
        }
    } else {
        TextShield {
            prefix,
            ..Default::default()
        }
    }
}
