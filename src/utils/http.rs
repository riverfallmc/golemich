use std::sync::LazyLock;
use reqwest::{Client, Result};

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::default());

pub async fn get_body(url: &str) -> Result<String> {
  CLIENT.get(url)
    .send().await?
    .text().await
}