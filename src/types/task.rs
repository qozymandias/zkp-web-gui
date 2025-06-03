use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestResult<T: Serialize + Clone> {
    pub success: bool,
    pub result: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationResult<T: Serialize + Clone> {
    pub data: T,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProofSubmitMode {
    Manual,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutoSubmitStatus {
    Round1,
    Round2,
    Batched,
    RegisteredProof,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectId {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    DryRunSuccess,
    DryRunFailed,
    Done,
    Fail,
    Unprovable,
    Stale,
}

impl From<&str> for TaskStatus {
    fn from(value: &str) -> Self {
        match value {
            "Pending" => TaskStatus::Pending,
            "Processing" => TaskStatus::Processing,
            "DryRunSuccess" => TaskStatus::DryRunSuccess,
            "DryRunFailed" => TaskStatus::DryRunFailed,
            "Done" => TaskStatus::Done,
            "Fail" => TaskStatus::Fail,
            "Unprovable" => TaskStatus::Unprovable,
            "Stale" => TaskStatus::Stale,
            _ => unreachable!("Conversion should never be allowed"),
        }
    }
}

impl TaskStatus {
    pub fn to_background_color(&self) -> &str {
        match self {
            TaskStatus::Pending => "#CA9B00",
            TaskStatus::Processing => "#CA9B00",
            TaskStatus::DryRunSuccess => "#CA9B00",
            TaskStatus::DryRunFailed => "#DD6B00",
            TaskStatus::Done => "#3E8166",
            TaskStatus::Fail => "#DD6B00",
            TaskStatus::Unprovable => "red",
            TaskStatus::Stale => "#636363",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    Setup,
    Prove,
    Verify,
    Batch,
    Deploy,
    Reset,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConciseTask {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_address: String,
    pub md5: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub submit_time: String,
    pub process_started: Option<String>,
    pub process_finished: Option<String>,
    pub proof_submit_mode: Option<ProofSubmitMode>,
    pub auto_submit_status: Option<AutoSubmitStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionType {
    None,
    GZip,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InputContextType {
    Custom,
    ImageInitial,
    ImageCurrent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskVerificationData {
    pub static_file_checksum: [u8; 32],
    pub verifier_contracts: Vec<VerifierContracts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierContracts {
    pub chain_id: u32,
    pub aggregator_verifier: String,
    pub batch_verifier: Option<String>,
    pub circuit_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProofData {
    pub round_1_batch_ids: Vec<AutoSubmitBatchData>,
    pub round_2_batch_ids: Vec<AutoSubmitBatchData>,
    pub final_proof_batch_ids: Vec<AutoSubmitBatchData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSubmitBatchData {
    // Use this in metadata to record round1queue id, round2queue id, final proof id etc
    pub id: String,
    pub chain_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskView {
    pub user_address: String,
    pub node_address: Option<String>,
    pub _id: ObjectId,
    pub status: TaskStatus,
    pub md5: String,
    pub task_type: TaskType,
    pub public_inputs: Vec<String>,
    pub private_inputs: Vec<String>,
    pub single_proof: Vec<u8>,
    pub proof: Vec<u8>,
    pub batch_instances: Vec<u8>,
    pub shadow_instances: Vec<u8>,
    pub instances: Vec<u8>,
    pub aux: Vec<u8>,
    pub input_context: Vec<u8>,
    pub input_context_type: Option<InputContextType>,
    pub output_context: Vec<u8>,
    pub chain_id: Option<u32>,
    pub external_host_table: Vec<u8>,
    pub submit_time: String,
    pub process_started: Option<String>,
    pub process_finished: Option<String>,
    pub task_fee: Vec<u8>,
    pub status_message: Option<String>,
    pub internal_message: Option<String>,
    pub guest_statics: Option<u32>,
    pub task_verification_data: TaskVerificationData,
    pub debug_logs: Option<String>,
    pub proof_submit_mode: Option<ProofSubmitMode>,
    pub batch_proof_data: Option<BatchProofData>,
    pub auto_submit_status: Option<AutoSubmitStatus>,
    pub retries_left: u8,
    pub compression: CompressionType,
}
