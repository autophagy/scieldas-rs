use crate::scieldas::{Scield, ScieldRequest, TextScield};
use crate::utils::{get_payload, readable_number};
use reqwest::Client;
use rocket::State;

const CRATE_API_URL: &str = "https://crates.io/api/v1/crates/";

const CRATE_DOWNLOADS_SCIELD: TextScield = TextScield {
    prefix: "Downloads",
    suffix: None,
};

const CRATE_VERSION_SCIELD: TextScield = TextScield {
    prefix: "Version",
    suffix: None,
};

pub fn routes() -> Vec<rocket::Route> {
    routes![crate_downloads, crate_version_downloads, crate_version]
}

#[get("/downloads/<crate_name>")]
pub async fn crate_downloads(
    client: &State<Client>,
    crate_name: ScieldRequest,
) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let downloads = get_payload(client, &request_url)
        .await?
        .pointer("/crate/downloads")?
        .as_f64()?;

    Some(Scield {
        scield: CRATE_DOWNLOADS_SCIELD,
        value: readable_number(downloads),
        filetype: crate_name.filetype,
    })
}

#[get("/downloads/<crate_name>/<version>")]
pub async fn crate_version_downloads(
    client: &State<Client>,
    crate_name: &str,
    version: ScieldRequest,
) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/{}/{}", CRATE_API_URL, crate_name, version.body);

    let downloads = get_payload(client, &request_url)
        .await?
        .pointer("/version/downloads")?
        .as_f64()?;

    Some(Scield {
        scield: CRATE_DOWNLOADS_SCIELD,
        value: readable_number(downloads),
        filetype: version.filetype,
    })
}

#[get("/version/<crate_name>")]
pub async fn crate_version(
    client: &State<Client>,
    crate_name: ScieldRequest,
) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let version = String::from(
        get_payload(client, &request_url)
            .await?
            .pointer("/crate/max_version")?
            .as_str()?,
    );

    Some(Scield {
        scield: CRATE_VERSION_SCIELD,
        value: version,
        filetype: crate_name.filetype,
    })
}
