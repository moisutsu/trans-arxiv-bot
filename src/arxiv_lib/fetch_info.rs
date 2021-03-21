use anyhow::Result;
use arxiv::{fetch_arxivs, query};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ArxivInfo {
    pub title: String,
    pub url: String,
    pub summary: String,
    pub published: DateTime<Utc>,
}

// date format: YYYYMMDDHHmm
pub async fn fetch_info(
    category: &str,
    date_from: &DateTime<Utc>,
    date_to: &DateTime<Utc>,
) -> Result<Vec<ArxivInfo>> {
    let mut arxivs = vec![];
    let date_time_format = "%Y%m%d%H%M%S";
    let query = query!(
        search_query = &format!(
            "cat:{} AND submittedDate:[{} TO {}]",
            category,
            date_from.format(date_time_format),
            date_to.format(date_time_format)
        ),
        sort_by = "submittedDate",
        sort_order = "ascending"
    );
    for arxiv in fetch_arxivs(query).await? {
        arxivs.push(ArxivInfo {
            title: arxiv.title,
            url: arxiv.pdf_url,
            summary: arxiv.summary.replace("\n", " "),
            published: arxiv.published.parse()?,
        })
    }
    Ok(arxivs)
}
