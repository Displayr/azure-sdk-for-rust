#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod recommendation_metadata {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(configuration: &crate::Configuration, name: &str) -> std::result::Result<MetadataEntity, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!("{}/providers/Microsoft.Advisor/metadata/{}", &configuration.base_path, name);
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: MetadataEntity = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            StatusCode::NOT_FOUND => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: ArmErrorResponseBody = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                get::NotFound404 { value: rsp_value }.fail()
            }
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
            NotFound404 { value: models::ArmErrorResponseBody },
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list(configuration: &crate::Configuration) -> std::result::Result<MetadataEntityListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!("{}/providers/Microsoft.Advisor/metadata", &configuration.base_path,);
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
                let rsp_value: MetadataEntityListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
pub mod configurations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list_by_subscription(
        configuration: &crate::Configuration,
        subscription_id: &str,
    ) -> std::result::Result<ConfigurationListResult, list_by_subscription::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/configurations",
            &configuration.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list_by_subscription::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_subscription::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_subscription::ResponseBytesError)?;
                let rsp_value: ConfigurationListResult =
                    serde_json::from_slice(&body).context(list_by_subscription::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_subscription::ResponseBytesError)?;
                let rsp_value: ArmErrorResponse = serde_json::from_slice(&body).context(list_by_subscription::DeserializeError { body })?;
                list_by_subscription::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_subscription {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ArmErrorResponse,
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
    pub async fn create_in_subscription(
        configuration: &crate::Configuration,
        config_contract: &ConfigData,
        subscription_id: &str,
        configuration_name: &str,
    ) -> std::result::Result<ConfigData, create_in_subscription::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/configurations/{}",
            &configuration.base_path, subscription_id, configuration_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(config_contract);
        let req = req_builder.build().context(create_in_subscription::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_in_subscription::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_in_subscription::ResponseBytesError)?;
                let rsp_value: ConfigData = serde_json::from_slice(&body).context(create_in_subscription::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_in_subscription::ResponseBytesError)?;
                let rsp_value: ArmErrorResponse =
                    serde_json::from_slice(&body).context(create_in_subscription::DeserializeError { body })?;
                create_in_subscription::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_in_subscription {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ArmErrorResponse,
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
    pub async fn list_by_resource_group(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group: &str,
    ) -> std::result::Result<ConfigurationListResult, list_by_resource_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Advisor/configurations",
            &configuration.base_path, subscription_id, resource_group
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: ConfigurationListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: ArmErrorResponse =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                list_by_resource_group::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ArmErrorResponse,
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
    pub async fn create_in_resource_group(
        configuration: &crate::Configuration,
        config_contract: &ConfigData,
        subscription_id: &str,
        configuration_name: &str,
        resource_group: &str,
    ) -> std::result::Result<ConfigData, create_in_resource_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Advisor/configurations/{}",
            &configuration.base_path, subscription_id, resource_group, configuration_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(config_contract);
        let req = req_builder.build().context(create_in_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_in_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_in_resource_group::ResponseBytesError)?;
                let rsp_value: ConfigData = serde_json::from_slice(&body).context(create_in_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_in_resource_group::ResponseBytesError)?;
                let rsp_value: ArmErrorResponse =
                    serde_json::from_slice(&body).context(create_in_resource_group::DeserializeError { body })?;
                create_in_resource_group::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_in_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::ArmErrorResponse,
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
pub mod recommendations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn generate(configuration: &crate::Configuration, subscription_id: &str) -> std::result::Result<(), generate::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/generateRecommendations",
            &configuration.base_path, subscription_id
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(generate::BuildRequestError)?;
        let rsp = client.execute(req).await.context(generate::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::ACCEPTED => Ok(()),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(generate::ResponseBytesError)?;
                generate::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod generate {
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
    pub async fn get_generate_status(
        configuration: &crate::Configuration,
        subscription_id: &str,
        operation_id: &str,
    ) -> std::result::Result<get_generate_status::Response, get_generate_status::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/generateRecommendations/{}",
            &configuration.base_path, subscription_id, operation_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(get_generate_status::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get_generate_status::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::ACCEPTED => Ok(get_generate_status::Response::Accepted202),
            StatusCode::NO_CONTENT => Ok(get_generate_status::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_generate_status::ResponseBytesError)?;
                get_generate_status::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get_generate_status {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
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
    pub async fn list(
        configuration: &crate::Configuration,
        subscription_id: &str,
        filter: Option<&str>,
        top: Option<i64>,
        skip_token: Option<&str>,
    ) -> std::result::Result<ResourceRecommendationBaseListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/recommendations",
            &configuration.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        if let Some(filter) = filter {
            req_builder = req_builder.query(&[("$filter", filter)]);
        }
        if let Some(top) = top {
            req_builder = req_builder.query(&[("$top", top)]);
        }
        if let Some(skip_token) = skip_token {
            req_builder = req_builder.query(&[("$skipToken", skip_token)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: ResourceRecommendationBaseListResult =
                    serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn get(
        configuration: &crate::Configuration,
        resource_uri: &str,
        recommendation_id: &str,
    ) -> std::result::Result<ResourceRecommendationBase, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}",
            &configuration.base_path, resource_uri, recommendation_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: ResourceRecommendationBase = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
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
    pub async fn list(configuration: &crate::Configuration) -> std::result::Result<OperationEntityListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!("{}/providers/Microsoft.Advisor/operations", &configuration.base_path,);
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
                let rsp_value: OperationEntityListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
pub mod suppressions {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        configuration: &crate::Configuration,
        resource_uri: &str,
        recommendation_id: &str,
        name: &str,
    ) -> std::result::Result<SuppressionContract, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}/suppressions/{}",
            &configuration.base_path, resource_uri, recommendation_id, name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: SuppressionContract = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
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
    pub async fn create(
        configuration: &crate::Configuration,
        resource_uri: &str,
        recommendation_id: &str,
        name: &str,
        suppression_contract: &SuppressionContract,
    ) -> std::result::Result<SuppressionContract, create::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}/suppressions/{}",
            &configuration.base_path, resource_uri, recommendation_id, name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(suppression_contract);
        let req = req_builder.build().context(create::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: SuppressionContract = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                create::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create {
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
    pub async fn delete(
        configuration: &crate::Configuration,
        resource_uri: &str,
        recommendation_id: &str,
        name: &str,
    ) -> std::result::Result<(), delete::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}/suppressions/{}",
            &configuration.base_path, resource_uri, recommendation_id, name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                delete::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete {
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
    pub async fn list(
        configuration: &crate::Configuration,
        subscription_id: &str,
        top: Option<i64>,
        skip_token: Option<&str>,
    ) -> std::result::Result<SuppressionContractListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/suppressions",
            &configuration.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        if let Some(top) = top {
            req_builder = req_builder.query(&[("$top", top)]);
        }
        if let Some(skip_token) = skip_token {
            req_builder = req_builder.query(&[("$skipToken", skip_token)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: SuppressionContractListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
