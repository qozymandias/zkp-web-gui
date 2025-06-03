use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::config::get_config;
use crate::types::task::ConciseTask;
use crate::types::task::PaginationResult;
use crate::types::task::RequestResult;

async fn parse_resp_from_json<T: DeserializeOwned>(resp: reqwest::Response) -> anyhow::Result<T> {
    let json_val: serde_json::Value = resp.json().await?;
    let str = serde_json::to_string(&json_val).unwrap_or("Invalid json".to_string());
    serde_json::from_value::<T>(json_val).map_err(|e| {
        let e = anyhow::anyhow!("\nError: {e}\nJson: {str}");
        tracing::error!("{e}");
        e
    })
}

/// Fetches and deserializes JSON from the given URL into any type `T`.
pub async fn fetch_json<T: DeserializeOwned, S: AsRef<str>>(url: S) -> anyhow::Result<T> {
    let resp = Client::new().get(url.as_ref().to_string()).send().await?.error_for_status()?;
    parse_resp_from_json(resp).await
}

pub async fn get_tasklist() -> anyhow::Result<RequestResult<PaginationResult<Vec<ConciseTask>>>> {
    fetch_json(format!("{}/tasklist", get_config().api.url)).await.map_err(|e| {
        tracing::info!("{e:?}");
        e
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_tasklist() {
        crate::config::set_config(crate::config::load_config().expect("Should load config"));
        let out = get_tasklist().await;
        println!("{out:?}");
        assert!(out.is_ok());
    }
}
