use std::collections::HashMap;

use anyhow::Result;
use kuon::TwitterAPI;

pub struct TwitterClient {
    api: TwitterAPI,
}

impl TwitterClient {
    pub async fn new() -> Result<Self> {
        let api = TwitterAPI::new_using_env().await?;
        Ok(TwitterClient { api })
    }

    // Tweet long sentences.
    // If the sentence exceeds the character limit of the tweet, reply with the rest of the sentence.
    pub async fn tweet_long_text(&self, contents: &str) -> Result<()> {
        let character_limit = 140;
        let mut contents = contents.chars().collect::<Vec<_>>();
        let mut status_id: Option<String> = None;

        while contents.len() != 0 {
            let bound = std::cmp::min(contents.len(), character_limit);
            let tweet_contents = contents.drain(..bound).collect::<String>();
            if let Some(in_reply_to_status_id) = status_id.clone() {
                let reply_id = self.reply(&tweet_contents, &in_reply_to_status_id).await?;
                status_id = Some(reply_id);
            } else {
                let tweet_result = self.api.tweet(&tweet_contents).await?;
                status_id = Some(tweet_result.id.to_string());
            }
        }
        Ok(())
    }

    async fn reply(&self, contents: &str, in_reply_to_status_id: &str) -> Result<String> {
        let params: HashMap<&str, &str> = vec![
            ("in_reply_to_status_id", in_reply_to_status_id),
            ("auto_populate_reply_metadata", "true"),
        ]
        .into_iter()
        .collect();
        let reply_result = self.api.tweet_with_params(contents, &params).await?;
        Ok(reply_result.id.to_string())
    }
}
