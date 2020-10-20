#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod operations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(configuration: &crate::Configuration) -> std::result::Result<OperationListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!("{}/providers/microsoft.visualstudio/operations", &configuration.base_path,);
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
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
pub mod accounts {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn check_name_availability(
        configuration: &crate::Configuration,
        subscription_id: &str,
        body: &CheckNameAvailabilityParameter,
    ) -> std::result::Result<CheckNameAvailabilityResult, check_name_availability::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/microsoft.visualstudio/checkNameAvailability",
            &configuration.base_path, subscription_id
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(body);
        let req = req_builder.build().context(check_name_availability::BuildRequestError)?;
        let rsp = client.execute(req).await.context(check_name_availability::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                let rsp_value: CheckNameAvailabilityResult =
                    serde_json::from_slice(&body).context(check_name_availability::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                check_name_availability::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod check_name_availability {
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
    pub async fn list_by_resource_group(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<AccountResourceListResult, list_by_resource_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account",
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
                let rsp_value: AccountResourceListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                list_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn get(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        resource_name: &str,
    ) -> std::result::Result<AccountResource, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}",
            &configuration.base_path, subscription_id, resource_group_name, resource_name
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
                let rsp_value: AccountResource = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            StatusCode::NOT_FOUND => get::NotFound404 {}.fail(),
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
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        body: &AccountResourceRequest,
        resource_name: &str,
    ) -> std::result::Result<AccountResource, create_or_update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}",
            &configuration.base_path, subscription_id, resource_group_name, resource_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(body);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: AccountResource = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            StatusCode::NOT_FOUND => create_or_update::NotFound404 {}.fail(),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                create_or_update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        body: &AccountTagRequest,
        resource_name: &str,
    ) -> std::result::Result<AccountResource, update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}",
            &configuration.base_path, subscription_id, resource_group_name, resource_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(body);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: AccountResource = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            StatusCode::NOT_FOUND => update::NotFound404 {}.fail(),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        resource_name: &str,
    ) -> std::result::Result<(), delete::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}",
            &configuration.base_path, subscription_id, resource_group_name, resource_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(()),
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
}
pub mod extensions {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list_by_account(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        account_resource_name: &str,
    ) -> std::result::Result<ExtensionResourceListResult, list_by_account::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}/extension",
            &configuration.base_path, subscription_id, resource_group_name, account_resource_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list_by_account::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_account::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_account::ResponseBytesError)?;
                let rsp_value: ExtensionResourceListResult =
                    serde_json::from_slice(&body).context(list_by_account::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_account::ResponseBytesError)?;
                list_by_account::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_by_account {
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
        resource_group_name: &str,
        subscription_id: &str,
        account_resource_name: &str,
        extension_resource_name: &str,
    ) -> std::result::Result<ExtensionResource, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}/extension/{}",
            &configuration.base_path, subscription_id, resource_group_name, account_resource_name, extension_resource_name
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
                let rsp_value: ExtensionResource = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            StatusCode::NOT_FOUND => get::NotFound404 {}.fail(),
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
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        body: &ExtensionResourceRequest,
        account_resource_name: &str,
        extension_resource_name: &str,
    ) -> std::result::Result<ExtensionResource, create::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}/extension/{}",
            &configuration.base_path, subscription_id, resource_group_name, account_resource_name, extension_resource_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(body);
        let req = req_builder.build().context(create::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: ExtensionResource = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
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
    pub async fn update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        body: &ExtensionResourceRequest,
        account_resource_name: &str,
        extension_resource_name: &str,
    ) -> std::result::Result<ExtensionResource, update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}/extension/{}",
            &configuration.base_path, subscription_id, resource_group_name, account_resource_name, extension_resource_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(body);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: ExtensionResource = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod update {
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
        resource_group_name: &str,
        subscription_id: &str,
        account_resource_name: &str,
        extension_resource_name: &str,
    ) -> std::result::Result<(), delete::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.visualstudio/account/{}/extension/{}",
            &configuration.base_path, subscription_id, resource_group_name, account_resource_name, extension_resource_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(()),
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
}
pub mod projects {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list_by_resource_group(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        root_resource_name: &str,
    ) -> std::result::Result<ProjectResourceListResult, list_by_resource_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.visualstudio/account/{}/project",
            &configuration.base_path, subscription_id, resource_group_name, root_resource_name
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
                let rsp_value: ProjectResourceListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                list_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
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
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn get(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        root_resource_name: &str,
        resource_name: &str,
    ) -> std::result::Result<ProjectResource, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.visualstudio/account/{}/project/{}",
            &configuration.base_path, subscription_id, resource_group_name, root_resource_name, resource_name
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
                let rsp_value: ProjectResource = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            StatusCode::NOT_FOUND => get::NotFound404 {}.fail(),
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
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create(
        configuration: &crate::Configuration,
        body: &ProjectResource,
        resource_group_name: &str,
        subscription_id: &str,
        root_resource_name: &str,
        resource_name: &str,
        validating: Option<&str>,
    ) -> std::result::Result<create::Response, create::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.visualstudio/account/{}/project/{}",
            &configuration.base_path, subscription_id, resource_group_name, root_resource_name, resource_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(body);
        if let Some(validating) = validating {
            req_builder = req_builder.query(&[("validating", validating)]);
        }
        let req = req_builder.build().context(create::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: ProjectResource = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => Ok(create::Response::Accepted202),
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(ProjectResource),
            Accepted202,
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
    pub async fn update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        body: &ProjectResource,
        root_resource_name: &str,
        resource_name: &str,
    ) -> std::result::Result<ProjectResource, update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.visualstudio/account/{}/project/{}",
            &configuration.base_path, subscription_id, resource_group_name, root_resource_name, resource_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(body);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: ProjectResource = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod update {
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
    pub async fn get_job_status(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        subscription_id: &str,
        root_resource_name: &str,
        resource_name: &str,
        sub_container_name: &str,
        operation: &str,
        job_id: Option<&str>,
    ) -> std::result::Result<get_job_status::Response, get_job_status::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.visualstudio/account/{}/project/{}/subContainers/{}/status",
            &configuration.base_path, subscription_id, resource_group_name, root_resource_name, resource_name, sub_container_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.query(&[("operation", operation)]);
        if let Some(job_id) = job_id {
            req_builder = req_builder.query(&[("jobId", job_id)]);
        }
        let req = req_builder.build().context(get_job_status::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get_job_status::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_job_status::ResponseBytesError)?;
                let rsp_value: ProjectResource = serde_json::from_slice(&body).context(get_job_status::DeserializeError { body })?;
                Ok(get_job_status::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => Ok(get_job_status::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_job_status::ResponseBytesError)?;
                get_job_status::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get_job_status {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(ProjectResource),
            Accepted202,
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
}
