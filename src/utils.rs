use cached::proc_macro::cached;
use cached::TimedSizedCache;
use reqwest::Client;
use rocket::State;
use serde_json::Value;

#[cached(
    type = "TimedSizedCache<String, Option<Value>>",
    create = "{ TimedSizedCache::with_size_and_lifespan(1000, 300) }",
    convert = "{ url.to_string() }"
)]
pub async fn get_payload(client: &State<Client>, url: &str) -> Option<Value> {
    let response = client.get(url).send().await;

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
}
