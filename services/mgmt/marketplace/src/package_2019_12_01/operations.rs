#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod private_store_client {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(configuration: &crate::Configuration, subscription_id: &str) -> std::result::Result<(), get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Marketplace/privateStoreClient/isPrivateClient",
            &configuration.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(()),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
pub mod operations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(configuration: &crate::Configuration) -> std::result::Result<OperationListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!("{}/providers/Microsoft.Marketplace/operations", &configuration.base_path,);
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: OperationListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                list::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ErrorResponse,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
        }
    }
}
