use anyhow::Result;
use arxiv::{fetch_arxivs, query};

#[derive(Debug)]
pub struct ArxivInfo {
    pub title: String,
    pub url: String,
    pub summary: String,
}

// date format: YYYYMMDDHHmm
pub async fn fetch_arxiv_info(
    category: &str,
    date_from: &str,
    date_to: &str,
) -> Result<Vec<ArxivInfo>> {
    let mut arxivs = vec![];
    let query = query!(
        search_query = &format!(
            "cat:{} AND submittedDate:[{} TO {}]",
            category, date_from, date_to
        ),
        sort_by = "submittedDate",
        sort_order = "ascending"
    );
    for arxiv in fetch_arxivs(query).await? {
        arxivs.push(ArxivInfo {
            title: arxiv.title,
            url: arxiv.pdf_url,
            summary: arxiv.summary.replace("\n", " "),
        })
    }
    Ok(arxivs)
}
