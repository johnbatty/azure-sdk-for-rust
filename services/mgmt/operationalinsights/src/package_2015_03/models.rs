#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Service Tier details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableServiceTier {
    #[doc = "The name of the Service Tier."]
    #[serde(rename = "ServiceTier", default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<available_service_tier::ServiceTier>,
    #[doc = "True if the Service Tier is enabled for the workspace."]
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The minimum retention for the Service Tier, in days."]
    #[serde(rename = "MinimumRetention", default, skip_serializing_if = "Option::is_none")]
    pub minimum_retention: Option<i64>,
    #[doc = "The maximum retention for the Service Tier, in days."]
    #[serde(rename = "MaximumRetention", default, skip_serializing_if = "Option::is_none")]
    pub maximum_retention: Option<i64>,
    #[doc = "The default retention for the Service Tier, in days."]
    #[serde(rename = "DefaultRetention", default, skip_serializing_if = "Option::is_none")]
    pub default_retention: Option<i64>,
    #[doc = "The capacity reservation level in GB per day. Returned for the Capacity Reservation Service Tier."]
    #[serde(rename = "CapacityReservationLevel", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_level: Option<i64>,
    #[doc = "Time when the sku was last updated for the workspace. Returned for the Capacity Reservation Service Tier."]
    #[serde(rename = "LastSkuUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_sku_update: Option<String>,
}
impl AvailableServiceTier {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod available_service_tier {
    use super::*;
    #[doc = "The name of the Service Tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceTier {
        Free,
        Standard,
        Premium,
        PerNode,
        #[serde(rename = "PerGB2018")]
        PerGb2018,
        Standalone,
        CapacityReservation,
    }
}
#[doc = "The core summary of a search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreSummary {
    #[doc = "The status of a core summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The number of documents of a core summary."]
    #[serde(rename = "numberOfDocuments")]
    pub number_of_documents: i64,
}
impl CoreSummary {
    pub fn new(number_of_documents: i64) -> Self {
        Self {
            status: None,
            number_of_documents,
        }
    }
}
#[doc = "Metadata for a workspace that isn't linked to an Azure subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkTarget {
    #[doc = "The GUID that uniquely identifies the workspace. "]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The display name of the workspace."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The DNS valid workspace name."]
    #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[doc = "The location of the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl LinkTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operation of OperationalInsights resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
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
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: OperationalInsights."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list OperationalInsights operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the OperationalInsights resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties of proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
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
#[doc = "Value object for saved search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSearch {
    #[doc = "The id of the saved search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the saved search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the saved search."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The ETag of the saved search."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Value object for saved search results."]
    pub properties: SavedSearchProperties,
}
impl SavedSearch {
    pub fn new(properties: SavedSearchProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            e_tag: None,
            properties,
        }
    }
}
#[doc = "Value object for saved search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedSearchProperties {
    #[doc = "The category of the saved search. This helps the user to find a saved search faster. "]
    pub category: String,
    #[doc = "Saved search display name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The query expression for the saved search. Please see https://docs.microsoft.com/en-us/azure/log-analytics/log-analytics-search-reference for reference."]
    pub query: String,
    #[doc = "The version number of the query language. The current version is 2 and is the default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "The tags attached to the saved search."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}
impl SavedSearchProperties {
    pub fn new(category: String, display_name: String, query: String) -> Self {
        Self {
            category,
            display_name,
            query,
            version: None,
            tags: Vec::new(),
        }
    }
}
#[doc = "The saved search list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavedSearchesListResult {
    #[doc = "Metadata for search results."]
    #[serde(rename = "__metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SearchMetadata>,
    #[doc = "The array of result values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SavedSearch>,
}
impl SavedSearchesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The get schema operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchGetSchemaResponse {
    #[doc = "Metadata for search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SearchMetadata>,
    #[doc = "The array of result values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SearchSchemaValue>,
}
impl SearchGetSchemaResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchMetadata {
    #[doc = "The request id of the search."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The search result type."]
    #[serde(rename = "resultType", default, skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
    #[doc = "The total number of search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[doc = "The number of top search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    #[doc = "The id of the search results request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The core summaries."]
    #[serde(rename = "coreSummaries", default, skip_serializing_if = "Vec::is_empty")]
    pub core_summaries: Vec<CoreSummary>,
    #[doc = "The status of the search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start time for the search."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The time of last update."]
    #[serde(rename = "lastUpdated", default, skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[doc = "The ETag of the search results."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "How the results are sorted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<SearchSort>,
    #[doc = "The request time."]
    #[serde(rename = "requestTime", default, skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,
    #[doc = "The aggregated value field."]
    #[serde(rename = "aggregatedValueField", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_value_field: Option<String>,
    #[doc = "The aggregated grouping fields."]
    #[serde(rename = "aggregatedGroupingFields", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_grouping_fields: Option<String>,
    #[doc = "The sum of all aggregates returned in the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum: Option<i64>,
    #[doc = "The max of all aggregates returned in the result set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[doc = "Schema metadata for search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<SearchMetadataSchema>,
}
impl SearchMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema metadata for search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchMetadataSchema {
    #[doc = "The name of the metadata schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version of the metadata schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
impl SearchMetadataSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Value object for schema results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSchemaValue {
    #[doc = "The name of the schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the schema."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The boolean that indicates the field is searchable as free text."]
    pub indexed: bool,
    #[doc = "The boolean that indicates whether or not the field is stored."]
    pub stored: bool,
    #[doc = "The boolean that indicates whether or not the field is a facet."]
    pub facet: bool,
    #[doc = "The array of workflows containing the field."]
    #[serde(rename = "ownerType", default, skip_serializing_if = "Vec::is_empty")]
    pub owner_type: Vec<String>,
}
impl SearchSchemaValue {
    pub fn new(indexed: bool, stored: bool, facet: bool) -> Self {
        Self {
            name: None,
            display_name: None,
            type_: None,
            indexed,
            stored,
            facet,
            owner_type: Vec::new(),
        }
    }
}
#[doc = "The sort parameters for search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchSort {
    #[doc = "The name of the field the search query is sorted on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The sort order of the search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<search_sort::Order>,
}
impl SearchSort {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod search_sort {
    use super::*;
    #[doc = "The sort order of the search."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Order {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
}
#[doc = "The shared keys for a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedKeys {
    #[doc = "The primary shared key of a workspace."]
    #[serde(rename = "primarySharedKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_shared_key: Option<String>,
    #[doc = "The secondary shared key of a workspace."]
    #[serde(rename = "secondarySharedKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_shared_key: Option<String>,
}
impl SharedKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a storage account connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[doc = "The Azure Resource Manager ID of the storage account resource."]
    pub id: String,
    #[doc = "The storage account key."]
    pub key: String,
}
impl StorageAccount {
    pub fn new(id: String, key: String) -> Self {
        Self { id, key }
    }
}
#[doc = "The top level storage insight resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageInsight {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Storage insight properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageInsightProperties>,
    #[doc = "The ETag of the storage insight."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl StorageInsight {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list storage insights operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageInsightListResult {
    #[doc = "A list of storage insight items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageInsight>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl StorageInsightListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage insight properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsightProperties {
    #[doc = "The names of the blob containers that the workspace should read"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub containers: Vec<String>,
    #[doc = "The names of the Azure tables that the workspace should read"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
    #[doc = "Describes a storage account connection."]
    #[serde(rename = "storageAccount")]
    pub storage_account: StorageAccount,
    #[doc = "The status of the storage insight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StorageInsightStatus>,
}
impl StorageInsightProperties {
    pub fn new(storage_account: StorageAccount) -> Self {
        Self {
            containers: Vec::new(),
            tables: Vec::new(),
            storage_account,
            status: None,
        }
    }
}
#[doc = "The status of the storage insight."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInsightStatus {
    #[doc = "The state of the storage insight connection to the workspace"]
    pub state: storage_insight_status::State,
    #[doc = "Description of the state of the storage insight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl StorageInsightStatus {
    pub fn new(state: storage_insight_status::State) -> Self {
        Self { state, description: None }
    }
}
pub mod storage_insight_status {
    use super::*;
    #[doc = "The state of the storage insight connection to the workspace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "OK")]
        Ok,
        #[serde(rename = "ERROR")]
        Error,
    }
}
#[doc = "A tag of a saved search."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[doc = "The tag name."]
    pub name: String,
    #[doc = "The tag value."]
    pub value: String,
}
impl Tag {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Describes the body of a purge request for an App Insights Workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeBody {
    #[doc = "Table from which to purge data."]
    pub table: String,
    #[doc = "The set of columns and filters (queries) to run over them to purge the resulting data."]
    pub filters: Vec<WorkspacePurgeBodyFilters>,
}
impl WorkspacePurgeBody {
    pub fn new(table: String, filters: Vec<WorkspacePurgeBodyFilters>) -> Self {
        Self { table, filters }
    }
}
#[doc = "User-defined filters to return data which will be purged from the table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspacePurgeBodyFilters {
    #[doc = "The column of the table over which the given query should run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[doc = "A query operator to evaluate over the provided column and value(s). Supported operators are ==, =~, in, in~, >, >=, <, <=, between, and have the same behavior as they would in a KQL query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[doc = "the value for the operator to function over. This can be a number (e.g., > 100), a string (timestamp >= '2017-09-01') or array of values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "When filtering over custom dimensions, this key will be used as the name of the custom dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl WorkspacePurgeBodyFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response containing operationId for a specific purge action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeResponse {
    #[doc = "Id to use when querying for status for a particular purge operation."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
impl WorkspacePurgeResponse {
    pub fn new(operation_id: String) -> Self {
        Self { operation_id }
    }
}
#[doc = "Response containing status for a specific purge operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePurgeStatusResponse {
    #[doc = "Status of the operation represented by the requested Id."]
    pub status: workspace_purge_status_response::Status,
}
impl WorkspacePurgeStatusResponse {
    pub fn new(status: workspace_purge_status_response::Status) -> Self {
        Self { status }
    }
}
pub mod workspace_purge_status_response {
    use super::*;
    #[doc = "Status of the operation represented by the requested Id."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "completed")]
        Completed,
    }
}
