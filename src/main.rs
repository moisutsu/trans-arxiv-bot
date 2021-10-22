use std::{collections::VecDeque, thread, time};

use anyhow::Result;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use clap::Parser;
use log::error;

use trans_arxiv_bot::{arxiv_lib, arxiv_lib::ArxivInfo, translate, Opts, TwitterClient};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();
    let twitter_client = TwitterClient::new().await?;
    let mut tweet_queue = VecDeque::new();

    // Add a second to avoid duplicate fetching of the latest paper.
    let mut date_from =
        arxiv_lib::latest_published_time(&opts.category).await? + Duration::seconds(1);

    loop {
        // To convert seconds to minutes, multiply by 60.
        thread::sleep(time::Duration::from_secs(opts.update_frequency * 60));

        let date_to = Utc::now();

        for arxiv in arxiv_lib::fetch_info(&opts.category, &date_from, &date_to)
            .await
            .unwrap_or_default()
        {
            date_from = arxiv.published + Duration::seconds(1);
            tweet_queue.push_front(arxiv);
        }

        if let Some(arxiv) = tweet_queue.pop_back() {
            twitter_client
                .tweet_translated_arxiv(&arxiv, &opts.source_lang, &opts.target_lang)
                .await
                .unwrap_or_else(|err| error!("{}", err));
        }

        // To avoid continuous tweeting, use a queue to delay tweeting.
        // To emphasize real-time performance, tweets above a certain number will not be saved in the queue.
        // Specifically, store only as much as you can tweet in a day.
        // Formula: tweet storage <= a day (minute) / tweet interval (minute)
        while tweet_queue.len() as u64 > 24 * 60 / opts.update_frequency {
            let arxiv = tweet_queue.pop_back().unwrap();
            twitter_client
                .tweet_translated_arxiv(&arxiv, &opts.source_lang, &opts.target_lang)
                .await
                .unwrap_or_else(|err| error!("{}", err));
        }
    }
}

#[async_trait]
trait TweetTranslatedArxiv {
    async fn tweet_translated_arxiv(
        &self,
        arxiv: &ArxivInfo,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<()>;
}

#[async_trait]
impl TweetTranslatedArxiv for TwitterClient {
    async fn tweet_translated_arxiv(
        &self,
        arxiv: &ArxivInfo,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<()> {
        let translated_summary = translate(&arxiv.summary, source_lang, target_lang).await?;
        let comment = arxiv
            .comment
            .clone()
            .map_or("".to_string(), |s| format!("Comment: {}\n", s));
        let tweet_contents = format!(
            "{} {}\n{}{}",
            arxiv.title, arxiv.url, comment, translated_summary
        );
        self.tweet_long_text(&tweet_contents).await?;
        Ok(())
    }
}
