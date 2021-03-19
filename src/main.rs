use anyhow::Result;
use dotenv::dotenv;
use trans_arxiv_bot::{translate, TwitterClient};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let twitter_client = TwitterClient::new().await?;
    let long_text = "Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency.[16][17] Rust is syntactically similar to C++,[18] but can guarantee memory safety by using a borrow checker to validate references.[19] Rust achieves memory safety without garbage collection, and reference counting is optional.[20][21]

    Rust was originally designed by Graydon Hoare at Mozilla Research, with contributions from Dave Herman, Brendan Eich, and others.[22][23] The designers refined the language while writing the Servo layout or browser engine,[24] and the Rust compiler. It has gained increasing use in industry, and Microsoft has been experimenting with the language for secure and safety-critical software components.[25][26]

    Rust has been voted the 'most loved programming language' in the Stack Overflow Developer Survey every year since 2016.[27]";
    let translated_text = translate(long_text, "en", "ja").await?;
    twitter_client.tweet_long_text(&translated_text).await?;
    Ok(())
}
