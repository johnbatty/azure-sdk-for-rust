#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Input of CheckNameAvailability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    #[doc = "The resource name to validate."]
    pub name: String,
    #[doc = "Type of CDN resource used in CheckNameAvailability."]
    #[serde(rename = "type")]
    pub type_: ResourceType,
}
impl CheckNameAvailabilityInput {
    pub fn new(name: String, type_: ResourceType) -> Self {
        Self { name, type_ }
    }
}
#[doc = "Output of check name availability API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityOutput {
    #[doc = "Indicates whether the name is available."]
    #[serde(rename = "NameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the name is not available."]
    #[serde(rename = "Reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed error message describing why the name is not available."]
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CDN CustomDomain represents a mapping between a user specified domain name and a CDN endpoint. This is to use custom domain names to represent the URLs for branding purposes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomain {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainProperties>,
}
impl CustomDomain {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainListResult {
    #[doc = "List of CDN CustomDomains within an endpoint."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomDomain>,
}
impl CustomDomainListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CustomDomain properties required for custom domain creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainPropertiesParameters>,
}
impl CustomDomainParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainProperties {
    #[doc = "The host name of the custom domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "Resource status of the custom domain."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<custom_domain_properties::ResourceState>,
    #[doc = "Provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl CustomDomainProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            resource_state: None,
            provisioning_state: None,
        }
    }
}
pub mod custom_domain_properties {
    use super::*;
    #[doc = "Resource status of the custom domain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainPropertiesParameters {
    #[doc = "The host name of the custom domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl CustomDomainPropertiesParameters {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[doc = "Deep created origins within a CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOrigin {
    #[doc = "Origin name"]
    pub name: String,
    #[doc = "Properties of deep created origin on a CDN endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeepCreatedOriginProperties>,
}
impl DeepCreatedOrigin {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "Properties of deep created origin on a CDN endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOriginProperties {
    #[doc = "The address of the origin. Domain names, IPv4 addresses, and IPv6 addresses are supported."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "The value of the HTTP port. Must be between 1 and 65535"]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "The value of the HTTPS port. Must be between 1 and 65535"]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
}
impl DeepCreatedOriginProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            http_port: None,
            https_port: None,
        }
    }
}
#[doc = "CDN endpoint is the entity within a CDN profile containing configuration information regarding caching behaviors and origins. The CDN endpoint is exposed using the URL format <endpointname>.azureedge.net by default, but custom domains can also be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
impl Endpoint {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Endpoint properties required for new endpoint creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointCreateParameters {
    #[doc = "Endpoint location"]
    pub location: String,
    #[doc = "Endpoint tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointPropertiesCreateParameters>,
}
impl EndpointCreateParameters {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointListResult {
    #[doc = "List of CDN endpoints within a profile"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Endpoint>,
}
impl EndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointProperties {
    #[doc = "The host name of the endpoint {endpointName}.{DNSZone}"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The host header the CDN provider will send along with content requests to origins. The default value is the host name of the origin."]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "The path used for origin requests."]
    #[serde(rename = "originPath", default, skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[doc = "List of content types on which compression will be applied. The value for the elements should be a valid MIME type."]
    #[serde(rename = "contentTypesToCompress", default, skip_serializing_if = "Vec::is_empty")]
    pub content_types_to_compress: Vec<String>,
    #[doc = "Indicates whether the compression is enabled. Default value is false. If compression is enabled, the content transferred from cdn endpoint to end user will be compressed. The requested content must be larger than 1 byte and smaller than 1 MB."]
    #[serde(rename = "isCompressionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<bool>,
    #[doc = "Indicates whether HTTP traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_http_allowed: Option<bool>,
    #[doc = "Indicates whether https traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpsAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_https_allowed: Option<bool>,
    #[doc = "Defines the query string caching behavior."]
    #[serde(rename = "queryStringCachingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<QueryStringCachingBehavior>,
    #[doc = "The set of origins for the CDN endpoint. When multiple origins exist, the first origin will be used as primary and rest will be used as failover options."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<DeepCreatedOrigin>,
    #[doc = "Resource status of the endpoint."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<endpoint_properties::ResourceState>,
    #[doc = "Provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl EndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod endpoint_properties {
    use super::*;
    #[doc = "Resource status of the endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Deleting,
        Running,
        Starting,
        Stopped,
        Stopping,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointPropertiesCreateParameters {
    #[doc = "The host header CDN provider will send along with content requests to origins. The default value is the host name of the origin."]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "The path used for origin requests."]
    #[serde(rename = "originPath", default, skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[doc = "List of content types on which compression will be applied. The value for the elements should be a valid MIME type."]
    #[serde(rename = "contentTypesToCompress", default, skip_serializing_if = "Vec::is_empty")]
    pub content_types_to_compress: Vec<String>,
    #[doc = "Indicates whether content compression is enabled. Default value is false. If compression is enabled, the content transferred from the CDN endpoint to the end user will be compressed. The requested content must be larger than 1 byte and smaller than 1 MB."]
    #[serde(rename = "isCompressionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<bool>,
    #[doc = "Indicates whether HTTP traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_http_allowed: Option<bool>,
    #[doc = "Indicates whether https traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpsAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_https_allowed: Option<bool>,
    #[doc = "Defines the query string caching behavior."]
    #[serde(rename = "queryStringCachingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<QueryStringCachingBehavior>,
    #[doc = "The set of origins for the CDN endpoint. When multiple origins exist, the first origin will be used as primary and rest will be used as failover options."]
    pub origins: Vec<DeepCreatedOrigin>,
}
impl EndpointPropertiesCreateParameters {
    pub fn new(origins: Vec<DeepCreatedOrigin>) -> Self {
        Self {
            origin_host_header: None,
            origin_path: None,
            content_types_to_compress: Vec::new(),
            is_compression_enabled: None,
            is_http_allowed: None,
            is_https_allowed: None,
            query_string_caching_behavior: None,
            origins,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointPropertiesUpdateParameters {
    #[doc = "The host header the CDN provider will send along with content requests to origins. The default value is the host name of the origin."]
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[doc = "The path used for origin requests."]
    #[serde(rename = "originPath", default, skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[doc = "List of content types on which compression will be applied. The value for the elements should be a valid MIME type."]
    #[serde(rename = "contentTypesToCompress", default, skip_serializing_if = "Vec::is_empty")]
    pub content_types_to_compress: Vec<String>,
    #[doc = "Indicates whether content compression is enabled. Default value is false. If compression is enabled, the content transferred from the CDN endpoint to the end user will be compressed. The requested content must be larger than 1 byte and smaller than 1 MB."]
    #[serde(rename = "isCompressionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<bool>,
    #[doc = "Indicates whether HTTP traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_http_allowed: Option<bool>,
    #[doc = "Indicates whether HTTPS traffic is allowed on the endpoint. Default value is true. At least one protocol (HTTP or HTTPS) must be allowed."]
    #[serde(rename = "isHttpsAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_https_allowed: Option<bool>,
    #[doc = "Defines the query string caching behavior."]
    #[serde(rename = "queryStringCachingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<QueryStringCachingBehavior>,
}
impl EndpointPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Endpoint properties required for new endpoint creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointUpdateParameters {
    #[doc = "Endpoint tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointPropertiesUpdateParameters>,
}
impl EndpointUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters required for endpoint load."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadParameters {
    #[doc = "The path to the content to be loaded. Should describe a file path."]
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
impl LoadParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self { content_paths }
    }
}
#[doc = "CDN REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Cdn"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of CDN operations supported by the CDN resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CDN origin is the source of the content being delivered via CDN. When the edge nodes represented by an endpoint do not have the requested content cached, they attempt to fetch it from one or more of the configured origins."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Origin {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginProperties>,
}
impl Origin {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginListResult {
    #[doc = "List of CDN origins within an endpoint"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Origin>,
}
impl OriginListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Origin properties needed for origin creation or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginPropertiesParameters>,
}
impl OriginParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginProperties {
    #[doc = "The address of the origin. Domain names, IPv4 addresses, and IPv6 addresses are supported."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "The value of the HTTP port. Must be between 1 and 65535."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "The value of the https port. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[doc = "Resource status of the origin."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<origin_properties::ResourceState>,
    #[doc = "Provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl OriginProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            http_port: None,
            https_port: None,
            resource_state: None,
            provisioning_state: None,
        }
    }
}
pub mod origin_properties {
    use super::*;
    #[doc = "Resource status of the origin."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginPropertiesParameters {
    #[doc = "The address of the origin. Domain names, IPv4 addresses, and IPv6 addresses are supported."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "The value of the HTTP port. Must be between 1 and 65535."]
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "The value of the HTTPS port. Must be between 1 and 65535."]
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
}
impl OriginPropertiesParameters {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            http_port: None,
            https_port: None,
        }
    }
}
#[doc = "CDN profile represents the top level resource and the entry point into the CDN API. This allows users to set up a logical grouping of endpoints in addition to creating shared configuration settings and selecting pricing tiers and providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
impl Profile {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Profile properties required for profile creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileCreateParameters {
    #[doc = "Profile location"]
    pub location: String,
    #[doc = "Profile tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfilePropertiesCreateParameters>,
}
impl ProfileCreateParameters {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileListResult {
    #[doc = "List of CDN profiles within a resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Profile>,
}
impl ProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileProperties {
    #[doc = "The SKU (pricing tier) of the CDN profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Resource status of the profile."]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<profile_properties::ResourceState>,
    #[doc = "Provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod profile_properties {
    use super::*;
    #[doc = "Resource status of the profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfilePropertiesCreateParameters {
    #[doc = "The SKU (pricing tier) of the CDN profile."]
    pub sku: Sku,
}
impl ProfilePropertiesCreateParameters {
    pub fn new(sku: Sku) -> Self {
        Self { sku }
    }
}
#[doc = "Profile properties required for profile update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileUpdateParameters {
    #[doc = "Profile tags"]
    pub tags: serde_json::Value,
}
impl ProfileUpdateParameters {
    pub fn new(tags: serde_json::Value) -> Self {
        Self { tags }
    }
}
#[doc = "Provisioning status of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Creating,
    Succeeded,
    Failed,
}
#[doc = "Parameters required for endpoint purge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurgeParameters {
    #[doc = "The path to the content to be purged. Can describe a file path or a wild card directory."]
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
impl PurgeParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self { content_paths }
    }
}
#[doc = "Defines the query string caching behavior."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryStringCachingBehavior {
    IgnoreQueryString,
    BypassCaching,
    UseQueryString,
    NotSet,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of CDN resource used in CheckNameAvailability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "Microsoft.Cdn/Profiles/Endpoints")]
    MicrosoftCdnProfilesEndpoints,
}
#[doc = "The SKU (pricing tier) of the CDN profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Name of the pricing tier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[doc = "Name of the pricing tier"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
        Premium,
    }
}
#[doc = "SSO URI required to login to third party web portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsoUri {
    #[doc = "The URI used to login to third party web portal."]
    #[serde(rename = "ssoUriValue", default, skip_serializing_if = "Option::is_none")]
    pub sso_uri_value: Option<String>,
}
impl SsoUri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM tracked resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    pub tags: serde_json::Value,
}
impl TrackedResource {
    pub fn new(location: String, tags: serde_json::Value) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags,
        }
    }
}
#[doc = "Input of the custom domain to be validated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateCustomDomainInput {
    #[doc = "The host name of the custom domain. Must be a domain name."]
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl ValidateCustomDomainInput {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[doc = "Output of custom domain validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateCustomDomainOutput {
    #[doc = "Indicates whether the custom domain is validated or not."]
    #[serde(rename = "customDomainValidated", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain_validated: Option<bool>,
    #[doc = "The reason why the custom domain is not valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The message describing why the custom domain is not valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateCustomDomainOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
