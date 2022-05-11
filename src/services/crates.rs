use crate::utils::{get_payload, readable_number};

static CRATE_DOWNLOADS_URL: &'static str = "https://crates.io/api/v1/crates/";

#[get("/downloads/<crate_name>")]
pub async fn get_crate_downloads(crate_name: &str) -> String {
    let request_url = format!("{}/{}", CRATE_DOWNLOADS_URL, crate_name);

    if let Some(v) = get_payload(&request_url).await {
        let downloads = v
            .get("crate")
            .and_then(|v| v.get("downloads"))
            .and_then(|v| v.as_f64());

        if let Some(x) = downloads {
            format!("Downloads :: {}", readable_number(x))
        } else {
            String::from("Downloads :: N/A")
        }
    } else {
        String::from("Downloads :: N/A")
    }
}

#[get("/downloads/<crate_name>/<version>")]
pub async fn get_crate_version_downloads(crate_name: &str, version: &str) -> String {
    let request_url = format!("{}/{}/{}", CRATE_DOWNLOADS_URL, crate_name, version);
    if let Some(v) = get_payload(&request_url).await {
        let downloads = v
            .get("version")
            .and_then(|v| v.get("downloads"))
            .and_then(|v| v.as_f64());

        match downloads {
            Some(x) => format!("Downloads (v{}) :: {}", version, readable_number(x)),
            None => format!("Downloads (v{}) :: N/A", version),
        }
    } else {
        String::from("Error")
    }
}

#[get("/version/<crate_name>")]
pub async fn get_crate_version(crate_name: &str) -> String {
    let request_url = format!("{}/{}", CRATE_DOWNLOADS_URL, crate_name);
    if let Some(v) = get_payload(&request_url).await {
        let version = v
            .get("crate")
            .and_then(|v| v.get("max_version"))
            .and_then(|v| v.as_str());

        match version {
            Some(x) => format!("Version :: {}", x),
            None => String::from("Version :: N/A"),
        }
    } else {
        String::from("Error")
    }
}
