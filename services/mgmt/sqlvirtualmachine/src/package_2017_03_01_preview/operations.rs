#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod availability_group_listeners {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        availability_group_listener_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<AvailabilityGroupListener, get::Error> {
        let client = &configuration.client;
        let uri_str = & format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}/availabilityGroupListeners/{}" , & configuration . base_path , subscription_id , resource_group_name , sql_virtual_machine_group_name , availability_group_listener_name) ;
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
                let rsp_value: AvailabilityGroupListener = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => get::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        availability_group_listener_name: &str,
        parameters: &AvailabilityGroupListener,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &configuration.client;
        let uri_str = & format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}/availabilityGroupListeners/{}" , & configuration . base_path , subscription_id , resource_group_name , sql_virtual_machine_group_name , availability_group_listener_name) ;
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: AvailabilityGroupListener =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: AvailabilityGroupListener =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => create_or_update::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(AvailabilityGroupListener),
            Created201(AvailabilityGroupListener),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        availability_group_listener_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &configuration.client;
        let uri_str = & format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}/availabilityGroupListeners/{}" , & configuration . base_path , subscription_id , resource_group_name , sql_virtual_machine_group_name , availability_group_listener_name) ;
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
            status_code => delete::DefaultResponse { status_code }.fail(),
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
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_by_group(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<AvailabilityGroupListenerListResult, list_by_group::Error> {
        let client = &configuration.client;
        let uri_str = & format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}/availabilityGroupListeners" , & configuration . base_path , subscription_id , resource_group_name , sql_virtual_machine_group_name) ;
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list_by_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_group::ResponseBytesError)?;
                let rsp_value: AvailabilityGroupListenerListResult =
                    serde_json::from_slice(&body).context(list_by_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => list_by_group::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod list_by_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
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
        let uri_str = &format!("{}/providers/Microsoft.SqlVirtualMachine/operations", &configuration.base_path,);
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
            status_code => list::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
pub mod sql_virtual_machine_groups {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<SqlVirtualMachineGroup, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_group_name
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
                let rsp_value: SqlVirtualMachineGroup = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => get::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        parameters: &SqlVirtualMachineGroup,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_group_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachineGroup =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachineGroup =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => create_or_update::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(SqlVirtualMachineGroup),
            Created201(SqlVirtualMachineGroup),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        parameters: &SqlVirtualMachineGroupUpdate,
        subscription_id: &str,
    ) -> std::result::Result<SqlVirtualMachineGroup, update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_group_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachineGroup = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => update::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_group_name
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
            status_code => delete::DefaultResponse { status_code }.fail(),
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
            DefaultResponse { status_code: StatusCode },
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
    ) -> std::result::Result<SqlVirtualMachineGroupListResult, list_by_resource_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups",
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
                let rsp_value: SqlVirtualMachineGroupListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => list_by_resource_group::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list(
        configuration: &crate::Configuration,
        subscription_id: &str,
    ) -> std::result::Result<SqlVirtualMachineGroupListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups",
            &configuration.base_path, subscription_id
        );
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
                let rsp_value: SqlVirtualMachineGroupListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => list::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
pub mod sql_virtual_machines {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list_by_sql_vm_group(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<SqlVirtualMachineListResult, list_by_sql_vm_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/{}/sqlVirtualMachines",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        let req = req_builder.build().context(list_by_sql_vm_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_sql_vm_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_sql_vm_group::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachineListResult =
                    serde_json::from_slice(&body).context(list_by_sql_vm_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => list_by_sql_vm_group::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod list_by_sql_vm_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list(
        configuration: &crate::Configuration,
        subscription_id: &str,
    ) -> std::result::Result<SqlVirtualMachineListResult, list::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines",
            &configuration.base_path, subscription_id
        );
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
                let rsp_value: SqlVirtualMachineListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => list::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn get(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_name: &str,
        expand: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<SqlVirtualMachine, get::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        if let Some(expand) = expand {
            req_builder = req_builder.query(&[("$expand", expand)]);
        }
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachine = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => get::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_name: &str,
        parameters: &SqlVirtualMachine,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachine = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachine = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => create_or_update::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(SqlVirtualMachine),
            Created201(SqlVirtualMachine),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn update(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_name: &str,
        parameters: &SqlVirtualMachineUpdate,
        subscription_id: &str,
    ) -> std::result::Result<SqlVirtualMachine, update::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &configuration.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: SqlVirtualMachine = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => update::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        configuration: &crate::Configuration,
        resource_group_name: &str,
        sql_virtual_machine_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}",
            &configuration.base_path, subscription_id, resource_group_name, sql_virtual_machine_name
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
            status_code => delete::DefaultResponse { status_code }.fail(),
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
            DefaultResponse { status_code: StatusCode },
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
    ) -> std::result::Result<SqlVirtualMachineListResult, list_by_resource_group::Error> {
        let client = &configuration.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines",
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
                let rsp_value: SqlVirtualMachineListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => list_by_resource_group::DefaultResponse { status_code }.fail(),
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse { status_code: StatusCode },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}
