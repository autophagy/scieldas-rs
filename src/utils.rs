use cached::proc_macro::cached;
use cached::TimedSizedCache;
use reqwest::Client;
use rocket::State;
use serde_json::Value;
use std::cmp;

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

pub fn readable_number(number: f64) -> String {
    if number == 0.0 {
        number.to_string()
    } else {
        let units = vec!["", "k", "m", "bn"];
        let magnitude: usize = cmp::min(number.abs().log(1000.0).floor() as usize, units.len() - 1);
        let amnt = number.abs() / (1000.0_f64.powf(magnitude as f64));

        if magnitude == 0 {
            format!("{}", number.floor())
        } else {
            let sign = if number > 0.0 { "" } else { "-" };
            let magnitude_symbol = match units.get(magnitude) {
                Some(x) => x,
                None => units[units.len() - 1],
            };
            format!(
                "{sign}{amnt}{unit}",
                sign = sign,
                amnt = amnt.floor(),
                unit = magnitude_symbol
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_readable_number() {
        assert_eq!(readable_number(12.0), "12");
        assert_eq!(readable_number(123.0), "123");
        assert_eq!(readable_number(1234.0), "1k");
        assert_eq!(readable_number(12_345.0), "12k");
        assert_eq!(readable_number(123_456.0), "123k");
        assert_eq!(readable_number(1_234_567.0), "1m");
        assert_eq!(readable_number(12_345_678.0), "12m");
        assert_eq!(readable_number(123_456_789.0), "123m");
        assert_eq!(readable_number(1_234_567_891.0), "1bn");
        assert_eq!(readable_number(-12.0), "-12");
        assert_eq!(readable_number(-123.0), "-123");
        assert_eq!(readable_number(-1234.0), "-1k");
        assert_eq!(readable_number(-12_345.0), "-12k");
        assert_eq!(readable_number(-123_456.0), "-123k");
        assert_eq!(readable_number(-1_234_567.0), "-1m");
        assert_eq!(readable_number(-12_345_678.0), "-12m");
        assert_eq!(readable_number(-123_456_789.0), "-123m");
        assert_eq!(readable_number(-1_234_567_891.0), "-1bn");
    }
}
