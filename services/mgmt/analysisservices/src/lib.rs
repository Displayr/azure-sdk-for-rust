#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2017-08")]
mod package_2017_08;
#[cfg(feature = "package-2017-08")]
pub use package_2017_08::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-08-beta")]
mod package_2017_08_beta;
#[cfg(feature = "package-2017-08-beta")]
pub use package_2017_08_beta::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-07")]
mod package_2017_07;
#[cfg(feature = "package-2017-07")]
pub use package_2017_07::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-05")]
mod package_2016_05;
#[cfg(feature = "package-2016-05")]
pub use package_2016_05::{models, operations, API_VERSION};
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
