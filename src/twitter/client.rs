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
        let mut contents = contents.chars().collect::<Vec<_>>();
        let mut status_id: Option<String> = None;

        while !contents.is_empty() {
            let bound = tweet_bound(&contents);
            let tweet_contents = contents.drain(..bound).collect::<String>();
            if let Some(in_reply_to_status_id) = status_id {
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

// Get the character count boundary of the tweet until the character limit is reached.
fn tweet_bound(contents: &[char]) -> usize {
    // The actual character limit is 280, but for the sake of completeness of the algorithm, it is 279.
    let mut remain_num_of_char = 279;
    let mut bound = 0;
    for &c in contents {
        remain_num_of_char -= if c.is_ascii() { 1 } else { 2 };
        bound += 1;
        if remain_num_of_char <= 0 {
            break;
        }
    }
    bound
}
