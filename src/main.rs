use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use clap::Clap;
use trans_arxiv_bot::{arxiv_lib, translate, Opts, TwitterClient};

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let (date_from, date_to) = get_date_range(&opts);
    let twitter_client = TwitterClient::new().await?;
    for arxiv in arxiv_lib::fetch_info(&opts.category, &date_from, &date_to).await? {
        let translated_summary =
            translate(&arxiv.summary, &opts.source_lang, &opts.target_lang).await?;
        let tweet_contents = format!("{} {}\n{}", arxiv.title, arxiv.url, translated_summary);
        twitter_client.tweet_long_text(&tweet_contents).await?;
    }
    Ok(())
}

fn get_date_range(opts: &Opts) -> (DateTime<Utc>, DateTime<Utc>) {
    let date_to = Utc::now();
    let date_from = date_to - Duration::days(opts.range_days) - Duration::hours(opts.range_hours);
    (date_from, date_to)
}
