#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "An identity that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyEntry {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The object ID of a user, service principal or security group in the Azure Active Directory tenant for the vault. The object ID must be unique for the list of access policies."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[doc = " Application ID of the client making request on behalf of a principal"]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Permissions the identity has for keys, secrets, certificates and storage."]
    pub permissions: Permissions,
}
impl AccessPolicyEntry {
    pub fn new(tenant_id: String, object_id: String, permissions: Permissions) -> Self {
        Self {
            tenant_id,
            object_id,
            application_id: None,
            permissions,
        }
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "A boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or is invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason that a vault name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "An error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "The reason that a vault name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[doc = "Deleted vault information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVault {
    #[doc = "The resource ID for the deleted key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of the key vault."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the deleted vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedVaultProperties>,
}
impl DeletedVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of vaults"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVaultListResult {
    #[doc = "The list of deleted vaults."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeletedVault>,
    #[doc = "The URL to get the next set of deleted vaults."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DeletedVaultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the deleted vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedVaultProperties {
    #[doc = "The resource id of the original vault."]
    #[serde(rename = "vaultId", default, skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[doc = "The location of the original vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The deleted date."]
    #[serde(rename = "deletionDate", default, skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<String>,
    #[doc = "The scheduled purged date."]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<String>,
    #[doc = "Tags of the original vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DeletedVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log specification of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of log specification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of specification."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key Vault REST API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The origin of operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of operation, include metric specifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft Key Vault."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Storage operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Storage operations supported by the Storage resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL to get the next set of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of operation, include metric specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "One property of operation, include log specifications."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Permissions the identity has for keys, secrets, certificates and storage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permissions {
    #[doc = "Permissions to keys"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[doc = "Permissions to secrets"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,
    #[doc = "Permissions to certificates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<String>,
    #[doc = "Permissions to storage accounts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<String>,
}
impl Permissions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key Vault resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The Azure Resource Manager resource ID for the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type of the key vault."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The supported Azure location where the key vault should be created."]
    pub location: String,
    #[doc = "The tags that will be assigned to the key vault. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "List of vault resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListResult {
    #[doc = "The list of vault resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Resource>,
    #[doc = "The URL to get the next set of vault resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "One property of operation, include log specifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Log specifications of operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "SKU family name"]
    pub family: sku::Family,
    #[doc = "SKU name to specify whether the key vault is a standard vault or a premium vault."]
    pub name: sku::Name,
}
impl Sku {
    pub fn new(family: sku::Family, name: sku::Name) -> Self {
        Self { family, name }
    }
}
pub mod sku {
    use super::*;
    #[doc = "SKU family name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        A,
    }
    #[doc = "SKU name to specify whether the key vault is a standard vault or a premium vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "premium")]
        Premium,
    }
}
#[doc = "Resource information with extended details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the vault"]
    pub properties: VaultProperties,
}
impl Vault {
    pub fn new(resource: Resource, properties: VaultProperties) -> Self {
        Self { resource, properties }
    }
}
#[doc = "Parameters for updating the access policy in a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyParameters {
    #[doc = "The resource id of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource name of the access policy."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource type of the access policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of the vault access policy"]
    pub properties: VaultAccessPolicyProperties,
}
impl VaultAccessPolicyParameters {
    pub fn new(properties: VaultAccessPolicyProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location: None,
            properties,
        }
    }
}
#[doc = "Properties of the vault access policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyProperties {
    #[doc = "An array of 0 to 16 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
    #[serde(rename = "accessPolicies")]
    pub access_policies: Vec<AccessPolicyEntry>,
}
impl VaultAccessPolicyProperties {
    pub fn new(access_policies: Vec<AccessPolicyEntry>) -> Self {
        Self { access_policies }
    }
}
#[doc = "The parameters used to check the availability of the vault name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCheckNameAvailabilityParameters {
    #[doc = "The vault name."]
    pub name: String,
    #[doc = "The type of resource, Microsoft.KeyVault/vaults"]
    #[serde(rename = "type")]
    pub type_: vault_check_name_availability_parameters::Type,
}
impl VaultCheckNameAvailabilityParameters {
    pub fn new(name: String, type_: vault_check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod vault_check_name_availability_parameters {
    use super::*;
    #[doc = "The type of resource, Microsoft.KeyVault/vaults"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.KeyVault/vaults")]
        MicrosoftKeyVaultVaults,
    }
}
#[doc = "Parameters for creating or updating a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCreateOrUpdateParameters {
    #[doc = "The supported Azure location where the key vault should be created."]
    pub location: String,
    #[doc = "The tags that will be assigned to the key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the vault"]
    pub properties: VaultProperties,
}
impl VaultCreateOrUpdateParameters {
    pub fn new(location: String, properties: VaultProperties) -> Self {
        Self {
            location,
            tags: None,
            properties,
        }
    }
}
#[doc = "List of vaults"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultListResult {
    #[doc = "The list of vaults."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[doc = "The URL to get the next set of vaults."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VaultListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating or updating a vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultPatchParameters {
    #[doc = "The tags that will be assigned to the key vault. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the vault"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultPatchProperties>,
}
impl VaultPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultPatchProperties {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "SKU details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "An array of 0 to 16 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[doc = "Property to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault."]
    #[serde(rename = "enabledForDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[doc = "Property to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys."]
    #[serde(rename = "enabledForDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[doc = "Property to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault."]
    #[serde(rename = "enabledForTemplateDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[doc = "Property specifying whether recoverable deletion ('soft' delete) is enabled for this key vault. The property may not be set to false."]
    #[serde(rename = "enableSoftDelete", default, skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_patch_properties::CreateMode>,
    #[doc = "Property specifying whether protection against purge is enabled for this vault; it is only effective if soft delete is also enabled. Once activated, the property may no longer be reset to false."]
    #[serde(rename = "enablePurgeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
}
impl VaultPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vault_patch_properties {
    use super::*;
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
}
#[doc = "Properties of the vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultProperties {
    #[doc = "The Azure Active Directory tenant ID that should be used for authenticating requests to the key vault."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "SKU details"]
    pub sku: Sku,
    #[doc = "An array of 0 to 16 identities that have access to the key vault. All identities in the array must use the same tenant ID as the key vault's tenant ID. When `createMode` is set to `recover`, access policies are not required. Otherwise, access policies are required."]
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[doc = "The URI of the vault for performing operations on keys and secrets."]
    #[serde(rename = "vaultUri", default, skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[doc = "Property to specify whether Azure Virtual Machines are permitted to retrieve certificates stored as secrets from the key vault."]
    #[serde(rename = "enabledForDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[doc = "Property to specify whether Azure Disk Encryption is permitted to retrieve secrets from the vault and unwrap keys."]
    #[serde(rename = "enabledForDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[doc = "Property to specify whether Azure Resource Manager is permitted to retrieve secrets from the key vault."]
    #[serde(rename = "enabledForTemplateDeployment", default, skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[doc = "Property specifying whether recoverable deletion is enabled for this key vault. Setting this property to true activates the soft delete feature, whereby vaults or vault entities can be recovered after deletion. Enabling this functionality is irreversible - that is, the property does not accept false as its value."]
    #[serde(rename = "enableSoftDelete", default, skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_properties::CreateMode>,
    #[doc = "Property specifying whether protection against purge is enabled for this vault. Setting this property to true activates protection against purge for this vault and its content - only the Key Vault service may initiate a hard, irrecoverable deletion. The setting is effective only if soft delete is also enabled. Enabling this functionality is irreversible - that is, the property does not accept false as its value."]
    #[serde(rename = "enablePurgeProtection", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
}
impl VaultProperties {
    pub fn new(tenant_id: String, sku: Sku) -> Self {
        Self {
            tenant_id,
            sku,
            access_policies: Vec::new(),
            vault_uri: None,
            enabled_for_deployment: None,
            enabled_for_disk_encryption: None,
            enabled_for_template_deployment: None,
            enable_soft_delete: None,
            create_mode: None,
            enable_purge_protection: None,
        }
    }
}
pub mod vault_properties {
    use super::*;
    #[doc = "The vault's create mode to indicate whether the vault need to be recovered or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
}
