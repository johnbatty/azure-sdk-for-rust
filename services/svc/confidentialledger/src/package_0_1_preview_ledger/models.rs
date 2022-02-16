#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Information about the enclaves running the Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerEnclaves {
    #[doc = "Identifier for an entity."]
    #[serde(rename = "currentNodeId")]
    pub current_node_id: EntityId,
    #[doc = "Dictionary of enclave quotes, indexed by node id."]
    #[serde(rename = "enclaveQuotes")]
    pub enclave_quotes: EnclaveQuotes,
}
impl ConfidentialLedgerEnclaves {
    pub fn new(current_node_id: EntityId, enclave_quotes: EnclaveQuotes) -> Self {
        Self {
            current_node_id,
            enclave_quotes,
        }
    }
}
#[doc = "An error response from Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerError {
    #[doc = "An error response from Confidential Ledger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConfidentialLedgerErrorBody>,
}
impl ConfidentialLedgerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "An error response from Confidential Ledger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<ConfidentialLedgerErrorBody>>,
}
impl ConfidentialLedgerErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of members in the consortium."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Consortium {
    pub members: Vec<ConsortiumMember>,
}
impl Consortium {
    pub fn new(members: Vec<ConsortiumMember>) -> Self {
        Self { members }
    }
}
#[doc = "Describes a member of the consortium."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsortiumMember {
    #[doc = "PEM-encoded certificate associated with the member."]
    pub certificate: String,
    #[doc = "Identifier for an entity."]
    pub id: EntityId,
}
impl ConsortiumMember {
    pub fn new(certificate: String, id: EntityId) -> Self {
        Self { certificate, id }
    }
}
#[doc = "The governance script for the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constitution {
    #[doc = "SHA256 digest of the constitution script."]
    pub digest: String,
    #[doc = "Contents of the constitution."]
    pub script: String,
}
impl Constitution {
    pub fn new(digest: String, script: String) -> Self {
        Self { digest, script }
    }
}
#[doc = "Contains the enclave quote."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnclaveQuote {
    #[doc = "Identifier for an entity."]
    #[serde(rename = "nodeId")]
    pub node_id: EntityId,
    #[doc = "MRENCLAVE value of the code running in the enclave."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mrenclave: Option<String>,
    #[doc = "Version of the quote presented."]
    #[serde(rename = "quoteVersion")]
    pub quote_version: String,
    #[doc = "Raw SGX quote, parsable by tools like Open Enclave's oeverify."]
    pub raw: String,
}
impl EnclaveQuote {
    pub fn new(node_id: EntityId, quote_version: String, raw: String) -> Self {
        Self {
            node_id,
            mrenclave: None,
            quote_version,
            raw,
        }
    }
}
#[doc = "Dictionary of enclave quotes, indexed by node id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnclaveQuotes {}
impl EnclaveQuotes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EntityId = String;
pub type LedgerEntries = Vec<LedgerEntry>;
#[doc = "An entry in the ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    #[doc = "Contents of the ledger entry."]
    pub contents: String,
    #[doc = "Identifier for sub-ledgers."]
    #[serde(rename = "subLedgerId", default, skip_serializing_if = "Option::is_none")]
    pub sub_ledger_id: Option<SubLedgerId>,
    #[doc = "A unique identifier for the state of the ledger. If returned as part of a LedgerEntry, it indicates the state from which the entry was read."]
    #[serde(rename = "transactionId", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
}
impl LedgerEntry {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            sub_ledger_id: None,
            transaction_id: None,
        }
    }
}
#[doc = "The result of querying for a ledger entry from an older transaction id. The ledger entry is available in the response only if the returned state is Ready."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerQueryResult {
    #[doc = "State of a ledger query."]
    pub state: LedgerQueryState,
    #[doc = "An entry in the ledger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry: Option<LedgerEntry>,
}
impl LedgerQueryResult {
    pub fn new(state: LedgerQueryState) -> Self {
        Self { state, entry: None }
    }
}
#[doc = "State of a ledger query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LedgerQueryState {
    Loading,
    Ready,
}
#[doc = "Details about a Confidential Ledger user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerUser {
    #[doc = "Represents an assignable role."]
    #[serde(rename = "assignedRole")]
    pub assigned_role: LedgerUserRole,
    #[doc = "Identifier for the user. This must either be an AAD object id or a certificate fingerprint."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}
impl LedgerUser {
    pub fn new(assigned_role: LedgerUserRole) -> Self {
        Self {
            assigned_role,
            user_id: None,
        }
    }
}
#[doc = "Represents an assignable role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LedgerUserRole {
    Administrator,
    Contributor,
    Reader,
}
#[doc = "Returned as a result of a write to the Confidential Ledger, the transaction id in the response indicates when the write will become durable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerWriteResult {
    #[doc = "Identifier for sub-ledgers."]
    #[serde(rename = "subLedgerId")]
    pub sub_ledger_id: SubLedgerId,
}
impl LedgerWriteResult {
    pub fn new(sub_ledger_id: SubLedgerId) -> Self {
        Self { sub_ledger_id }
    }
}
pub type MerkleProof = Vec<MerkleProofElement>;
#[doc = "An item in the Merkle proof."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MerkleProofElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}
impl MerkleProofElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated ledger entries returned in response to a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedLedgerEntries {
    #[doc = "State of a ledger query."]
    pub state: LedgerQueryState,
    #[doc = "Path from which to retrieve the next page of results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of ledger entries."]
    pub entries: LedgerEntries,
}
impl PagedLedgerEntries {
    pub fn new(state: LedgerQueryState, entries: LedgerEntries) -> Self {
        Self {
            state,
            next_link: None,
            entries,
        }
    }
}
#[doc = "A receipt certifying the transaction at the specified id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptContents {
    #[doc = "Merkle tree leaf for this transaction."]
    pub leaf: String,
    #[doc = "Identifier for an entity."]
    #[serde(rename = "nodeId")]
    pub node_id: EntityId,
    #[doc = "The Merkle proof verifying a transaction."]
    pub proof: MerkleProof,
    #[doc = "Root of the Merkle tree at the time the transaction was recorded."]
    pub root: String,
    #[doc = "Signature by the node, with its certificate, over the Merkle root."]
    pub signature: String,
}
impl ReceiptContents {
    pub fn new(leaf: String, node_id: EntityId, proof: MerkleProof, root: String, signature: String) -> Self {
        Self {
            leaf,
            node_id,
            proof,
            root,
            signature,
        }
    }
}
#[doc = "Object for assigning a role to a user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[doc = "Represents an assignable role."]
    #[serde(rename = "roleName")]
    pub role_name: LedgerUserRole,
    #[doc = "Description of the role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl RoleAssignment {
    pub fn new(role_name: LedgerUserRole) -> Self {
        Self {
            role_name,
            description: None,
        }
    }
}
pub type SubLedgerId = String;
pub type TransactionId = String;
#[doc = "A receipt certifying the transaction at the specified id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[doc = "A receipt certifying the transaction at the specified id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<ReceiptContents>,
    #[doc = "State of a ledger query."]
    pub state: LedgerQueryState,
    #[doc = "A unique identifier for the state of the ledger. If returned as part of a LedgerEntry, it indicates the state from which the entry was read."]
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
impl TransactionReceipt {
    pub fn new(state: LedgerQueryState, transaction_id: TransactionId) -> Self {
        Self {
            receipt: None,
            state,
            transaction_id,
        }
    }
}
#[doc = "Represents the state of the transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransactionState {
    Committed,
    Pending,
}
#[doc = "Response returned to a query for the transaction status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStatus {
    #[doc = "Represents the state of the transaction."]
    pub state: TransactionState,
    #[doc = "A unique identifier for the state of the ledger. If returned as part of a LedgerEntry, it indicates the state from which the entry was read."]
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
impl TransactionStatus {
    pub fn new(state: TransactionState, transaction_id: TransactionId) -> Self {
        Self { state, transaction_id }
    }
}
pub type UserId = String;
