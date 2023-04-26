use serde::{Serialize, Deserialize};
use regex::RegexSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessData {
    pub paths: Vec<String>,
    pub servers: Vec<String>,
}

impl AccessData {
    pub fn file_access_authorized(&self, path: &str) -> bool {
        Self::access_authorized(self.paths.clone(), path)
    }

    pub fn server_access_authorized(&self, server: &str) -> bool {
        Self::access_authorized(self.servers.clone(), server)
    }

    fn access_authorized(patterns: Vec<String>, content: &str) -> bool {
        match RegexSet::new(&patterns) {
            Ok(re) => re.matches(content).matched_any(),
            Err(e) => {
                error!(
                    "Invalid regex `{:?}` was granted: {}",
                    patterns, e,
                );
                // In the case that there is an invalid pattern,
                // we assume no access.
                false
            }
        }
    }
}
