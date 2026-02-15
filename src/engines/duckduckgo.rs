use std::{collections::HashSet, time::Duration};

use crate::{
    engines::base::{SearchEngine, SearchResults, apply_headers},
    search_result::SearchResult,
};
use select::{
    document::Document,
    predicate::{Class, Name},
};

pub struct DuckDuckGo {
    results: SearchResults,
}

fn extract_next_params(doc: &Document) -> Option<Vec<(String, String)>> {
    let form = doc.find(Class("next_form")).next()?;

    let mut params = vec![];

    for input in form.find(Name("input")) {
        if let (Some(name), Some(value)) = (input.attr("name"), input.attr("value")) {
            params.push((name.to_string(), value.to_string()));
        }
    }

    Some(params)
}

impl SearchEngine for DuckDuckGo {
    fn as_results(&self) -> SearchResults {
        self.results.clone()
    }

    fn request(query: &str, count: usize) -> anyhow::Result<Self> {
        let mut results = vec![];
        let mut rank = 1;

        let mut params = vec![
            ("q".to_string(), query.to_string()),
            ("kl".to_string(), "wt-wt".to_string()),
            ("df".to_string(), String::new()),
        ];

        let mut seen = HashSet::new();

        while results.len() < count {
            let body_string = params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");

            // We create a new client per request to avoid bot suspicion
            let client = reqwest::blocking::Client::new();
            let kl_value = params
                .iter()
                .find(|(k, _)| k == "kl")
                .map_or("wt-wt", |(_, v)| v.as_str());
            let body = apply_headers(
                client
                    .post("https://lite.duckduckgo.com/lite/")
                    .timeout(Duration::from_secs(30)),
            )
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Cookie", format!("kl={kl_value}"))
            .body(body_string)
            .send()?
            .text()?;

            if body.contains("anomaly-modal") {
                return Err(anyhow::anyhow!("Blocked by captcha"));
            }

            let doc = Document::from(body.as_str());
            let mut rows = doc.find(Name("tr"));

            while let Some(row) = rows.next() {
                if let Some(link) = row.find(Class("result-link")).next() {
                    let title = link.text().trim().to_string();
                    let url = link.attr("href").unwrap_or("").to_string();

                    let description = rows
                        .next()
                        .and_then(|r| r.find(Class("result-snippet")).next())
                        .map(|n| n.text().trim().to_string())
                        .unwrap_or_default();

                    rows.next();
                    rows.next();

                    if seen.insert(url.clone()) {
                        results.push(SearchResult::new(title, description, url, rank));
                        rank += 1;
                    }

                    if results.len() >= count {
                        break;
                    }
                }
            }

            if results.len() >= count {
                break;
            }

            match extract_next_params(&doc) {
                Some(next) => params = next,
                None => break,
            }
        }

        Ok(Self { results })
    }
}
