use crate::engines::{
    base::{SearchEngine, SearchResults},
    duckduckgo::DuckDuckGo,
};

#[must_use]
pub fn encode_query(query: &str) -> String {
    urlencoding::encode(query).to_string()
}

pub fn search_duckduckgo(query: &str, count: usize) -> anyhow::Result<SearchResults> {
    Ok(DuckDuckGo::request(query, count)?.as_results())
}
