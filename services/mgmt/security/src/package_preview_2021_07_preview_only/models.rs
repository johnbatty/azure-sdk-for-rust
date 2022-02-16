#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Describes an Azure resource with location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTrackedResourceLocation {
    #[doc = "Location where the resource is stored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AzureTrackedResourceLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ETag {
    #[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ETag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource with kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Kind {
    #[doc = "Kind of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Kind {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
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
#[doc = "The security connector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityConnector {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "A set of properties that defines the security connector configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityConnectorProperties>,
}
impl SecurityConnector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of properties that defines the security connector configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityConnectorProperties {
    #[doc = "The multi cloud resource identifier (account id in case of AWS connector)."]
    #[serde(rename = "hierarchyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_identifier: Option<String>,
    #[doc = "The multi cloud resource's cloud name."]
    #[serde(rename = "cloudName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_name: Option<security_connector_properties::CloudName>,
    #[doc = "A collection of offerings for the security connector."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub offerings: Vec<CloudOffering>,
    #[doc = "The multi cloud account's organizational data"]
    #[serde(rename = "organizationalData", default, skip_serializing_if = "Option::is_none")]
    pub organizational_data: Option<security_connector_properties::OrganizationalData>,
}
impl SecurityConnectorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_connector_properties {
    use super::*;
    #[doc = "The multi cloud resource's cloud name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CloudName {
        Azure,
        #[serde(rename = "AWS")]
        Aws,
        #[serde(rename = "GCP")]
        Gcp,
    }
    #[doc = "The multi cloud account's organizational data"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct OrganizationalData {
        #[doc = "The multi cloud account's membership type in the organization"]
        #[serde(rename = "organizationMembershipType", default, skip_serializing_if = "Option::is_none")]
        pub organization_membership_type: Option<organizational_data::OrganizationMembershipType>,
        #[doc = "If the multi cloud account is not of membership type organization, this will be the ID of the account's parent"]
        #[serde(rename = "parentHierarchyId", default, skip_serializing_if = "Option::is_none")]
        pub parent_hierarchy_id: Option<String>,
        #[doc = "If the multi cloud account is of membership type organization, this will be the name of the onboarding stackset"]
        #[serde(rename = "stacksetName", default, skip_serializing_if = "Option::is_none")]
        pub stackset_name: Option<String>,
        #[doc = "If the multi cloud account is of membership type organization, list of accounts excluded from offering"]
        #[serde(rename = "excludedAccountIds", default, skip_serializing_if = "Vec::is_empty")]
        pub excluded_account_ids: Vec<String>,
    }
    impl OrganizationalData {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod organizational_data {
        use super::*;
        #[doc = "The multi cloud account's membership type in the organization"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum OrganizationMembershipType {
            Member,
            Organization,
        }
    }
}
#[doc = "List of security connectors response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityConnectorsList {
    #[doc = "The list of security connectors under the given scope."]
    pub value: Vec<SecurityConnector>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SecurityConnectorsList {
    pub fn new(value: Vec<SecurityConnector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A list of key value pairs that describe the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "A list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure tracked resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub azure_tracked_resource_location: AzureTrackedResourceLocation,
    #[serde(flatten)]
    pub kind: Kind,
    #[serde(flatten)]
    pub e_tag: ETag,
    #[serde(flatten)]
    pub tags: Tags,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security offering details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudOffering {
    #[doc = "The type of the security offering."]
    #[serde(rename = "offeringType")]
    pub offering_type: cloud_offering::OfferingType,
    #[doc = "The offering description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CloudOffering {
    pub fn new(offering_type: cloud_offering::OfferingType) -> Self {
        Self {
            offering_type,
            description: None,
        }
    }
}
pub mod cloud_offering {
    use super::*;
    #[doc = "The type of the security offering."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OfferingType {
        CspmMonitorAws,
        DefenderForContainersAws,
        DefenderForServersAws,
    }
}
#[doc = "The CSPM monitoring for AWS offering configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "nativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub native_cloud_connection: Option<cspm_monitor_aws_offering::NativeCloudConnection>,
}
impl CspmMonitorAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            native_cloud_connection: None,
        }
    }
}
pub mod cspm_monitor_aws_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NativeCloudConnection {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl NativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for Containers AWS offering configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForContainersAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The kubernetes service connection configuration"]
    #[serde(rename = "kubernetesService", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_service: Option<defender_for_containers_aws_offering::KubernetesService>,
    #[doc = "The kubernetes to scuba connection configuration"]
    #[serde(rename = "kubernetesScubaReader", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_scuba_reader: Option<defender_for_containers_aws_offering::KubernetesScubaReader>,
    #[doc = "The cloudwatch to kinesis connection configuration"]
    #[serde(rename = "cloudWatchToKinesis", default, skip_serializing_if = "Option::is_none")]
    pub cloud_watch_to_kinesis: Option<defender_for_containers_aws_offering::CloudWatchToKinesis>,
    #[doc = "The kinesis to s3 connection configuration"]
    #[serde(rename = "kinesisToS3", default, skip_serializing_if = "Option::is_none")]
    pub kinesis_to_s3: Option<defender_for_containers_aws_offering::KinesisToS3>,
}
impl DefenderForContainersAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            kubernetes_service: None,
            kubernetes_scuba_reader: None,
            cloud_watch_to_kinesis: None,
            kinesis_to_s3: None,
        }
    }
}
pub mod defender_for_containers_aws_offering {
    use super::*;
    #[doc = "The kubernetes service connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KubernetesService {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KubernetesService {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The kubernetes to scuba connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KubernetesScubaReader {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KubernetesScubaReader {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The cloudwatch to kinesis connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CloudWatchToKinesis {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl CloudWatchToKinesis {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The kinesis to s3 connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KinesisToS3 {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KinesisToS3 {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for Servers AWS offering configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForServersAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The Defender for servers connection configuration"]
    #[serde(rename = "defenderForServers", default, skip_serializing_if = "Option::is_none")]
    pub defender_for_servers: Option<defender_for_servers_aws_offering::DefenderForServers>,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_for_servers_aws_offering::ArcAutoProvisioning>,
}
impl DefenderForServersAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            defender_for_servers: None,
            arc_auto_provisioning: None,
        }
    }
}
pub mod defender_for_servers_aws_offering {
    use super::*;
    #[doc = "The Defender for servers connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefenderForServers {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl DefenderForServers {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "Metadata of Service Principal secret for autoprovisioning"]
        #[serde(rename = "servicePrincipalSecretMetadata", default, skip_serializing_if = "Option::is_none")]
        pub service_principal_secret_metadata: Option<arc_auto_provisioning::ServicePrincipalSecretMetadata>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Metadata of Service Principal secret for autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ServicePrincipalSecretMetadata {
            #[doc = "expiration date of service principal secret"]
            #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
            pub expiry_date: Option<String>,
            #[doc = "region of parameter store where secret is kept"]
            #[serde(rename = "parameterStoreRegion", default, skip_serializing_if = "Option::is_none")]
            pub parameter_store_region: Option<String>,
            #[doc = "name of secret resource in parameter store"]
            #[serde(rename = "parameterNameInStore", default, skip_serializing_if = "Option::is_none")]
            pub parameter_name_in_store: Option<String>,
        }
        impl ServicePrincipalSecretMetadata {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
