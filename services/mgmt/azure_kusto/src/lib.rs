#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-01")]
mod package_2021_01;
#[cfg(feature = "package-2021-01")]
pub use package_2021_01::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-09-18")]
mod package_2020_09_18;
#[cfg(feature = "package-2020-09-18")]
pub use package_2020_09_18::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-06-14")]
mod package_2020_06_14;
#[cfg(feature = "package-2020-06-14")]
pub use package_2020_06_14::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-02-15")]
mod package_2020_02_15;
#[cfg(feature = "package-2020-02-15")]
pub use package_2020_02_15::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-11-09")]
mod package_2019_11_09;
#[cfg(feature = "package-2019-11-09")]
pub use package_2019_11_09::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-09-07")]
mod package_2019_09_07;
#[cfg(feature = "package-2019-09-07")]
pub use package_2019_09_07::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-05-15")]
mod package_2019_05_15;
#[cfg(feature = "package-2019-05-15")]
pub use package_2019_05_15::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-01-21")]
mod package_2019_01_21;
#[cfg(feature = "package-2019-01-21")]
pub use package_2019_01_21::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-09-07-preview")]
mod package_2018_09_07_preview;
#[cfg(feature = "package-2018-09-07-preview")]
pub use package_2018_09_07_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-09-07-privatepreview")]
mod package_2017_09_07_privatepreview;
#[cfg(feature = "package-2017-09-07-privatepreview")]
pub use package_2017_09_07_privatepreview::{models, operations, API_VERSION};
#[cfg(feature = "schema-2019-09-07")]
mod schema_2019_09_07;
#[cfg(feature = "schema-2019-09-07")]
pub use schema_2019_09_07::{models, operations, API_VERSION};
#[cfg(feature = "schema-2019-05-15")]
mod schema_2019_05_15;
#[cfg(feature = "schema-2019-05-15")]
pub use schema_2019_05_15::{models, operations, API_VERSION};
#[cfg(feature = "schema-2019-01-21")]
mod schema_2019_01_21;
#[cfg(feature = "schema-2019-01-21")]
pub use schema_2019_01_21::{models, operations, API_VERSION};
#[cfg(feature = "schema-2018-09-07-preview")]
mod schema_2018_09_07_preview;
#[cfg(feature = "schema-2018-09-07-preview")]
pub use schema_2018_09_07_preview::{models, operations, API_VERSION};
#[cfg(feature = "schema-2017-09-07-privatepreview")]
mod schema_2017_09_07_privatepreview;
use azure_core::setters;
#[cfg(feature = "schema-2017-09-07-privatepreview")]
pub use schema_2017_09_07_privatepreview::{models, operations, API_VERSION};
pub fn config(
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        api_version: None,
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    api_version: Option<String>,
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { api_version : String => Some (api_version) , base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            api_version: self.api_version.unwrap_or(API_VERSION.to_owned()),
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or("https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self.token_credential_resource.unwrap_or("https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    api_version: String,
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn api_version(&self) -> &str {
        self.api_version.as_str()
    }
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref().as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}