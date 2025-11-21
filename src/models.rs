use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bang {
    pub url: String,
    pub keys: Vec<String>,
    pub search_context: Option<String>,
}

impl Bang {
    pub fn new(url: &str, keys: &[&str]) -> Self {
        Bang {
            url: url.to_string(),
            keys: keys.iter().map(|s| s.to_string()).collect(),
            search_context: None,
        }
    }

    pub fn with_context(url: &str, search_context: &str, keys: &[&str]) -> Self {
        Bang {
            url: url.to_string(),
            keys: keys.iter().map(|s| s.to_string()).collect(),
            search_context: Some(search_context.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub query: String,
    pub keys: Vec<String>,
}

impl Query {
    pub fn new(query: &str, keys: &[&str]) -> Self {
        Query {
            query: query.to_string(),
            keys: keys.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Referral {
    pub hostname: String,
    pub browser_id: String,
    pub referral: String,
}

impl Referral {
    pub fn new(hostname: &str, browser_id: &str, referral: &str) -> Self {
        Referral {
            hostname: hostname.to_string(),
            browser_id: browser_id.to_string(),
            referral: referral.to_string(),
        }
    }
}
