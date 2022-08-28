use crate::shields::{Shield, ShieldRequest, TextShield};
use crate::utils::{get_payload, readable_number};

const CRATE_API_URL: &str = "https://crates.io/api/v1/crates/";

const CRATE_DOWNLOADS_SHIELD: TextShield = TextShield {
    prefix: "Downloads",
    suffix: None,
};

const CRATE_VERSION_SHIELD: TextShield = TextShield {
    prefix: "Version",
    suffix: None,
};

#[get("/downloads/<crate_name>")]
pub async fn get_crate_downloads(crate_name: ShieldRequest) -> Option<Shield<TextShield>> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let downloads = get_payload(&request_url)
        .await?
        .get("crate")?
        .get("downloads")?
        .as_f64()?;

    Some(Shield {
        shield: CRATE_DOWNLOADS_SHIELD,
        value: readable_number(downloads),
        filetype: crate_name.filetype,
    })
}

#[get("/downloads/<crate_name>/<version>")]
pub async fn get_crate_version_downloads(
    crate_name: &str,
    version: ShieldRequest,
) -> Option<Shield<TextShield>> {
    let request_url = format!("{}/{}/{}", CRATE_API_URL, crate_name, version.body);

    let downloads = get_payload(&request_url)
        .await?
        .get("version")?
        .get("downloads")?
        .as_f64()?;

    Some(Shield {
        shield: CRATE_DOWNLOADS_SHIELD,
        value: readable_number(downloads),
        filetype: version.filetype,
    })
}

#[get("/version/<crate_name>")]
pub async fn get_crate_version(crate_name: ShieldRequest) -> Option<Shield<TextShield>> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let version = String::from(
        get_payload(&request_url)
            .await?
            .get("crate")?
            .get("max_version")?
            .as_str()?,
    );

    Some(Shield {
        shield: CRATE_VERSION_SHIELD,
        value: version,
        filetype: crate_name.filetype,
    })
}
