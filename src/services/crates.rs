use crate::shields::{ShieldRequest, TextShield};
use crate::utils::{get_payload, readable_number};

static CRATE_API_URL: &str = "https://crates.io/api/v1/crates/";

#[get("/downloads/<crate_name>")]
pub async fn get_crate_downloads(crate_name: ShieldRequest) -> Option<TextShield> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let downloads = get_payload(&request_url)
        .await?
        .get("crate")?
        .get("downloads")?
        .as_f64()?;

    Some(TextShield {
        prefix: String::from("Downloads"),
        value: readable_number(downloads),
        filetype: crate_name.filetype,
        ..Default::default()
    })
}

#[get("/downloads/<crate_name>/<version>")]
pub async fn get_crate_version_downloads(
    crate_name: &str,
    version: ShieldRequest,
) -> Option<TextShield> {
    let request_url = format!("{}/{}/{}", CRATE_API_URL, crate_name, version.body);

    let downloads = get_payload(&request_url)
        .await?
        .get("version")?
        .get("downloads")?
        .as_f64()?;

    Some(TextShield {
        prefix: format!("Downloads (v{})", version.body),
        value: readable_number(downloads),
        filetype: version.filetype,
        ..Default::default()
    })
}

#[get("/version/<crate_name>")]
pub async fn get_crate_version(crate_name: ShieldRequest) -> Option<TextShield> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let version = String::from(
        get_payload(&request_url)
            .await?
            .get("crate")?
            .get("max_version")?
            .as_str()?,
    );

    Some(TextShield {
        prefix: String::from("Version"),
        value: version,
        filetype: crate_name.filetype,
        ..Default::default()
    })
}
