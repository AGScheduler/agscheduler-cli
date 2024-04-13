use std::default::Default;
use std::time::Duration;

use reqwest::Method;
use serde_json::Value;

pub struct Options {
    pub method: Method,
    pub timeout: Duration,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            method: Method::GET,
            timeout: Duration::from_secs(6),
        }
    }
}

pub async fn fetch(url: String, options: Options) -> anyhow::Result<Value> {
    let client = reqwest::Client::new();

    let response = client
        .request(options.method, url)
        .timeout(options.timeout)
        .send()
        .await?;
    if !response.status().is_success() {
        let err = response.text().await?;
        return Err(anyhow::anyhow!(err));
    }

    let body = response.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    if v["error"] != "" {
        return Err(anyhow::anyhow!(v["error"].as_str().unwrap().to_string()));
    }

    Ok(v["data"].to_owned())
}
