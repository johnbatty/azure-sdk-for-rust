#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Information about managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The managed application properties."]
    pub properties: ApplicationProperties,
    #[doc = "Plan for the managed application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "The kind of the managed application. Allowed values are MarketPlace and ServiceCatalog."]
    pub kind: String,
}
impl Application {
    pub fn new(properties: ApplicationProperties, kind: String) -> Self {
        Self {
            generic_resource: GenericResource::default(),
            properties,
            plan: None,
            kind,
        }
    }
}
#[doc = "Managed application artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationArtifact {
    #[doc = "The managed application artifact name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The managed application artifact blob uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The managed application artifact type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ApplicationArtifactType>,
}
impl ApplicationArtifact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application artifact type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactType {
    Template,
    Custom,
}
#[doc = "Information about managed application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinition {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The managed application definition properties."]
    pub properties: ApplicationDefinitionProperties,
}
impl ApplicationDefinition {
    pub fn new(properties: ApplicationDefinitionProperties) -> Self {
        Self {
            generic_resource: GenericResource::default(),
            properties,
        }
    }
}
#[doc = "List of managed application definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationDefinitionListResult {
    #[doc = "The array of managed application definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ApplicationDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionProperties {
    #[doc = "The managed application lock level."]
    #[serde(rename = "lockLevel")]
    pub lock_level: ApplicationLockLevel,
    #[doc = "The managed application definition display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A value indicating whether the package is enabled or not."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<String>,
    #[doc = "The managed application provider authorizations."]
    pub authorizations: Vec<ApplicationProviderAuthorization>,
    #[doc = "The collection of managed application artifacts. The portal will use the files specified as artifacts to construct the user experience of creating a managed application from a managed application definition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplicationArtifact>,
    #[doc = "The managed application definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The managed application definition package file Uri. Use this element"]
    #[serde(rename = "packageFileUri", default, skip_serializing_if = "Option::is_none")]
    pub package_file_uri: Option<String>,
    #[doc = "The inline main template json which has resources to be provisioned. It can be a JObject or well-formed JSON string."]
    #[serde(rename = "mainTemplate", default, skip_serializing_if = "Option::is_none")]
    pub main_template: Option<serde_json::Value>,
    #[doc = "The createUiDefinition json for the backing template with Microsoft.Solutions/applications resource. It can be a JObject or well-formed JSON string."]
    #[serde(rename = "createUiDefinition", default, skip_serializing_if = "Option::is_none")]
    pub create_ui_definition: Option<serde_json::Value>,
}
impl ApplicationDefinitionProperties {
    pub fn new(lock_level: ApplicationLockLevel, authorizations: Vec<ApplicationProviderAuthorization>) -> Self {
        Self {
            lock_level,
            display_name: None,
            is_enabled: None,
            authorizations,
            artifacts: Vec::new(),
            description: None,
            package_file_uri: None,
            main_template: None,
            create_ui_definition: None,
        }
    }
}
#[doc = "List of managed applications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationListResult {
    #[doc = "The array of managed applications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ApplicationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application lock level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationLockLevel {
    CanNotDelete,
    ReadOnly,
    None,
}
#[doc = "Information about managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPatchable {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The managed application properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationPropertiesPatchable>,
    #[doc = "Plan for the managed application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanPatchable>,
    #[doc = "The kind of the managed application. Allowed values are MarketPlace and ServiceCatalog."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl ApplicationPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[doc = "The managed resource group Id."]
    #[serde(rename = "managedResourceGroupId")]
    pub managed_resource_group_id: String,
    #[doc = "The fully qualified path of managed application definition Id."]
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[doc = "Name and value pairs that define the managed application parameters. It can be a JObject or a well formed JSON string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Name and value pairs that define the managed application outputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "Provisioning status of the managed application."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ApplicationProperties {
    pub fn new(managed_resource_group_id: String) -> Self {
        Self {
            managed_resource_group_id,
            application_definition_id: None,
            parameters: None,
            outputs: None,
            provisioning_state: None,
        }
    }
}
#[doc = "The managed application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPropertiesPatchable {
    #[doc = "The managed resource group Id."]
    #[serde(rename = "managedResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[doc = "The fully qualified path of managed application definition Id."]
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[doc = "Name and value pairs that define the managed application parameters. It can be a JObject or a well formed JSON string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Name and value pairs that define the managed application outputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "Provisioning status of the managed application."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ApplicationPropertiesPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application provider authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProviderAuthorization {
    #[doc = "The provider's principal identifier. This is the identity that the provider will use to call ARM to manage the managed application resources."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The provider's role definition identifier. This role will define all the permissions that the provider must have on the managed application's container resource group. This role definition cannot have permission to delete the resource group."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
impl ApplicationProviderAuthorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            role_definition_id,
        }
    }
}
#[doc = "Error response indicates managed application is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Http status code."]
    #[serde(rename = "httpStatus", default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "ID of the resource that manages this resource."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "SKU for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl GenericResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "Microsoft.Solutions operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
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
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Solutions"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Application, JitRequest, etc."]
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
#[doc = "Result of the request to list Microsoft.Solutions operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Microsoft.Solutions operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Plan for the managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[doc = "The plan name."]
    pub name: String,
    #[doc = "The publisher ID."]
    pub publisher: String,
    #[doc = "The product code."]
    pub product: String,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The plan's version."]
    pub version: String,
}
impl Plan {
    pub fn new(name: String, publisher: String, product: String, version: String) -> Self {
        Self {
            name,
            publisher,
            product,
            promotion_code: None,
            version,
        }
    }
}
#[doc = "Plan for the managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanPatchable {
    #[doc = "The plan name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The product code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The plan's version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PlanPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning status of the managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Updating,
}
#[doc = "Resource information."]
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
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The SKU name."]
    pub name: String,
    #[doc = "The SKU tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The SKU size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The SKU family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The SKU model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "The SKU capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            model: None,
            capacity: None,
        }
    }
}
