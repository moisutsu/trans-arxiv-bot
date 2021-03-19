use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct TranslateResponse {
    text: String,
}

// see: https://qiita.com/satto_sann/items/be4177360a0bc3691fdf
//      https://gist.github.com/moisutsu/6d5b1721d4c4e4aa7e6184f2a6f557d5
pub async fn translate(text: &str, source_lang: &str, target_lang: &str) -> Result<String> {
    let params: HashMap<&str, &str> = vec![
        ("text", text),
        ("source", source_lang),
        ("target", target_lang),
    ]
    .into_iter()
    .collect();
    let client = reqwest::Client::new();
    let url = std::env::var("TRANSLATE_URL")?;
    let response_body = client.post(url).json(&params).send().await?.text().await?;
    let response_json: TranslateResponse = serde_json::from_str(&response_body)?;

    Ok(response_json.text)
}
