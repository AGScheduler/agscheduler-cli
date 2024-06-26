use std::default::Default;
use std::time::Duration;

use reqwest::Method;
use serde_json::Value;

use crate::utils;

pub static mut PASSWORD_SHA2: String = String::new();

pub struct Options {
    pub method: Method,
    pub body: String,
    pub timeout: Duration,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            method: Method::GET,
            body: String::new(),
            timeout: Duration::from_secs(6),
        }
    }
}

pub async fn fetch(url: String, options: Options) -> anyhow::Result<Value> {
    let client = reqwest::Client::new();

    let password_sha2;
    unsafe { password_sha2 = PASSWORD_SHA2.as_str().to_string() }

    let response = client
        .request(options.method, url)
        .header("Auth-Password-SHA2", password_sha2)
        .body(options.body)
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

pub async fn fetch_show_json(url: String, options: Options) {
    match fetch(url, options).await {
        Ok(result) => {
            utils::show_json(result);
        }
        Err(err) => {
            println!("Error: {}", err)
        }
    }
}

pub async fn fetch_show_ok(url: String, options: Options) {
    match fetch(url, options).await {
        Ok(_) => {
            println!("Ok")
        }
        Err(err) => {
            println!("Error: {}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn it_fetch_ok() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let body = json!({"data": {"hello":"world"}, "error": ""}).to_string();

        server
            .mock("GET", "/hello")
            .with_status(200)
            .with_body(body)
            .create_async()
            .await;

        let result = fetch(format!("{}{}", url, "/hello"), Options::default())
            .await
            .unwrap();

        assert_eq!("{\"hello\":\"world\"}", result.to_string());
    }

    #[tokio::test]
    async fn it_fetch_status_failed() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let body = "404 page not found";

        server
            .mock("GET", "/")
            .with_status(404)
            .with_body(body)
            .create_async()
            .await;

        let result = fetch(format!("{}{}", url, "/"), Options::default())
            .await
            .unwrap_err();

        assert_eq!(body, result.to_string());
    }

    #[tokio::test]
    async fn it_fetch_error() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let body = json!({"data": null, "error": "`id` not found!"}).to_string();

        server
            .mock("POST", "/job")
            .with_status(200)
            .with_body(body)
            .create_async()
            .await;

        let result = fetch(
            format!("{}{}", url, "/job"),
            Options {
                method: Method::POST,
                ..Default::default()
            },
        )
        .await
        .unwrap_err();

        assert_eq!("`id` not found!", result.to_string());
    }
}
