use crate::models::{Bang, Query, Referral};

pub const DEFAULT_BANG_DEFAULT: &str = "g";
pub const FIREFOX: &str = "firefox";
pub const FIREFOX_MOBILE: &str = "firefox-mobile";
pub const VIVALDI: &str = "vivaldi";

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub default_bang: String,
    pub bang_chars: String,
    pub browser_id: Option<String>,
}

impl Settings {
    // Manual JSON serialization to avoid serde_json overhead
    pub fn to_json(&self) -> String {
        let browser_id_str = if let Some(ref id) = self.browser_id {
            format!("\"{}\"", id.replace("\"", "\\\""))
        } else {
            "null".to_string()
        };
        format!(
            "{{\"defaultBang\":\"{}\",\"bangChars\":\"{}\",\"browserId\":{}}}",
            self.default_bang.replace("\"", "\\\""),
            self.bang_chars.replace("\"", "\\\""),
            browser_id_str
        )
    }
    
    // Manual JSON deserialization
    pub fn from_json(json: &str) -> Option<Self> {
        // Simple JSON parsing for our specific structure
        let json = json.trim();
        if !json.starts_with('{') || !json.ends_with('}') {
            return None;
        }
        
        let mut default_bang = String::from("g");
        let mut bang_chars = String::from("!@/");
        let mut browser_id = None;
        
        // Parse each field
        for part in json[1..json.len()-1].split(',') {
            let part = part.trim();
            if let Some(colon_pos) = part.find(':') {
                let key = part[..colon_pos].trim().trim_matches('"');
                let value = part[colon_pos+1..].trim();
                
                match key {
                    "defaultBang" => {
                        if value.starts_with('"') && value.ends_with('"') {
                            default_bang = value[1..value.len()-1].to_string();
                        }
                    }
                    "bangChars" => {
                        if value.starts_with('"') && value.ends_with('"') {
                            bang_chars = value[1..value.len()-1].to_string();
                        }
                    }
                    "browserId" => {
                        if value != "null" && value.starts_with('"') && value.ends_with('"') {
                            browser_id = Some(value[1..value.len()-1].to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
        
        Some(Settings {
            default_bang,
            bang_chars,
            browser_id,
        })
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            default_bang: "g".to_string(),
            bang_chars: "!@/".to_string(),
            browser_id: None,
        }
    }
}

/// I'd like my browser to get paid for my searches (optional, if ID is in settings)
pub fn get_referrals() -> Vec<Referral> {
    vec![
        // Firefox
        Referral::new("google.com", FIREFOX, "client=firefox-b-d"),
        Referral::new("duckduckgo.com", FIREFOX, "t=ffab"),
        // Firefox Mobile
        Referral::new("google.com", FIREFOX_MOBILE, "client=firefox-b-m"),
        Referral::new("duckduckgo.com", FIREFOX_MOBILE, "t=ffab"),
        // Vivaldi
        Referral::new("startpage.com", VIVALDI, "segment=startpage.vivaldi"),
        Referral::new("qwant.com", VIVALDI, "client=brz-vivaldi&t=web"),
        Referral::new("duckduckgo.com", VIVALDI, "t=vivaldi&ia=web"),
    ]
}

/// Limited queries that get combined with the GeneralPurposeEngines specified below.
/// E.g., `wp` becomes `gwp` for Google, `dwp` for DuckDuckGo, etc.
pub fn get_queries() -> Vec<Query> {
    vec![
        Query::new("site:wikipedia.org%20{{{s}}}", &["w", "wp", "wiki", "wikipedia"]),
        Query::new("site:news.ycombinator.com%20{{{s}}}", &["hn", "hackernews"]),
        Query::new("site:reddit.com%20{{{s}}}", &["r", "reddit"]),
        Query::new("site:lobste.rs%20{{{s}}}", &["lobsters"]),
        Query::new("site:github.com%20{{{s}}}", &["github", "gh"]),
        Query::new("site:gist.github.com%20{{{s}}}", &["gist"]),
        Query::new("site:stackoverflow.com%20{{{s}}}", &["so"]),
        Query::new("site:social.alexn.org%20{{{s}}}", &["social"]),
        Query::new("site:alexn.org%20{{{s}}}", &["alexn"]),
    ]
}

/// General-purpose search engines. For specific ones, see get_special_purpose_engines.
pub fn get_general_purpose_engines() -> Vec<Bang> {
    vec![
        // Google
        Bang::new("https://www.google.com/search?q={{{s}}}", &["g", "google"]),
        // Brave
        Bang::new("https://search.brave.com/search?q={{{s}}}", &["b", "br"]),
        // StartPage
        Bang::new("https://www.startpage.com/do/search?query={{{s}}}", &["s", "sp"]),
        // Qwant
        Bang::new("https://www.qwant.com/?q={{{s}}}", &["q", "qw"]),
        // DuckDuckGo
        Bang::new("https://duckduckgo.com/?q={{{s}}}", &["d", "ddg"]),
        // Bing
        Bang::new("https://www.bing.com/search?q={{{s}}}", &["bi", "bing"]),
    ]
}

pub fn get_special_purpose_engines() -> Vec<Bang> {
    vec![
        // Google
        Bang::new("https://google.com/maps/place/{{{s}}}", &["gm", "gmaps"]),
        Bang::new("https://google.com/search?tbm=isch&q={{{s}}}&tbs=imgo:1", &["gi", "gimages"]),
        Bang::new("https://chrome.google.com/webstore/search/{{{s}}}?_category=extensions", &["gws", "gwebstore"]),
        Bang::new("https://www.youtube.com/results?search_query={{{s}}}", &["youtube", "yt", "y"]),
        // Popular
        Bang::new("https://en.wikipedia.org/wiki/Special:Search?search={{{s}}}", &["w", "wikipedia"]),
        Bang::new("https://www.reddit.com/search?q={{{s}}}", &["r", "reddit"]),
        Bang::new("http://www.imdb.com/find?s=all&q={{{s}}}", &["imdb"]),
        Bang::new("https://hn.algolia.com/?q={{{s}}}", &["hn"]),
        Bang::new("https://lobste.rs/search?q={{{s}}}", &["lobsters"]),
        // github
        Bang::new("http://github.com/search?q={{{s}}}&type=Everything&repo=&langOverride=&start_value=1", &["gh", "github"]),
        Bang::new("https://github.com/search?utf8=%E2%9C%93&q={{{s}}}", &["git"]),
        Bang::new("https://gist.github.com/search?q={{{s}}}", &["gist"]),
        Bang::new("https://gist.github.com/search?q=user%3Aalexandru%20{{{s}}}", &["gistm", "mygist"]),
        // Local
        Bang::new("https://dexonline.ro/definitie/{{{s}}}", &["dex", "dexonline"]),
        // Programming
        Bang::new("https://index.scala-lang.org/search?q={{{s}}}", &["scaladex", "scalai"]),
        Bang::new("https://central.sonatype.com/search?q={{{s}}}", &["maven", "mvn"]),
        Bang::new("https://www.scala-lang.org/api/current/index.html?search={{{s}}}", &["scalaapi"]),
        Bang::with_context(
            "https://search.brave.com/goggles?q={{{s}}}&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fscala",
            "scala",
            &["scala", "sc"]
        ),
        Bang::with_context(
            "https://search.brave.com/goggles?q=json&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Fbrave%2Fgoggles-quickstart%2Fmain%2Fgoggles%2Frust_programming.goggle",
            "rust",
            &["rust", "rs"]
        ),
        Bang::with_context(
            "https://search.brave.com/goggles?q={{{s}}}&source=web&goggles_id=https%3A%2F%2Fraw.githubusercontent.com%2Falexandru%2Fbangs%2Fmain%2Fgoogles%2Fkotlin",
            "kotlin",
            &["kotlin", "kt"]
        ),
        // AI/LLM
        Bang::new("https://chatgpt.com/?hints=search&temporary-chat=true&prompt={{{s}}}", &["gpt", "chatgpt", "ai", "aig"]),
        Bang::new("https://chat.mistral.ai/chat?q={{{s}}}", &["mistral", "aim"]),
        Bang::new("https://www.perplexity.ai/search/?q={{{s}}}", &["perplexity", "aip"]),
    ]
}
