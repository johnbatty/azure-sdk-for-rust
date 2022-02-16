#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "API connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiConnectionDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<api_connection_definition::Properties>,
}
impl ApiConnectionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_connection_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Display name"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Status of the connection"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub statuses: Vec<ConnectionStatusDefinition>,
        #[doc = "Dictionary of parameter values"]
        #[serde(rename = "parameterValues", default, skip_serializing_if = "Option::is_none")]
        pub parameter_values: Option<serde_json::Value>,
        #[doc = "Dictionary of custom parameter values"]
        #[serde(rename = "customParameterValues", default, skip_serializing_if = "Option::is_none")]
        pub custom_parameter_values: Option<serde_json::Value>,
        #[doc = "Dictionary of nonsecret parameter values"]
        #[serde(rename = "nonSecretParameterValues", default, skip_serializing_if = "Option::is_none")]
        pub non_secret_parameter_values: Option<serde_json::Value>,
        #[doc = "Timestamp of the connection creation"]
        #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
        pub created_time: Option<String>,
        #[doc = "Timestamp of last connection change"]
        #[serde(rename = "changedTime", default, skip_serializing_if = "Option::is_none")]
        pub changed_time: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub api: Option<ApiReference>,
        #[doc = "Links to test the API connection"]
        #[serde(rename = "testLinks", default, skip_serializing_if = "Vec::is_empty")]
        pub test_links: Vec<ApiConnectionTestLink>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of API connection definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiConnectionDefinitionCollection {
    #[doc = "Collection of API connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiConnectionDefinition>,
}
impl ApiConnectionDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API connection properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiConnectionTestLink {
    #[doc = "Test link request URI"]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "HTTP Method"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
impl ApiConnectionTestLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OAuth settings for the connection provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOAuthSettings {
    #[doc = "Identity provider"]
    #[serde(rename = "identityProvider", default, skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
    #[doc = "Resource provider client id"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Client Secret needed for OAuth"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "OAuth scopes"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[doc = "Url"]
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[doc = "Read only properties for this oauth setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "OAuth parameters key is the name of parameter"]
    #[serde(rename = "customParameters", default, skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<serde_json::Value>,
}
impl ApiOAuthSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OAuth settings for the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOAuthSettingsParameter {
    #[doc = "Value of the setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Options available to this parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "UI definitions per culture as caller can specify the culture"]
    #[serde(rename = "uiDefinition", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition: Option<serde_json::Value>,
}
impl ApiOAuthSettingsParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The JSON representation of the swagger"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swagger: Option<serde_json::Value>,
    #[doc = "Brand color"]
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[doc = "The custom API description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The icon URI"]
    #[serde(rename = "iconUri", default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[doc = "The name of the API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ApiReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API backend service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceBackendService {
    #[doc = "The service URL"]
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
}
impl ApiResourceBackendService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceDefinitions {
    #[doc = "The original swagger URL"]
    #[serde(rename = "originalSwaggerUrl", default, skip_serializing_if = "Option::is_none")]
    pub original_swagger_url: Option<String>,
    #[doc = "The modified swagger URL"]
    #[serde(rename = "modifiedSwaggerUrl", default, skip_serializing_if = "Option::is_none")]
    pub modified_swagger_url: Option<String>,
}
impl ApiResourceDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "General information about the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceGeneralInformation {
    #[doc = "The icon URL"]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The API description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "URL to the Terms of Use"]
    #[serde(rename = "termsOfUseUrl", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use_url: Option<String>,
    #[doc = "Release tag"]
    #[serde(rename = "releaseTag", default, skip_serializing_if = "Option::is_none")]
    pub release_tag: Option<String>,
}
impl ApiResourceGeneralInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceMetadata {
    #[doc = "The source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Brand color"]
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[doc = "Hide key"]
    #[serde(rename = "hideKey", default, skip_serializing_if = "Option::is_none")]
    pub hide_key: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsDictionary>,
    #[doc = "The API type"]
    #[serde(rename = "apiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<ApiType>,
    #[doc = "The service with name and endpoint names"]
    #[serde(rename = "wsdlService", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_service: Option<WsdlService>,
    #[doc = "The WSDL import method"]
    #[serde(rename = "wsdlImportMethod", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_import_method: Option<WsdlImportMethod>,
    #[doc = "The connection type"]
    #[serde(rename = "connectionType", default, skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
}
impl ApiResourceMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the API policies either as embedded content or as a link to uploaded content"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourcePolicies {
    #[doc = "API level policies as XML"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "Link to the JSON of the policies"]
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<String>,
}
impl ApiResourcePolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Connection parameters"]
    #[serde(rename = "connectionParameters", default, skip_serializing_if = "Option::is_none")]
    pub connection_parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ApiResourceMetadata>,
    #[doc = "Runtime URLs"]
    #[serde(rename = "runtimeUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_urls: Vec<String>,
    #[doc = "General information about the API"]
    #[serde(rename = "generalInformation", default, skip_serializing_if = "Option::is_none")]
    pub general_information: Option<ApiResourceGeneralInformation>,
    #[doc = "The managed API capabilities"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[doc = "The JSON representation of the swagger"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swagger: Option<serde_json::Value>,
    #[doc = "The API backend service"]
    #[serde(rename = "backendService", default, skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<ApiResourceBackendService>,
    #[doc = "Describes the API policies either as embedded content or as a link to uploaded content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<ApiResourcePolicies>,
    #[doc = "URL where the swagger can be downloaded from"]
    #[serde(rename = "apiDefinitionUrl", default, skip_serializing_if = "Option::is_none")]
    pub api_definition_url: Option<String>,
    #[doc = "API Definitions"]
    #[serde(rename = "apiDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub api_definitions: Option<ApiResourceDefinitions>,
}
impl ApiResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiType {
    NotSpecified,
    Rest,
    Soap,
}
#[doc = "Confirm consent code request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfirmConsentCodeDefinition {
    #[doc = "Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "AAD object ID. This is userId"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Code that was returned during consent flow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl ConfirmConsentCodeDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionError {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connection_error::Properties>,
}
impl ConnectionError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Code of the status"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Description of the status"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The gateway definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connection_gateway_definition::Properties>,
}
impl ConnectionGatewayDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_gateway_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The gateway installation reference"]
        #[serde(rename = "connectionGatewayInstallation", default, skip_serializing_if = "Option::is_none")]
        pub connection_gateway_installation: Option<ConnectionGatewayReference>,
        #[doc = "The gateway admin"]
        #[serde(rename = "contactInformation", default, skip_serializing_if = "Vec::is_empty")]
        pub contact_information: Vec<String>,
        #[doc = "The gateway display name"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "The gateway description"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "The machine name of the gateway"]
        #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
        pub machine_name: Option<String>,
        #[doc = "The gateway status"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<serde_json::Value>,
        #[doc = "The URI of the backend"]
        #[serde(rename = "backendUri", default, skip_serializing_if = "Option::is_none")]
        pub backend_uri: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of connection gateway definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayDefinitionCollection {
    #[doc = "Collection of connection gateways"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectionGatewayDefinition>,
}
impl ConnectionGatewayDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The gateway installation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayInstallationDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connection_gateway_installation_definition::Properties>,
}
impl ConnectionGatewayInstallationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_gateway_installation_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The gateway installation reference"]
        #[serde(rename = "connectionGateway", default, skip_serializing_if = "Option::is_none")]
        pub connection_gateway: Option<ConnectionGatewayReference>,
        #[doc = "The gateway admin"]
        #[serde(rename = "contactInformation", default, skip_serializing_if = "Vec::is_empty")]
        pub contact_information: Vec<String>,
        #[doc = "The gateway display name"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "The gateway description"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "The machine name of the gateway"]
        #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
        pub machine_name: Option<String>,
        #[doc = "The gateway status"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<serde_json::Value>,
        #[doc = "The URI of the backend"]
        #[serde(rename = "backendUri", default, skip_serializing_if = "Option::is_none")]
        pub backend_uri: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of connection gateway installation definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayInstallationDefinitionCollection {
    #[doc = "Collection of connection gateway installations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectionGatewayInstallationDefinition>,
}
impl ConnectionGatewayInstallationDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The gateway installation reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "Resource reference location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource reference name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ConnectionGatewayReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection provider parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionParameter {
    #[doc = "Type of the parameter"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<connection_parameter::Type>,
    #[doc = "OAuth settings for the connection provider"]
    #[serde(rename = "oAuthSettings", default, skip_serializing_if = "Option::is_none")]
    pub o_auth_settings: Option<ApiOAuthSettings>,
}
impl ConnectionParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_parameter {
    use super::*;
    #[doc = "Type of the parameter"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "securestring")]
        Securestring,
        #[serde(rename = "secureobject")]
        Secureobject,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "object")]
        Object,
        #[serde(rename = "array")]
        Array,
        #[serde(rename = "oauthSetting")]
        OauthSetting,
        #[serde(rename = "connection")]
        Connection,
    }
}
#[doc = "Connection status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionStatusDefinition {
    #[doc = "The gateway status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Target of the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Connection error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConnectionError>,
}
impl ConnectionStatusDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of consent links"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsentLinkCollection {
    #[doc = "Collection of resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConsentLinkDefinition>,
}
impl ConsentLinkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A consent link"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsentLinkDefinition {
    #[doc = "URI for the consent link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[doc = "URI for first party login"]
    #[serde(rename = "firstPartyLoginUri", default, skip_serializing_if = "Option::is_none")]
    pub first_party_login_uri: Option<String>,
    #[doc = "Display name of the parameter in the connection provider's OAuth settings"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Status of the link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<consent_link_definition::Status>,
}
impl ConsentLinkDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod consent_link_definition {
    use super::*;
    #[doc = "Status of the link"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unauthenticated,
        Authenticated,
        Error,
    }
}
#[doc = "Consent link definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsentLinkParameterDefinition {
    #[doc = "Name of the parameter in the connection provider's OAuth settings"]
    #[serde(rename = "parameterName", default, skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[doc = "Name of the parameter in the connection provider's OAuth settings"]
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[doc = "AAD OID (user or group) if the principal type is ActiveDirectory. MSA PUID if the principal type is MicrosoftAccount"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ConsentLinkParameterDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[doc = "Custom API properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomApiPropertiesDefinition>,
}
impl CustomApiDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of custom API definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiDefinitionCollection {
    #[doc = "Collection of custom APIs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomApiDefinition>,
}
impl CustomApiDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom API properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiPropertiesDefinition {
    #[doc = "Connection parameters"]
    #[serde(rename = "connectionParameters", default, skip_serializing_if = "Option::is_none")]
    pub connection_parameters: Option<serde_json::Value>,
    #[doc = "Runtime URLs"]
    #[serde(rename = "runtimeUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_urls: Vec<String>,
    #[doc = "The custom API capabilities"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[doc = "The JSON representation of the swagger"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swagger: Option<serde_json::Value>,
    #[doc = "Brand color"]
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[doc = "The custom API description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The icon URI"]
    #[serde(rename = "iconUri", default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[doc = "The API backend service"]
    #[serde(rename = "backendService", default, skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<ApiResourceBackendService>,
    #[doc = "API Definitions"]
    #[serde(rename = "apiDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub api_definitions: Option<ApiResourceDefinitions>,
    #[doc = "The API type"]
    #[serde(rename = "apiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<ApiType>,
    #[doc = "The WSDL definition"]
    #[serde(rename = "wsdlDefinition", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_definition: Option<WsdlDefinition>,
}
impl CustomApiPropertiesDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The custom API reference"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiReference {
    #[serde(flatten)]
    pub api_reference: ApiReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CustomApiReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List connection keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListConnectionKeysDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<list_connection_keys_definition::Properties>,
}
impl ListConnectionKeysDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod list_connection_keys_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Time span for how long the keys will be valid"]
        #[serde(rename = "validityTimeSpan", default, skip_serializing_if = "Option::is_none")]
        pub validity_time_span: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Request for a list of consent links"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListConsentLinksDefinition {
    #[doc = "Collection of resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ConsentLinkParameterDefinition>,
}
impl ListConsentLinksDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed API definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApiDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[doc = "API resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiResourceProperties>,
}
impl ManagedApiDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of managed API definitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApiDefinitionCollection {
    #[doc = "Collection of managed APIs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedApiDefinition>,
}
impl ManagedApiDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceDefinition {
    #[doc = "Resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource ETag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsDictionary>,
}
impl ResourceDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "Resource reference id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource reference type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsDictionary {}
impl TagsDictionary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The WSDL definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsdlDefinition {
    #[doc = "The WSDL URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The WSDL content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The service with name and endpoint names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<WsdlService>,
    #[doc = "The WSDL import method"]
    #[serde(rename = "importMethod", default, skip_serializing_if = "Option::is_none")]
    pub import_method: Option<WsdlImportMethod>,
}
impl WsdlDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The WSDL import method"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WsdlImportMethod {
    NotSpecified,
    SoapToRest,
    SoapPassThrough,
}
#[doc = "The service with name and endpoint names"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WsdlService {
    #[doc = "The service's qualified name"]
    #[serde(rename = "qualifiedName")]
    pub qualified_name: String,
    #[doc = "List of the endpoints' qualified names"]
    #[serde(rename = "endpointQualifiedNames", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_qualified_names: Vec<String>,
}
impl WsdlService {
    pub fn new(qualified_name: String) -> Self {
        Self {
            qualified_name,
            endpoint_qualified_names: Vec::new(),
        }
    }
}
#[doc = "A list of custom API WSDL interfaces"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsdlServiceCollection {
    #[doc = "Collection of WSDL interfaces"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WsdlService>,
}
impl WsdlServiceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
