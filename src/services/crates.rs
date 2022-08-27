use crate::shields::{Shield, ShieldRequest, TextShield};
use crate::utils::{get_payload, readable_number};

static CRATE_API_URL: &str = "https://crates.io/api/v1/crates/";

#[get("/downloads/<crate_name>")]
pub async fn get_crate_downloads(crate_name: ShieldRequest) -> Option<Shield<TextShield>> {
    let request_url = format!("{}/{}", CRATE_API_URL, crate_name.body);

    let downloads = get_payload(&request_url)
        .await?
        .get("crate")?
        .get("downloads")?
        .as_f64()?;

    Some(Shield {
        shield: TextShield {
            prefix: "Downloads".to_string(),
            value: readable_number(downloads),
            ..Default::default()
        },
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
        shield: TextShield {
            prefix: format!("Downloads (v{})", version.body),
            value: readable_number(downloads),
            ..Default::default()
        },
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
        shield: TextShield {
            prefix: String::from("Version"),
            value: version,
            ..Default::default()
        },
        filetype: crate_name.filetype,
    })
}
