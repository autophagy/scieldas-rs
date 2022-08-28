use crate::scieldas::{Scield, ScieldRequest, TextScield};
use crate::utils::{get_payload, readable_number};

const CRATE_API_URL: &str = "https://crates.io/api/v1/crates/";

const CRATE_DOWNLOADS_SCIELD: TextScield = TextScield {
    prefix: "Downloads",
    suffix: None,
};

const CRATE_VERSION_SCIELD: TextScield = TextScield {
    prefix: "Version",
    suffix: None,
};

#[get("/downloads/<crate_name>")]
pub async fn get_crate_downloads(crate_name: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let downloads = get_payload(&request_url)
        .await?
        .get("crate")?
        .get("downloads")?
        .as_f64()?;

    Some(Scield {
        scield: CRATE_DOWNLOADS_SCIELD,
        value: readable_number(downloads),
        filetype: crate_name.filetype,
    })
}

#[get("/downloads/<crate_name>/<version>")]
pub async fn get_crate_version_downloads(
    crate_name: &str,
    version: ScieldRequest,
) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/{}/{}", CRATE_API_URL, crate_name, version.body);

    let downloads = get_payload(&request_url)
        .await?
        .get("version")?
        .get("downloads")?
        .as_f64()?;

    Some(Scield {
        scield: CRATE_DOWNLOADS_SCIELD,
        value: readable_number(downloads),
        filetype: version.filetype,
    })
}

#[get("/version/<crate_name>")]
pub async fn get_crate_version(crate_name: ScieldRequest) -> Option<Scield<TextScield>> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let version = String::from(
        get_payload(&request_url)
            .await?
            .get("crate")?
            .get("max_version")?
            .as_str()?,
    );

    Some(Scield {
        scield: CRATE_VERSION_SCIELD,
        value: version,
        filetype: crate_name.filetype,
    })
}
