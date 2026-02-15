use std::time::Duration;

use crate::{
    engines::base::{SearchEngine, SearchResults, apply_headers},
    search_result::SearchResult,
};
use isahc::{Request, prelude::*};
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};

pub struct DuckDuckGo {
    results: SearchResults,
}

impl SearchEngine for DuckDuckGo {
    fn as_results(&self) -> SearchResults {
        self.results.clone()
    }

    fn request(query: &str, count: usize) -> anyhow::Result<Self> {
        let mut results = vec![];
        let body = apply_headers(
            Request::post("https://lite.duckduckgo.com/lite/").timeout(Duration::from_secs(30)),
        )
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Cookie", "kl=wt-wt")
        .body(format!("q={query}&kl=&df="))?
        .send()?
        .text()?;

        if body.contains("anomaly-modal") || body.contains("bots use DuckDuckGo too") {
            return Err(anyhow::anyhow!("Blocked by captcha"));
        }

        let doc = Document::from(body.as_str());
        let mut rows = doc.find(Name("tr"));
        let mut rank = 1;

        while let Some(row) = rows.next() {
            if let Some(link) = row.find(Class("result-link")).next() {
                let title = link.text().trim().to_string();
                let url = link.attr("href").unwrap_or("").to_string();
                let description = rows
                    .next()
                    .and_then(|r: Node<'_>| r.find(Class("result-snippet")).next())
                    .map(|n: Node<'_>| n.text().trim().to_string())
                    .unwrap_or_default();

                rows.next(); // link-text
                rows.next(); // spacer
                results.push(SearchResult::new(title, description, url, rank));
                rank += 1;
            }
        }

        Ok(Self { results })
    }
}
