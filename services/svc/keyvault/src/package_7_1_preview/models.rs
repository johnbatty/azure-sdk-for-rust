#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The action that will be executed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Action {
    #[doc = "The type of the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<action::ActionType>,
}
impl Action {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod action {
    use super::*;
    #[doc = "The type of the action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        EmailContacts,
        AutoRenew,
    }
}
#[doc = "Details of the organization administrator of the certificate issuer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdministratorDetails {
    #[doc = "First name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl AdministratorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object attributes managed by the KeyVault service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Attributes {
    #[doc = "Determines whether the object is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Not before date in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    #[doc = "Expiry date in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[doc = "Creation time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}
impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The backup certificate result, containing the backup blob."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupCertificateResult {
    #[doc = "The backup blob containing the backed up certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl BackupCertificateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The backup key result, containing the backup blob."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupKeyResult {
    #[doc = "The backup blob containing the backed up key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl BackupKeyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The backup secret result, containing the backup blob."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupSecretResult {
    #[doc = "The backup blob containing the backed up secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl BackupSecretResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The backup storage result, containing the backup blob."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupStorageResult {
    #[doc = "The backup blob containing the backed up storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl BackupStorageResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate management attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateAttributes {
    #[serde(flatten)]
    pub attributes: Attributes,
    #[doc = "softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0."]
    #[serde(rename = "recoverableDays", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_days: Option<i32>,
    #[doc = "Reflects the deletion recovery level currently in effect for certificates in the current vault. If it contains 'Purgeable', the certificate can be permanently deleted by a privileged user; otherwise, only the system can purge the certificate, at the end of the retention interval."]
    #[serde(rename = "recoveryLevel", default, skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<certificate_attributes::RecoveryLevel>,
}
impl CertificateAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_attributes {
    use super::*;
    #[doc = "Reflects the deletion recovery level currently in effect for certificates in the current vault. If it contains 'Purgeable', the certificate can be permanently deleted by a privileged user; otherwise, only the system can purge the certificate, at the end of the retention interval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryLevel")]
    pub enum RecoveryLevel {
        Purgeable,
        #[serde(rename = "Recoverable+Purgeable")]
        RecoverablePurgeable,
        Recoverable,
        #[serde(rename = "Recoverable+ProtectedSubscription")]
        RecoverableProtectedSubscription,
        #[serde(rename = "CustomizedRecoverable+Purgeable")]
        CustomizedRecoverablePurgeable,
        CustomizedRecoverable,
        #[serde(rename = "CustomizedRecoverable+ProtectedSubscription")]
        CustomizedRecoverableProtectedSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purgeable => serializer.serialize_unit_variant("RecoveryLevel", 0u32, "Purgeable"),
                Self::RecoverablePurgeable => serializer.serialize_unit_variant("RecoveryLevel", 1u32, "Recoverable+Purgeable"),
                Self::Recoverable => serializer.serialize_unit_variant("RecoveryLevel", 2u32, "Recoverable"),
                Self::RecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 3u32, "Recoverable+ProtectedSubscription")
                }
                Self::CustomizedRecoverablePurgeable => {
                    serializer.serialize_unit_variant("RecoveryLevel", 4u32, "CustomizedRecoverable+Purgeable")
                }
                Self::CustomizedRecoverable => serializer.serialize_unit_variant("RecoveryLevel", 5u32, "CustomizedRecoverable"),
                Self::CustomizedRecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 6u32, "CustomizedRecoverable+ProtectedSubscription")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A certificate bundle consists of a certificate (X509) plus its attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateBundle {
    #[doc = "The certificate id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The key id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[doc = "The secret id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "Thumbprint of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
    #[doc = "Management policy for a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<CertificatePolicy>,
    #[doc = "CER contents of x509 certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cer: Option<String>,
    #[doc = "The content type of the secret."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The certificate management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CertificateAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CertificateBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateCreateParameters {
    #[doc = "Management policy for a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<CertificatePolicy>,
    #[doc = "The certificate management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CertificateAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CertificateCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate import parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateImportParameters {
    #[doc = "Base64 encoded representation of the certificate object to import. This certificate needs to contain the private key."]
    pub value: String,
    #[doc = "If the private key in base64EncodedCertificate is encrypted, the password used for encryption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,
    #[doc = "Management policy for a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<CertificatePolicy>,
    #[doc = "The certificate management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CertificateAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CertificateImportParameters {
    pub fn new(value: String) -> Self {
        Self {
            value,
            pwd: None,
            policy: None,
            attributes: None,
            tags: None,
        }
    }
}
#[doc = "The certificate issuer item containing certificate issuer metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateIssuerItem {
    #[doc = "Certificate Identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The issuer provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl CertificateIssuerItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate issuer list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateIssuerListResult {
    #[doc = "A response message containing a list of certificate issuers in the key vault along with a link to the next page of certificate issuers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CertificateIssuerItem>,
    #[doc = "The URL to get the next set of certificate issuers."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateIssuerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CertificateIssuerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate issuer set parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateIssuerSetParameters {
    #[doc = "The issuer provider."]
    pub provider: String,
    #[doc = "The credentials to be used for the certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<IssuerCredentials>,
    #[doc = "Details of the organization of the certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_details: Option<OrganizationDetails>,
    #[doc = "The attributes of an issuer managed by the Key Vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IssuerAttributes>,
}
impl CertificateIssuerSetParameters {
    pub fn new(provider: String) -> Self {
        Self {
            provider,
            credentials: None,
            org_details: None,
            attributes: None,
        }
    }
}
#[doc = "The certificate issuer update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateIssuerUpdateParameters {
    #[doc = "The issuer provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The credentials to be used for the certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<IssuerCredentials>,
    #[doc = "Details of the organization of the certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_details: Option<OrganizationDetails>,
    #[doc = "The attributes of an issuer managed by the Key Vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IssuerAttributes>,
}
impl CertificateIssuerUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate item containing certificate metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateItem {
    #[doc = "Certificate identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The certificate management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CertificateAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Thumbprint of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
}
impl CertificateItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateListResult {
    #[doc = "A response message containing a list of certificates in the key vault along with a link to the next page of certificates."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CertificateItem>,
    #[doc = "The URL to get the next set of certificates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CertificateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate merge parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateMergeParameters {
    #[doc = "The certificate or the certificate chain to merge."]
    pub x5c: Vec<String>,
    #[doc = "The certificate management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CertificateAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CertificateMergeParameters {
    pub fn new(x5c: Vec<String>) -> Self {
        Self {
            x5c,
            attributes: None,
            tags: None,
        }
    }
}
#[doc = "A certificate operation is returned in case of asynchronous requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateOperation {
    #[doc = "The certificate id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Parameters for the issuer of the X509 component of a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<IssuerParameters>,
    #[doc = "The certificate signing request (CSR) that is being used in the certificate operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
    #[doc = "Indicates if cancellation was requested on the certificate operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_requested: Option<bool>,
    #[doc = "Status of the certificate operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The status details of the certificate operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "The key vault server error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "Location which contains the result of the certificate operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Identifier for the certificate operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl CertificateOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate operation update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateOperationUpdateParameter {
    #[doc = "Indicates if cancellation was requested on the certificate operation."]
    pub cancellation_requested: bool,
}
impl CertificateOperationUpdateParameter {
    pub fn new(cancellation_requested: bool) -> Self {
        Self { cancellation_requested }
    }
}
#[doc = "Management policy for a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificatePolicy {
    #[doc = "The certificate id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Properties of the key pair backing a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_props: Option<KeyProperties>,
    #[doc = "Properties of the key backing a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_props: Option<SecretProperties>,
    #[doc = "Properties of the X509 component of a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509_props: Option<X509CertificateProperties>,
    #[doc = "Actions that will be performed by Key Vault over the lifetime of a certificate."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub lifetime_actions: Vec<LifetimeAction>,
    #[doc = "Parameters for the issuer of the X509 component of a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<IssuerParameters>,
    #[doc = "The certificate management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CertificateAttributes>,
}
impl CertificatePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The certificate restore parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateRestoreParameters {
    #[doc = "The backup blob associated with a certificate bundle."]
    pub value: String,
}
impl CertificateRestoreParameters {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "The certificate update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateUpdateParameters {
    #[doc = "Management policy for a certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<CertificatePolicy>,
    #[doc = "The certificate management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<CertificateAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CertificateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The contact information for the vault certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Contact {
    #[doc = "Email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl Contact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The contacts for the vault certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Contacts {
    #[doc = "Identifier for the contacts collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The contact list for the vault certificates."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contacts: Vec<Contact>,
}
impl Contacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Deleted Certificate consisting of its previous id, attributes and its tags, as well as information on when it will be purged."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedCertificateBundle {
    #[serde(flatten)]
    pub certificate_bundle: CertificateBundle,
    #[doc = "The url of the recovery object, used to identify and recover the deleted certificate."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the certificate is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the certificate was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedCertificateBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted certificate item containing metadata about the deleted certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedCertificateItem {
    #[serde(flatten)]
    pub certificate_item: CertificateItem,
    #[doc = "The url of the recovery object, used to identify and recover the deleted certificate."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the certificate is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the certificate was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedCertificateItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of certificates that have been deleted in this vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedCertificateListResult {
    #[doc = "A response message containing a list of deleted certificates in the vault along with a link to the next page of deleted certificates"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeletedCertificateItem>,
    #[doc = "The URL to get the next set of deleted certificates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedCertificateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedCertificateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A DeletedKeyBundle consisting of a WebKey plus its Attributes and deletion info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedKeyBundle {
    #[serde(flatten)]
    pub key_bundle: KeyBundle,
    #[doc = "The url of the recovery object, used to identify and recover the deleted key."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the key is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the key was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedKeyBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted key item containing the deleted key metadata and information about deletion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedKeyItem {
    #[serde(flatten)]
    pub key_item: KeyItem,
    #[doc = "The url of the recovery object, used to identify and recover the deleted key."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the key is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the key was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedKeyItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of keys that have been deleted in this vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedKeyListResult {
    #[doc = "A response message containing a list of deleted keys in the vault along with a link to the next page of deleted keys"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeletedKeyItem>,
    #[doc = "The URL to get the next set of deleted keys."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedKeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedKeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A deleted SAS definition bundle consisting of its previous id, attributes and its tags, as well as information on when it will be purged."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedSasDefinitionBundle {
    #[serde(flatten)]
    pub sas_definition_bundle: SasDefinitionBundle,
    #[doc = "The url of the recovery object, used to identify and recover the deleted SAS definition."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the SAS definition is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the SAS definition was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedSasDefinitionBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted SAS definition item containing metadata about the deleted SAS definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedSasDefinitionItem {
    #[serde(flatten)]
    pub sas_definition_item: SasDefinitionItem,
    #[doc = "The url of the recovery object, used to identify and recover the deleted SAS definition."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the SAS definition is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the SAS definition was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedSasDefinitionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted SAS definition list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedSasDefinitionListResult {
    #[doc = "A response message containing a list of the deleted SAS definitions in the vault along with a link to the next page of deleted sas definitions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeletedSasDefinitionItem>,
    #[doc = "The URL to get the next set of deleted SAS definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedSasDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedSasDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Deleted Secret consisting of its previous id, attributes and its tags, as well as information on when it will be purged."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedSecretBundle {
    #[serde(flatten)]
    pub secret_bundle: SecretBundle,
    #[doc = "The url of the recovery object, used to identify and recover the deleted secret."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the secret is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the secret was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedSecretBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted secret item containing metadata about the deleted secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedSecretItem {
    #[serde(flatten)]
    pub secret_item: SecretItem,
    #[doc = "The url of the recovery object, used to identify and recover the deleted secret."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the secret is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the secret was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedSecretItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted secret list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedSecretListResult {
    #[doc = "A response message containing a list of the deleted secrets in the vault along with a link to the next page of deleted secrets"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeletedSecretItem>,
    #[doc = "The URL to get the next set of deleted secrets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedSecretListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedSecretListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted storage account item containing metadata about the deleted storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedStorageAccountItem {
    #[serde(flatten)]
    pub storage_account_item: StorageAccountItem,
    #[doc = "The url of the recovery object, used to identify and recover the deleted storage account."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the storage account is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the storage account was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedStorageAccountItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A deleted storage account bundle consisting of its previous id, attributes and its tags, as well as information on when it will be purged."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedStorageBundle {
    #[serde(flatten)]
    pub storage_bundle: StorageBundle,
    #[doc = "The url of the recovery object, used to identify and recover the deleted storage account."]
    #[serde(rename = "recoveryId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    #[doc = "The time when the storage account is scheduled to be purged, in UTC"]
    #[serde(rename = "scheduledPurgeDate", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i64>,
    #[doc = "The time when the storage account was deleted, in UTC"]
    #[serde(rename = "deletedDate", default, skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i64>,
}
impl DeletedStorageBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deleted storage account list result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedStorageListResult {
    #[doc = "A response message containing a list of the deleted storage accounts in the vault along with a link to the next page of deleted storage accounts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeletedStorageAccountItem>,
    #[doc = "The URL to get the next set of deleted storage accounts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedStorageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeletedStorageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key vault server error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The key vault server error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<Error>>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The attributes of an issuer managed by the Key Vault service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssuerAttributes {
    #[doc = "Determines whether the issuer is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Creation time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}
impl IssuerAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The issuer for Key Vault certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssuerBundle {
    #[doc = "Identifier for the issuer object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The issuer provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The credentials to be used for the certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<IssuerCredentials>,
    #[doc = "Details of the organization of the certificate issuer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_details: Option<OrganizationDetails>,
    #[doc = "The attributes of an issuer managed by the Key Vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<IssuerAttributes>,
}
impl IssuerBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The credentials to be used for the certificate issuer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssuerCredentials {
    #[doc = "The user name/account name/account id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The password/secret/account key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,
}
impl IssuerCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for the issuer of the X509 component of a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssuerParameters {
    #[doc = "Name of the referenced issuer object or reserved names; for example, 'Self' or 'Unknown'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Certificate type as supported by the provider (optional); for example 'OV-SSL', 'EV-SSL'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cty: Option<String>,
    #[doc = "Indicates if the certificates generated under this policy should be published to certificate transparency logs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_transparency: Option<bool>,
}
impl IssuerParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "As of http://tools.ietf.org/html/draft-ietf-jose-json-web-key-18"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonWebKey {
    #[doc = "Key identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[doc = "JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kty: Option<json_web_key::Kty>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub key_ops: Vec<String>,
    #[doc = "RSA modulus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[doc = "RSA public exponent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[doc = "RSA private exponent, or the D component of an EC private key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    #[doc = "RSA private key parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dp: Option<String>,
    #[doc = "RSA private key parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dq: Option<String>,
    #[doc = "RSA private key parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qi: Option<String>,
    #[doc = "RSA secret prime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[doc = "RSA secret prime, with p < q."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[doc = "Symmetric key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<String>,
    #[doc = "HSM Token, used with 'Bring Your Own Key'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_hsm: Option<String>,
    #[doc = "Elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crv: Option<json_web_key::Crv>,
    #[doc = "X component of an EC public key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[doc = "Y component of an EC public key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}
impl JsonWebKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod json_web_key {
    use super::*;
    #[doc = "JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kty")]
    pub enum Kty {
        #[serde(rename = "EC")]
        Ec,
        #[serde(rename = "EC-HSM")]
        EcHsm,
        #[serde(rename = "RSA")]
        Rsa,
        #[serde(rename = "RSA-HSM")]
        RsaHsm,
        #[serde(rename = "oct")]
        Oct,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kty {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kty {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kty {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ec => serializer.serialize_unit_variant("Kty", 0u32, "EC"),
                Self::EcHsm => serializer.serialize_unit_variant("Kty", 1u32, "EC-HSM"),
                Self::Rsa => serializer.serialize_unit_variant("Kty", 2u32, "RSA"),
                Self::RsaHsm => serializer.serialize_unit_variant("Kty", 3u32, "RSA-HSM"),
                Self::Oct => serializer.serialize_unit_variant("Kty", 4u32, "oct"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Crv")]
    pub enum Crv {
        #[serde(rename = "P-256")]
        P256,
        #[serde(rename = "P-384")]
        P384,
        #[serde(rename = "P-521")]
        P521,
        #[serde(rename = "P-256K")]
        P256k,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Crv {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Crv {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Crv {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::P256 => serializer.serialize_unit_variant("Crv", 0u32, "P-256"),
                Self::P384 => serializer.serialize_unit_variant("Crv", 1u32, "P-384"),
                Self::P521 => serializer.serialize_unit_variant("Crv", 2u32, "P-521"),
                Self::P256k => serializer.serialize_unit_variant("Crv", 3u32, "P-256K"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The attributes of a key managed by the key vault service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyAttributes {
    #[serde(flatten)]
    pub attributes: Attributes,
    #[doc = "softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0."]
    #[serde(rename = "recoverableDays", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_days: Option<i32>,
    #[doc = "Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval."]
    #[serde(rename = "recoveryLevel", default, skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<key_attributes::RecoveryLevel>,
}
impl KeyAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_attributes {
    use super::*;
    #[doc = "Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryLevel")]
    pub enum RecoveryLevel {
        Purgeable,
        #[serde(rename = "Recoverable+Purgeable")]
        RecoverablePurgeable,
        Recoverable,
        #[serde(rename = "Recoverable+ProtectedSubscription")]
        RecoverableProtectedSubscription,
        #[serde(rename = "CustomizedRecoverable+Purgeable")]
        CustomizedRecoverablePurgeable,
        CustomizedRecoverable,
        #[serde(rename = "CustomizedRecoverable+ProtectedSubscription")]
        CustomizedRecoverableProtectedSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purgeable => serializer.serialize_unit_variant("RecoveryLevel", 0u32, "Purgeable"),
                Self::RecoverablePurgeable => serializer.serialize_unit_variant("RecoveryLevel", 1u32, "Recoverable+Purgeable"),
                Self::Recoverable => serializer.serialize_unit_variant("RecoveryLevel", 2u32, "Recoverable"),
                Self::RecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 3u32, "Recoverable+ProtectedSubscription")
                }
                Self::CustomizedRecoverablePurgeable => {
                    serializer.serialize_unit_variant("RecoveryLevel", 4u32, "CustomizedRecoverable+Purgeable")
                }
                Self::CustomizedRecoverable => serializer.serialize_unit_variant("RecoveryLevel", 5u32, "CustomizedRecoverable"),
                Self::CustomizedRecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 6u32, "CustomizedRecoverable+ProtectedSubscription")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A KeyBundle consisting of a WebKey plus its attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyBundle {
    #[doc = "As of http://tools.ietf.org/html/draft-ietf-jose-json-web-key-18"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<JsonWebKey>,
    #[doc = "The attributes of a key managed by the key vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "True if the key's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
}
impl KeyBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyCreateParameters {
    #[doc = "The type of key to create. For valid values, see JsonWebKeyType."]
    pub kty: key_create_parameters::Kty,
    #[doc = "The key size in bits. For example: 2048, 3072, or 4096 for RSA."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub key_ops: Vec<String>,
    #[doc = "The attributes of a key managed by the key vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crv: Option<key_create_parameters::Crv>,
}
impl KeyCreateParameters {
    pub fn new(kty: key_create_parameters::Kty) -> Self {
        Self {
            kty,
            key_size: None,
            key_ops: Vec::new(),
            attributes: None,
            tags: None,
            crv: None,
        }
    }
}
pub mod key_create_parameters {
    use super::*;
    #[doc = "The type of key to create. For valid values, see JsonWebKeyType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kty")]
    pub enum Kty {
        #[serde(rename = "EC")]
        Ec,
        #[serde(rename = "EC-HSM")]
        EcHsm,
        #[serde(rename = "RSA")]
        Rsa,
        #[serde(rename = "RSA-HSM")]
        RsaHsm,
        #[serde(rename = "oct")]
        Oct,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kty {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kty {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kty {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ec => serializer.serialize_unit_variant("Kty", 0u32, "EC"),
                Self::EcHsm => serializer.serialize_unit_variant("Kty", 1u32, "EC-HSM"),
                Self::Rsa => serializer.serialize_unit_variant("Kty", 2u32, "RSA"),
                Self::RsaHsm => serializer.serialize_unit_variant("Kty", 3u32, "RSA-HSM"),
                Self::Oct => serializer.serialize_unit_variant("Kty", 4u32, "oct"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Crv")]
    pub enum Crv {
        #[serde(rename = "P-256")]
        P256,
        #[serde(rename = "P-384")]
        P384,
        #[serde(rename = "P-521")]
        P521,
        #[serde(rename = "P-256K")]
        P256k,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Crv {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Crv {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Crv {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::P256 => serializer.serialize_unit_variant("Crv", 0u32, "P-256"),
                Self::P384 => serializer.serialize_unit_variant("Crv", 1u32, "P-384"),
                Self::P521 => serializer.serialize_unit_variant("Crv", 2u32, "P-521"),
                Self::P256k => serializer.serialize_unit_variant("Crv", 3u32, "P-256K"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The key import parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyImportParameters {
    #[doc = "Whether to import as a hardware key (HSM) or software key."]
    #[serde(rename = "Hsm", default, skip_serializing_if = "Option::is_none")]
    pub hsm: Option<bool>,
    #[doc = "As of http://tools.ietf.org/html/draft-ietf-jose-json-web-key-18"]
    pub key: JsonWebKey,
    #[doc = "The attributes of a key managed by the key vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl KeyImportParameters {
    pub fn new(key: JsonWebKey) -> Self {
        Self {
            hsm: None,
            key,
            attributes: None,
            tags: None,
        }
    }
}
#[doc = "The key item containing key metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyItem {
    #[doc = "Key identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[doc = "The attributes of a key managed by the key vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "True if the key's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
}
impl KeyItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyListResult {
    #[doc = "A response message containing a list of keys in the key vault along with a link to the next page of keys."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<KeyItem>,
    #[doc = "The URL to get the next set of keys."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KeyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl KeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyOperationResult {
    #[doc = "Key identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl KeyOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key operations parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyOperationsParameters {
    #[doc = "algorithm identifier"]
    pub alg: key_operations_parameters::Alg,
    pub value: String,
}
impl KeyOperationsParameters {
    pub fn new(alg: key_operations_parameters::Alg, value: String) -> Self {
        Self { alg, value }
    }
}
pub mod key_operations_parameters {
    use super::*;
    #[doc = "algorithm identifier"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Alg")]
    pub enum Alg {
        #[serde(rename = "RSA-OAEP")]
        RsaOaep,
        #[serde(rename = "RSA-OAEP-256")]
        RsaOaep256,
        #[serde(rename = "RSA1_5")]
        Rsa15,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Alg {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Alg {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Alg {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RsaOaep => serializer.serialize_unit_variant("Alg", 0u32, "RSA-OAEP"),
                Self::RsaOaep256 => serializer.serialize_unit_variant("Alg", 1u32, "RSA-OAEP-256"),
                Self::Rsa15 => serializer.serialize_unit_variant("Alg", 2u32, "RSA1_5"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of the key pair backing a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyProperties {
    #[doc = "Not supported in this version. Indicates if the private key can be exported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exportable: Option<bool>,
    #[doc = "The type of key pair to be used for the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kty: Option<key_properties::Kty>,
    #[doc = "The key size in bits. For example: 2048, 3072, or 4096 for RSA."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,
    #[doc = "Indicates if the same key pair will be used on certificate renewal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reuse_key: Option<bool>,
    #[doc = "Elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crv: Option<key_properties::Crv>,
}
impl KeyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_properties {
    use super::*;
    #[doc = "The type of key pair to be used for the certificate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kty")]
    pub enum Kty {
        #[serde(rename = "EC")]
        Ec,
        #[serde(rename = "EC-HSM")]
        EcHsm,
        #[serde(rename = "RSA")]
        Rsa,
        #[serde(rename = "RSA-HSM")]
        RsaHsm,
        #[serde(rename = "oct")]
        Oct,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kty {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kty {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kty {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ec => serializer.serialize_unit_variant("Kty", 0u32, "EC"),
                Self::EcHsm => serializer.serialize_unit_variant("Kty", 1u32, "EC-HSM"),
                Self::Rsa => serializer.serialize_unit_variant("Kty", 2u32, "RSA"),
                Self::RsaHsm => serializer.serialize_unit_variant("Kty", 3u32, "RSA-HSM"),
                Self::Oct => serializer.serialize_unit_variant("Kty", 4u32, "oct"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Elliptic curve name. For valid values, see JsonWebKeyCurveName."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Crv")]
    pub enum Crv {
        #[serde(rename = "P-256")]
        P256,
        #[serde(rename = "P-384")]
        P384,
        #[serde(rename = "P-521")]
        P521,
        #[serde(rename = "P-256K")]
        P256k,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Crv {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Crv {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Crv {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::P256 => serializer.serialize_unit_variant("Crv", 0u32, "P-256"),
                Self::P384 => serializer.serialize_unit_variant("Crv", 1u32, "P-384"),
                Self::P521 => serializer.serialize_unit_variant("Crv", 2u32, "P-521"),
                Self::P256k => serializer.serialize_unit_variant("Crv", 3u32, "P-256K"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The key restore parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyRestoreParameters {
    #[doc = "The backup blob associated with a key bundle."]
    pub value: String,
}
impl KeyRestoreParameters {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "The key operations parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeySignParameters {
    #[doc = "The signing/verification algorithm identifier. For more information on possible algorithm types, see JsonWebKeySignatureAlgorithm."]
    pub alg: key_sign_parameters::Alg,
    pub value: String,
}
impl KeySignParameters {
    pub fn new(alg: key_sign_parameters::Alg, value: String) -> Self {
        Self { alg, value }
    }
}
pub mod key_sign_parameters {
    use super::*;
    #[doc = "The signing/verification algorithm identifier. For more information on possible algorithm types, see JsonWebKeySignatureAlgorithm."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Alg")]
    pub enum Alg {
        #[serde(rename = "PS256")]
        Ps256,
        #[serde(rename = "PS384")]
        Ps384,
        #[serde(rename = "PS512")]
        Ps512,
        #[serde(rename = "RS256")]
        Rs256,
        #[serde(rename = "RS384")]
        Rs384,
        #[serde(rename = "RS512")]
        Rs512,
        #[serde(rename = "RSNULL")]
        Rsnull,
        #[serde(rename = "ES256")]
        Es256,
        #[serde(rename = "ES384")]
        Es384,
        #[serde(rename = "ES512")]
        Es512,
        #[serde(rename = "ES256K")]
        Es256k,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Alg {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Alg {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Alg {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ps256 => serializer.serialize_unit_variant("Alg", 0u32, "PS256"),
                Self::Ps384 => serializer.serialize_unit_variant("Alg", 1u32, "PS384"),
                Self::Ps512 => serializer.serialize_unit_variant("Alg", 2u32, "PS512"),
                Self::Rs256 => serializer.serialize_unit_variant("Alg", 3u32, "RS256"),
                Self::Rs384 => serializer.serialize_unit_variant("Alg", 4u32, "RS384"),
                Self::Rs512 => serializer.serialize_unit_variant("Alg", 5u32, "RS512"),
                Self::Rsnull => serializer.serialize_unit_variant("Alg", 6u32, "RSNULL"),
                Self::Es256 => serializer.serialize_unit_variant("Alg", 7u32, "ES256"),
                Self::Es384 => serializer.serialize_unit_variant("Alg", 8u32, "ES384"),
                Self::Es512 => serializer.serialize_unit_variant("Alg", 9u32, "ES512"),
                Self::Es256k => serializer.serialize_unit_variant("Alg", 10u32, "ES256K"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The key update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyUpdateParameters {
    #[doc = "Json web key operations. For more information on possible key operations, see JsonWebKeyOperation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub key_ops: Vec<String>,
    #[doc = "The attributes of a key managed by the key vault service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl KeyUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key vault error exception."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultError {
    #[doc = "The key vault server error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl azure_core::Continuable for KeyVaultError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl KeyVaultError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key verify parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVerifyParameters {
    #[doc = "The signing/verification algorithm. For more information on possible algorithm types, see JsonWebKeySignatureAlgorithm."]
    pub alg: key_verify_parameters::Alg,
    #[doc = "The digest used for signing."]
    pub digest: String,
    #[doc = "The signature to be verified."]
    pub value: String,
}
impl KeyVerifyParameters {
    pub fn new(alg: key_verify_parameters::Alg, digest: String, value: String) -> Self {
        Self { alg, digest, value }
    }
}
pub mod key_verify_parameters {
    use super::*;
    #[doc = "The signing/verification algorithm. For more information on possible algorithm types, see JsonWebKeySignatureAlgorithm."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Alg")]
    pub enum Alg {
        #[serde(rename = "PS256")]
        Ps256,
        #[serde(rename = "PS384")]
        Ps384,
        #[serde(rename = "PS512")]
        Ps512,
        #[serde(rename = "RS256")]
        Rs256,
        #[serde(rename = "RS384")]
        Rs384,
        #[serde(rename = "RS512")]
        Rs512,
        #[serde(rename = "RSNULL")]
        Rsnull,
        #[serde(rename = "ES256")]
        Es256,
        #[serde(rename = "ES384")]
        Es384,
        #[serde(rename = "ES512")]
        Es512,
        #[serde(rename = "ES256K")]
        Es256k,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Alg {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Alg {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Alg {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ps256 => serializer.serialize_unit_variant("Alg", 0u32, "PS256"),
                Self::Ps384 => serializer.serialize_unit_variant("Alg", 1u32, "PS384"),
                Self::Ps512 => serializer.serialize_unit_variant("Alg", 2u32, "PS512"),
                Self::Rs256 => serializer.serialize_unit_variant("Alg", 3u32, "RS256"),
                Self::Rs384 => serializer.serialize_unit_variant("Alg", 4u32, "RS384"),
                Self::Rs512 => serializer.serialize_unit_variant("Alg", 5u32, "RS512"),
                Self::Rsnull => serializer.serialize_unit_variant("Alg", 6u32, "RSNULL"),
                Self::Es256 => serializer.serialize_unit_variant("Alg", 7u32, "ES256"),
                Self::Es384 => serializer.serialize_unit_variant("Alg", 8u32, "ES384"),
                Self::Es512 => serializer.serialize_unit_variant("Alg", 9u32, "ES512"),
                Self::Es256k => serializer.serialize_unit_variant("Alg", 10u32, "ES256K"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The key verify result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVerifyResult {
    #[doc = "True if the signature is verified, otherwise false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
impl KeyVerifyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Action and its trigger that will be performed by Key Vault over the lifetime of a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LifetimeAction {
    #[doc = "A condition to be satisfied for an action to be executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[doc = "The action that will be executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}
impl LifetimeAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the organization of the certificate issuer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationDetails {
    #[doc = "Id of the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Details of the organization administrator."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub admin_details: Vec<AdministratorDetails>,
}
impl OrganizationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The pending certificate signing request result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PendingCertificateSigningRequestResult {
    #[doc = "The pending certificate signing request as Base64 encoded string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl PendingCertificateSigningRequestResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAS definition management attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasDefinitionAttributes {
    #[doc = "the enabled state of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Creation time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[doc = "softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0."]
    #[serde(rename = "recoverableDays", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_days: Option<i32>,
    #[doc = "Reflects the deletion recovery level currently in effect for SAS definitions in the current vault. If it contains 'Purgeable' the SAS definition can be permanently deleted by a privileged user; otherwise, only the system can purge the SAS definition, at the end of the retention interval."]
    #[serde(rename = "recoveryLevel", default, skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<sas_definition_attributes::RecoveryLevel>,
}
impl SasDefinitionAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sas_definition_attributes {
    use super::*;
    #[doc = "Reflects the deletion recovery level currently in effect for SAS definitions in the current vault. If it contains 'Purgeable' the SAS definition can be permanently deleted by a privileged user; otherwise, only the system can purge the SAS definition, at the end of the retention interval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryLevel")]
    pub enum RecoveryLevel {
        Purgeable,
        #[serde(rename = "Recoverable+Purgeable")]
        RecoverablePurgeable,
        Recoverable,
        #[serde(rename = "Recoverable+ProtectedSubscription")]
        RecoverableProtectedSubscription,
        #[serde(rename = "CustomizedRecoverable+Purgeable")]
        CustomizedRecoverablePurgeable,
        CustomizedRecoverable,
        #[serde(rename = "CustomizedRecoverable+ProtectedSubscription")]
        CustomizedRecoverableProtectedSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purgeable => serializer.serialize_unit_variant("RecoveryLevel", 0u32, "Purgeable"),
                Self::RecoverablePurgeable => serializer.serialize_unit_variant("RecoveryLevel", 1u32, "Recoverable+Purgeable"),
                Self::Recoverable => serializer.serialize_unit_variant("RecoveryLevel", 2u32, "Recoverable"),
                Self::RecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 3u32, "Recoverable+ProtectedSubscription")
                }
                Self::CustomizedRecoverablePurgeable => {
                    serializer.serialize_unit_variant("RecoveryLevel", 4u32, "CustomizedRecoverable+Purgeable")
                }
                Self::CustomizedRecoverable => serializer.serialize_unit_variant("RecoveryLevel", 5u32, "CustomizedRecoverable"),
                Self::CustomizedRecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 6u32, "CustomizedRecoverable+ProtectedSubscription")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A SAS definition bundle consists of key vault SAS definition details plus its attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasDefinitionBundle {
    #[doc = "The SAS definition id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Storage account SAS definition secret id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "The SAS definition token template signed with an arbitrary key.  Tokens created according to the SAS definition will have the same properties as the template."]
    #[serde(rename = "templateUri", default, skip_serializing_if = "Option::is_none")]
    pub template_uri: Option<String>,
    #[doc = "The type of SAS token the SAS definition will create."]
    #[serde(rename = "sasType", default, skip_serializing_if = "Option::is_none")]
    pub sas_type: Option<sas_definition_bundle::SasType>,
    #[doc = "The validity period of SAS tokens created according to the SAS definition."]
    #[serde(rename = "validityPeriod", default, skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<String>,
    #[doc = "The SAS definition management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SasDefinitionAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SasDefinitionBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sas_definition_bundle {
    use super::*;
    #[doc = "The type of SAS token the SAS definition will create."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SasType")]
    pub enum SasType {
        #[serde(rename = "account")]
        Account,
        #[serde(rename = "service")]
        Service,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SasType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SasType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SasType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Account => serializer.serialize_unit_variant("SasType", 0u32, "account"),
                Self::Service => serializer.serialize_unit_variant("SasType", 1u32, "service"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The SAS definition create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SasDefinitionCreateParameters {
    #[doc = "The SAS definition token template signed with an arbitrary key.  Tokens created according to the SAS definition will have the same properties as the template."]
    #[serde(rename = "templateUri")]
    pub template_uri: String,
    #[doc = "The type of SAS token the SAS definition will create."]
    #[serde(rename = "sasType")]
    pub sas_type: sas_definition_create_parameters::SasType,
    #[doc = "The validity period of SAS tokens created according to the SAS definition."]
    #[serde(rename = "validityPeriod")]
    pub validity_period: String,
    #[doc = "The SAS definition management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SasDefinitionAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SasDefinitionCreateParameters {
    pub fn new(template_uri: String, sas_type: sas_definition_create_parameters::SasType, validity_period: String) -> Self {
        Self {
            template_uri,
            sas_type,
            validity_period,
            attributes: None,
            tags: None,
        }
    }
}
pub mod sas_definition_create_parameters {
    use super::*;
    #[doc = "The type of SAS token the SAS definition will create."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SasType")]
    pub enum SasType {
        #[serde(rename = "account")]
        Account,
        #[serde(rename = "service")]
        Service,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SasType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SasType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SasType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Account => serializer.serialize_unit_variant("SasType", 0u32, "account"),
                Self::Service => serializer.serialize_unit_variant("SasType", 1u32, "service"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The SAS definition item containing storage SAS definition metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasDefinitionItem {
    #[doc = "The storage SAS identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The storage account SAS definition secret id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "The SAS definition management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SasDefinitionAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SasDefinitionItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage account SAS definition list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasDefinitionListResult {
    #[doc = "A response message containing a list of SAS definitions along with a link to the next page of SAS definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SasDefinitionItem>,
    #[doc = "The URL to get the next set of SAS definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SasDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SasDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAS definition update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasDefinitionUpdateParameters {
    #[doc = "The SAS definition token template signed with an arbitrary key.  Tokens created according to the SAS definition will have the same properties as the template."]
    #[serde(rename = "templateUri", default, skip_serializing_if = "Option::is_none")]
    pub template_uri: Option<String>,
    #[doc = "The type of SAS token the SAS definition will create."]
    #[serde(rename = "sasType", default, skip_serializing_if = "Option::is_none")]
    pub sas_type: Option<sas_definition_update_parameters::SasType>,
    #[doc = "The validity period of SAS tokens created according to the SAS definition."]
    #[serde(rename = "validityPeriod", default, skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<String>,
    #[doc = "The SAS definition management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SasDefinitionAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SasDefinitionUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sas_definition_update_parameters {
    use super::*;
    #[doc = "The type of SAS token the SAS definition will create."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SasType")]
    pub enum SasType {
        #[serde(rename = "account")]
        Account,
        #[serde(rename = "service")]
        Service,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SasType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SasType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SasType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Account => serializer.serialize_unit_variant("SasType", 0u32, "account"),
                Self::Service => serializer.serialize_unit_variant("SasType", 1u32, "service"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The secret management attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretAttributes {
    #[serde(flatten)]
    pub attributes: Attributes,
    #[doc = "softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0."]
    #[serde(rename = "recoverableDays", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_days: Option<i32>,
    #[doc = "Reflects the deletion recovery level currently in effect for secrets in the current vault. If it contains 'Purgeable', the secret can be permanently deleted by a privileged user; otherwise, only the system can purge the secret, at the end of the retention interval."]
    #[serde(rename = "recoveryLevel", default, skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<secret_attributes::RecoveryLevel>,
}
impl SecretAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod secret_attributes {
    use super::*;
    #[doc = "Reflects the deletion recovery level currently in effect for secrets in the current vault. If it contains 'Purgeable', the secret can be permanently deleted by a privileged user; otherwise, only the system can purge the secret, at the end of the retention interval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryLevel")]
    pub enum RecoveryLevel {
        Purgeable,
        #[serde(rename = "Recoverable+Purgeable")]
        RecoverablePurgeable,
        Recoverable,
        #[serde(rename = "Recoverable+ProtectedSubscription")]
        RecoverableProtectedSubscription,
        #[serde(rename = "CustomizedRecoverable+Purgeable")]
        CustomizedRecoverablePurgeable,
        CustomizedRecoverable,
        #[serde(rename = "CustomizedRecoverable+ProtectedSubscription")]
        CustomizedRecoverableProtectedSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purgeable => serializer.serialize_unit_variant("RecoveryLevel", 0u32, "Purgeable"),
                Self::RecoverablePurgeable => serializer.serialize_unit_variant("RecoveryLevel", 1u32, "Recoverable+Purgeable"),
                Self::Recoverable => serializer.serialize_unit_variant("RecoveryLevel", 2u32, "Recoverable"),
                Self::RecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 3u32, "Recoverable+ProtectedSubscription")
                }
                Self::CustomizedRecoverablePurgeable => {
                    serializer.serialize_unit_variant("RecoveryLevel", 4u32, "CustomizedRecoverable+Purgeable")
                }
                Self::CustomizedRecoverable => serializer.serialize_unit_variant("RecoveryLevel", 5u32, "CustomizedRecoverable"),
                Self::CustomizedRecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 6u32, "CustomizedRecoverable+ProtectedSubscription")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A secret consisting of a value, id and its attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretBundle {
    #[doc = "The secret value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The secret id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The content type of the secret."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "If this is a secret backing a KV certificate, then this field specifies the corresponding key backing the KV certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[doc = "True if the secret's lifetime is managed by key vault. If this is a secret backing a certificate, then managed will be true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
}
impl SecretBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The secret item containing secret metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretItem {
    #[doc = "Secret identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Type of the secret value such as a password."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "True if the secret's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
}
impl SecretItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The secret list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretListResult {
    #[doc = "A response message containing a list of secrets in the key vault along with a link to the next page of secrets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SecretItem>,
    #[doc = "The URL to get the next set of secrets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecretListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SecretListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the key backing a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretProperties {
    #[doc = "The media type (MIME type)."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}
impl SecretProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The secret restore parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretRestoreParameters {
    #[doc = "The backup blob associated with a secret bundle."]
    pub value: String,
}
impl SecretRestoreParameters {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "The secret set parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretSetParameters {
    #[doc = "The value of the secret."]
    pub value: String,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Type of the secret value such as a password."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
}
impl SecretSetParameters {
    pub fn new(value: String) -> Self {
        Self {
            value,
            tags: None,
            content_type: None,
            attributes: None,
        }
    }
}
#[doc = "The secret update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretUpdateParameters {
    #[doc = "Type of the secret value such as a password."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The secret management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SecretUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage account management attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountAttributes {
    #[doc = "the enabled state of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Creation time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Last updated time in UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
    #[doc = "softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0."]
    #[serde(rename = "recoverableDays", default, skip_serializing_if = "Option::is_none")]
    pub recoverable_days: Option<i32>,
    #[doc = "Reflects the deletion recovery level currently in effect for storage accounts in the current vault. If it contains 'Purgeable' the storage account can be permanently deleted by a privileged user; otherwise, only the system can purge the storage account, at the end of the retention interval."]
    #[serde(rename = "recoveryLevel", default, skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<storage_account_attributes::RecoveryLevel>,
}
impl StorageAccountAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_account_attributes {
    use super::*;
    #[doc = "Reflects the deletion recovery level currently in effect for storage accounts in the current vault. If it contains 'Purgeable' the storage account can be permanently deleted by a privileged user; otherwise, only the system can purge the storage account, at the end of the retention interval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryLevel")]
    pub enum RecoveryLevel {
        Purgeable,
        #[serde(rename = "Recoverable+Purgeable")]
        RecoverablePurgeable,
        Recoverable,
        #[serde(rename = "Recoverable+ProtectedSubscription")]
        RecoverableProtectedSubscription,
        #[serde(rename = "CustomizedRecoverable+Purgeable")]
        CustomizedRecoverablePurgeable,
        CustomizedRecoverable,
        #[serde(rename = "CustomizedRecoverable+ProtectedSubscription")]
        CustomizedRecoverableProtectedSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purgeable => serializer.serialize_unit_variant("RecoveryLevel", 0u32, "Purgeable"),
                Self::RecoverablePurgeable => serializer.serialize_unit_variant("RecoveryLevel", 1u32, "Recoverable+Purgeable"),
                Self::Recoverable => serializer.serialize_unit_variant("RecoveryLevel", 2u32, "Recoverable"),
                Self::RecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 3u32, "Recoverable+ProtectedSubscription")
                }
                Self::CustomizedRecoverablePurgeable => {
                    serializer.serialize_unit_variant("RecoveryLevel", 4u32, "CustomizedRecoverable+Purgeable")
                }
                Self::CustomizedRecoverable => serializer.serialize_unit_variant("RecoveryLevel", 5u32, "CustomizedRecoverable"),
                Self::CustomizedRecoverableProtectedSubscription => {
                    serializer.serialize_unit_variant("RecoveryLevel", 6u32, "CustomizedRecoverable+ProtectedSubscription")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The storage account create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    #[doc = "Storage account resource id."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Current active storage account key name."]
    #[serde(rename = "activeKeyName")]
    pub active_key_name: String,
    #[doc = "whether keyvault should manage the storage account for the user."]
    #[serde(rename = "autoRegenerateKey")]
    pub auto_regenerate_key: bool,
    #[doc = "The key regeneration time duration specified in ISO-8601 format."]
    #[serde(rename = "regenerationPeriod", default, skip_serializing_if = "Option::is_none")]
    pub regeneration_period: Option<String>,
    #[doc = "The storage account management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<StorageAccountAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageAccountCreateParameters {
    pub fn new(resource_id: String, active_key_name: String, auto_regenerate_key: bool) -> Self {
        Self {
            resource_id,
            active_key_name,
            auto_regenerate_key,
            regeneration_period: None,
            attributes: None,
            tags: None,
        }
    }
}
#[doc = "The storage account item containing storage account metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountItem {
    #[doc = "Storage identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Storage account resource Id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The storage account management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<StorageAccountAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageAccountItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage account key regenerate parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountRegenerteKeyParameters {
    #[doc = "The storage account key name."]
    #[serde(rename = "keyName")]
    pub key_name: String,
}
impl StorageAccountRegenerteKeyParameters {
    pub fn new(key_name: String) -> Self {
        Self { key_name }
    }
}
#[doc = "The storage account update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountUpdateParameters {
    #[doc = "The current active storage account key name."]
    #[serde(rename = "activeKeyName", default, skip_serializing_if = "Option::is_none")]
    pub active_key_name: Option<String>,
    #[doc = "whether keyvault should manage the storage account for the user."]
    #[serde(rename = "autoRegenerateKey", default, skip_serializing_if = "Option::is_none")]
    pub auto_regenerate_key: Option<bool>,
    #[doc = "The key regeneration time duration specified in ISO-8601 format."]
    #[serde(rename = "regenerationPeriod", default, skip_serializing_if = "Option::is_none")]
    pub regeneration_period: Option<String>,
    #[doc = "The storage account management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<StorageAccountAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Storage account bundle consists of key vault storage account details plus its attributes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageBundle {
    #[doc = "The storage account id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The storage account resource id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The current active storage account key name."]
    #[serde(rename = "activeKeyName", default, skip_serializing_if = "Option::is_none")]
    pub active_key_name: Option<String>,
    #[doc = "whether keyvault should manage the storage account for the user."]
    #[serde(rename = "autoRegenerateKey", default, skip_serializing_if = "Option::is_none")]
    pub auto_regenerate_key: Option<bool>,
    #[doc = "The key regeneration time duration specified in ISO-8601 format."]
    #[serde(rename = "regenerationPeriod", default, skip_serializing_if = "Option::is_none")]
    pub regeneration_period: Option<String>,
    #[doc = "The storage account management attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<StorageAccountAttributes>,
    #[doc = "Application specific metadata in the form of key-value pairs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageBundle {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The storage accounts list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageListResult {
    #[doc = "A response message containing a list of storage accounts in the key vault along with a link to the next page of storage accounts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StorageAccountItem>,
    #[doc = "The URL to get the next set of storage accounts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The secret restore parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageRestoreParameters {
    #[doc = "The backup blob associated with a storage account."]
    pub value: String,
}
impl StorageRestoreParameters {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "The subject alternate names of a X509 object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubjectAlternativeNames {
    #[doc = "Email addresses."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub emails: Vec<String>,
    #[doc = "Domain names."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dns_names: Vec<String>,
    #[doc = "User principal names."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub upns: Vec<String>,
}
impl SubjectAlternativeNames {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A condition to be satisfied for an action to be executed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Trigger {
    #[doc = "Percentage of lifetime at which to trigger. Value should be between 1 and 99."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifetime_percentage: Option<i32>,
    #[doc = "Days before expiry to attempt renewal. Value should be between 1 and validity_in_months multiplied by 27. If validity_in_months is 36, then value should be between 1 and 972 (36 * 27)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_before_expiry: Option<i32>,
}
impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the X509 component of a certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509CertificateProperties {
    #[doc = "The subject name. Should be a valid X509 distinguished Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "The enhanced key usage."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ekus: Vec<String>,
    #[doc = "The subject alternate names of a X509 object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sans: Option<SubjectAlternativeNames>,
    #[doc = "List of key usages."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub key_usage: Vec<String>,
    #[doc = "The duration that the certificate is valid in months."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_months: Option<i32>,
}
impl X509CertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
