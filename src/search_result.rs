use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub description: String,
    pub url: String,
    pub rank: usize,
}

impl SearchResult {
    #[must_use]
    pub const fn new(title: String, description: String, url: String, rank: usize) -> Self {
        Self {
            title,
            description,
            url,
            rank,
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {}: {}", self.rank, self.title, self.url)
    }
}
