#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Localized display information of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryDisplay {
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "ResourceType for which this Operation can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operations Name itself."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation having details of what operation is about."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ClientDiscoveryDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox log specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForLogSpecification {
    #[doc = "Name for shoebox log specification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "blob duration of shoebox log specification"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl ClientDiscoveryForLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox properties in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForProperties {
    #[doc = "Class to represent shoebox service specification in json client discovery."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ClientDiscoveryForServiceSpecification>,
}
impl ClientDiscoveryForProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class to represent shoebox service specification in json client discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryForServiceSpecification {
    #[doc = "List of log specifications of this operation."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<ClientDiscoveryForLogSpecification>,
}
impl ClientDiscoveryForServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operations List response which contains list of available APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryResponse {
    #[doc = "List of available operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClientDiscoveryValueForSingleApi>,
    #[doc = "Link to the next chunk of Response."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ClientDiscoveryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientDiscoveryValueForSingleApi {
    #[doc = "Name of the Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display information of an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ClientDiscoveryDisplay>,
    #[doc = "The intended executor of the operation;governs the display of the operation in the RBAC UX and the audit logs UX"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Class to represent shoebox properties in json client discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClientDiscoveryForProperties>,
}
impl ClientDiscoveryValueForSingleApi {
    pub fn new() -> Self {
        Self::default()
    }
}
