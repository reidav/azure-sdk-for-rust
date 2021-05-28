#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-03")]
mod package_2021_03;
#[cfg(feature = "package-2021-03")]
pub use package_2021_03::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2021-03")]
mod package_preview_2021_03;
#[cfg(feature = "package-preview-2021-03")]
pub use package_preview_2021_03::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2021-02")]
mod package_preview_2021_02;
#[cfg(feature = "package-preview-2021-02")]
pub use package_preview_2021_02::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-08-31")]
mod package_2020_08_31;
#[cfg(feature = "package-2020-08-31")]
pub use package_2020_08_31::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2020-08-31")]
mod package_preview_2020_08_31;
#[cfg(feature = "package-preview-2020-08-31")]
pub use package_preview_2020_08_31::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-08")]
mod package_2020_08;
#[cfg(feature = "package-2020-08")]
pub use package_2020_08::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2020-07")]
mod package_preview_2020_07;
#[cfg(feature = "package-preview-2020-07")]
pub use package_preview_2020_07::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-06")]
mod package_2020_06;
#[cfg(feature = "package-2020-06")]
pub use package_2020_06::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-04")]
mod package_2020_04;
#[cfg(feature = "package-2020-04")]
pub use package_2020_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-03")]
mod package_2020_03;
#[cfg(feature = "package-2020-03")]
pub use package_2020_03::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-11")]
mod package_2019_11;
#[cfg(feature = "package-2019-11")]
pub use package_2019_11::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2019-07")]
mod package_preview_2019_07;
#[cfg(feature = "package-preview-2019-07")]
pub use package_preview_2019_07::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-03")]
mod package_2019_03;
#[cfg(feature = "package-2019-03")]
pub use package_2019_03::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2019-03")]
mod package_preview_2019_03;
#[cfg(feature = "package-preview-2019-03")]
pub use package_preview_2019_03::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-12-preview")]
mod package_2018_12_preview;
#[cfg(feature = "package-2018-12-preview")]
pub use package_2018_12_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-04")]
mod package_2018_04;
#[cfg(feature = "package-2018-04")]
pub use package_2018_04::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-01")]
mod package_2018_01;
#[cfg(feature = "package-2018-01")]
pub use package_2018_01::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-07")]
mod package_2017_07;
#[cfg(feature = "package-2017-07")]
pub use package_2017_07::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-01")]
mod package_2017_01;
#[cfg(feature = "package-2017-01")]
pub use package_2017_01::{models, operations, API_VERSION};
#[cfg(feature = "package-2016-02")]
mod package_2016_02;
#[cfg(feature = "package-2016-02")]
pub use package_2016_02::{models, operations, API_VERSION};
#[cfg(feature = "profile-hybrid-2020-09-01")]
mod profile_hybrid_2020_09_01;
use azure_core::setters;
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub use profile_hybrid_2020_09_01::{models, operations, API_VERSION};
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