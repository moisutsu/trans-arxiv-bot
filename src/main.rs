use std::{thread, time};

use anyhow::Result;
use chrono::{Duration, Utc};
use clap::Clap;
use trans_arxiv_bot::{arxiv_lib, translate, Opts, TwitterClient};

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let twitter_client = TwitterClient::new().await?;

    // Add a second to avoid duplicate fetching of the latest paper.
    let mut date_from =
        arxiv_lib::latest_published_time(&opts.category).await? + Duration::seconds(1);

    loop {
        // To convert seconds to minutes, multiply by 60.
        thread::sleep(time::Duration::from_secs(opts.update_frequency * 60));

        let date_to = Utc::now();
        let arxivs = match arxiv_lib::fetch_info(&opts.category, &date_from, &date_to).await {
            Ok(arxivs) => arxivs,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        for arxiv in arxivs {
            date_from = arxiv.published + Duration::seconds(1);
            let translated_summary =
                match translate(&arxiv.summary, &opts.source_lang, &opts.target_lang).await {
                    Ok(translated_summary) => translated_summary,
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };

            let tweet_contents = format!("{} {}\n{}", arxiv.title, arxiv.url, translated_summary);
            if let Err(err) = twitter_client.tweet_long_text(&tweet_contents).await {
                eprintln!("{}", err);
            }
        }
    }
}
