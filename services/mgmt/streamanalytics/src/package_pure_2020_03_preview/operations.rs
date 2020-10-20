#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod clusters {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
    ) -> std::result::Result<Cluster, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name
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
                let rsp_value: Cluster = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update(
        configuration: &crate::Configuration,
        cluster: &Cluster,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
        if_match: Option<&str>,
        if_none_match: Option<&str>,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(cluster);
        if let Some(if_match) = if_match {
            req_builder = req_builder.header("If-Match", if_match);
        }
        if let Some(if_none_match) = if_none_match {
            req_builder = req_builder.header("If-None-Match", if_none_match);
        }
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: Cluster = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: Cluster = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                create_or_update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(Cluster),
            Created201(Cluster),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn update(
        configuration: &crate::Configuration,
        cluster: &Cluster,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
        if_match: Option<&str>,
    ) -> std::result::Result<update::Response, update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(cluster);
        if let Some(if_match) = if_match {
            req_builder = req_builder.header("If-Match", if_match);
        }
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: Cluster = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => Ok(update::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(Cluster),
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(delete::DeserializeError { body })?;
                delete::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_by_subscription(
        configuration: &crate::Configuration,
        subscription_id: &str,
    ) -> std::result::Result<ClusterListResult, list_by_subscription::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.StreamAnalytics/clusters",
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
                let rsp_value: ClusterListResult =
                    serde_json::from_slice(&body).context(list_by_subscription::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_subscription::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list_by_subscription::DeserializeError { body })?;
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_by_resource_group(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> std::result::Result<ClusterListResult, list_by_resource_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters",
            &configuration.base_path, subscription_id, resource_group_name
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
                let rsp_value: ClusterListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_streaming_jobs(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
    ) -> std::result::Result<ClusterJobListResult, list_streaming_jobs::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}/listStreamingJobs",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list_streaming_jobs::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_streaming_jobs::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_streaming_jobs::ResponseBytesError)?;
                let rsp_value: ClusterJobListResult =
                    serde_json::from_slice(&body).context(list_streaming_jobs::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_streaming_jobs::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list_streaming_jobs::DeserializeError { body })?;
                list_streaming_jobs::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_streaming_jobs {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
pub mod private_endpoints {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
        private_endpoint_name: &str,
    ) -> std::result::Result<PrivateEndpoint, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}/privateEndpoints/{}",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name, private_endpoint_name
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
                let rsp_value: PrivateEndpoint = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
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
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update(
        configuration: &crate::Configuration,
        private_endpoint: &PrivateEndpoint,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
        private_endpoint_name: &str,
        if_match: Option<&str>,
        if_none_match: Option<&str>,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}/privateEndpoints/{}",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name, private_endpoint_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(private_endpoint);
        if let Some(if_match) = if_match {
            req_builder = req_builder.header("If-Match", if_match);
        }
        if let Some(if_none_match) = if_none_match {
            req_builder = req_builder.header("If-None-Match", if_none_match);
        }
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: PrivateEndpoint = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: PrivateEndpoint = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                create_or_update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(PrivateEndpoint),
            Created201(PrivateEndpoint),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
        private_endpoint_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}/privateEndpoints/{}",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name, private_endpoint_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(delete::DeserializeError { body })?;
                delete::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_by_cluster(
        configuration: &crate::Configuration,
        subscription_id: &str,
        resource_group_name: &str,
        cluster_name: &str,
    ) -> std::result::Result<PrivateEndpointListResult, list_by_cluster::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.StreamAnalytics/clusters/{}/privateEndpoints",
            &configuration.base_path, subscription_id, resource_group_name, cluster_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list_by_cluster::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_cluster::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_cluster::ResponseBytesError)?;
                let rsp_value: PrivateEndpointListResult =
                    serde_json::from_slice(&body).context(list_by_cluster::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_cluster::ResponseBytesError)?;
                let rsp_value: Error = serde_json::from_slice(&body).context(list_by_cluster::DeserializeError { body })?;
                list_by_cluster::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_cluster {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode, value: models::Error },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
