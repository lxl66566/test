use anyhow::Result;
use reqwest;
pub async fn request(word: &str) -> Result<String> {
    let content = reqwest::get(
        // TODO
    )
    .await?
    .text()
    .await?;
}
