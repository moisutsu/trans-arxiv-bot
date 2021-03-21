use anyhow::Result;
use arxiv::{fetch_arxivs, query};
use chrono::{DateTime, Utc};

pub async fn latest_published_time(category: &str) -> Result<DateTime<Utc>> {
    let query = query!(
        search_query = &format!("cat:{}", category),
        sort_by = "submittedDate",
        sort_order = "descending",
        max_results = 1
    );
    let arxivs = fetch_arxivs(query).await?;
    Ok(arxivs[0].published.parse()?)
}
