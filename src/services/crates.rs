use serde_json::Value;

static CRATE_DOWNLOADS_URL: &'static str = "https://crates.io/api/v1/crates/";

async fn get_payload(url: &str) -> Option<Value> {
    let client = reqwest::Client::builder().user_agent("scieldas").build();

    if let Ok(c) = client {
        let response = c.get(url).send().await;

        if let Ok(r) = response {
            let root: Result<Value, reqwest::Error> = r.json().await;
            if let Ok(s) = root {
                Some(s)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn readable_number(number: f64) -> String {
    if number == 0.0 {
        number.to_string()
    } else {
        let units = vec!["", "K", "M", "G", "T", "P"];
        let magnitude: usize = number.abs().log(1000.0).floor() as usize;
        let amnt = number.abs() / (1000.0_f64.powf(magnitude as f64));

        if magnitude == 0 {
            format!("{}", amnt)
        } else {
            let sign = if number > 0.0 { "" } else { "-" };
            format!(
                "{sign}{amnt}{unit}",
                sign = sign,
                amnt = amnt.floor(),
                unit = units[magnitude]
            )
        }
    }
}

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
