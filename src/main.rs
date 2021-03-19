use anyhow::Result;
use dotenv::dotenv;
use trans_arxiv_bot::{fetch_arxiv_info, translate, TwitterClient};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let twitter_client = TwitterClient::new().await?;
    for arxiv in fetch_arxiv_info("cat:cs.CL", "20190101", "20190102").await? {
        let translated_summary = translate(&arxiv.summary, "en", "ja").await?;
        let tweet_contents = format!("{} {}\n{}", arxiv.title, arxiv.url, translated_summary);
        twitter_client.tweet_long_text(&tweet_contents).await?;
    }
    Ok(())
}
