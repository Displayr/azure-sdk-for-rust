#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2018-06-01-preview")]
mod package_2018_06_01_preview;
#[cfg(feature = "package-2018-06-01-preview")]
pub use package_2018_06_01_preview::{models, operations, API_VERSION};
pub struct Configuration {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub bearer_access_token: Option<String>,
}
impl Configuration {
    pub fn new(bearer_access_token: &str) -> Self {
        Self {
            bearer_access_token: Some(bearer_access_token.to_owned()),
            ..Default::default()
        }
    }
}
impl Default for Configuration {
    fn default() -> Self {
        Self {
            api_version: API_VERSION.to_owned(),
            client: reqwest::Client::new(),
            base_path: "https://management.azure.com".to_owned(),
            bearer_access_token: None,
        }
    }
}
