use isahc::http::request;

use crate::search_result::SearchResult;

pub type SearchResults = Vec<SearchResult>;

pub trait SearchEngine {
    fn request(query: &str, count: usize) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized;
    #[must_use]
    fn as_results(&self) -> SearchResults;
}

pub fn apply_headers(req: request::Builder) -> request::Builder {
    req.header("User-Agent", ua_generator::ua::spoof_firefox_windows_ua())
        .header("Accept", "text/html")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Sec-GPC", "1")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Sec-Fetch-User", "?1")
        .header("Priority", "u=0, i")
}
