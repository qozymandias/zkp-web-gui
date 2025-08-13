use dioxus::prelude::*;
use zkp_service_helper::interface::AutoSubmitStatus;
use zkp_service_helper::interface::ImageChecksum;
use zkp_service_helper::interface::ProofSubmitMode;
use zkp_service_helper::interface::Task;
use zkp_service_helper::interface::TaskStatus;
use zkp_service_helper::interface::TaskType;

use crate::components::card::EntryListT;
use crate::components::card::EntryT;
use crate::utils::bytes_to_bigint;
use crate::utils::bytes_to_num_string;
use crate::utils::calc_processing_time_secs;
use crate::utils::serde_to_string;
use crate::utils::timestamp_formatted;

#[derive(Clone, PartialEq)]
pub enum Entry {
    Raw(String),
    Logs(String),
    TaskType(TaskType),
    TaskStatus(TaskStatus),
    ProofMode(ProofSubmitMode),
    BatchProof(Option<AutoSubmitStatus>),
    Timestamp(String),
    MD5LinkRoundedBox(String),
    AddressLinkRoundedBox(String),
    LongInput(Vec<String>),
    DownloadButton(String),
    Bytes(Vec<u8>, Option<usize>),
    Checksum(Option<ImageChecksum>),
}

impl EntryT for Entry {
    fn into_cell(self) -> Element {
        match self {
            Self::Raw(cell) => rsx! {
                div {
                    "{cell}"
                }
            },
            Self::Timestamp(cell) => rsx! {
                div {
                    { timestamp_formatted(&cell) }
                }
            },
            Self::MD5LinkRoundedBox(cell) => rsx! {
                div {
                    id: "link-pill-box",
                    Link {
                        color: "white",
                        to: crate::Route::ImageDetails { id : cell.to_string() },
                        "{cell}"
                    }
                }
            },
            Self::AddressLinkRoundedBox(cell) => rsx! {
                div {
                    id: "link-pill-box",
                    Link {
                        color: "white",
                        to: crate::Route::TaskDetails { id : cell.to_string() },
                        "{cell}"
                    }
                }
            },
            Self::DownloadButton(_cell) => rsx! {
                div {
                    id: "link-pill-box",
                    a {
                        "Download External Host Table Data"
                    }
                }
            },
            Self::LongInput(cells) => rsx! {
                div {
                    class: "proof-detail-scroll",
                    { cells
                        .into_iter()
                        .enumerate()
                        .map(|(i, cell)| rsx! { span { key: "{i}", "{cell}\u{00A0}" } }) }
                }
            },
            Self::Bytes(cells, chunksize) => rsx! {
                div {
                    class: "proof-detail-scroll",
                    { bytes_to_bigint(&cells, chunksize)
                        .into_iter()
                        .map(|cell| format!("{cell:#X}"))
                        .enumerate()
                        .map(|(i, cell)| rsx! { span { key: "{i}", "{cell}\u{00A0}" } }) }
                }
            },
            Self::Logs(cell) => Self::LongInput(cell.split("\n").map(ToString::to_string).collect()).into_cell(),
            Self::TaskType(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
            Self::TaskStatus(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
            Self::ProofMode(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
            Self::BatchProof(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
            Self::Checksum(cell) => {
                let x = cell
                    .as_ref()
                    .map(|it| format!("0x{}", hex::encode(&it.x)))
                    .unwrap_or("NA".to_string());
                let y = cell
                    .as_ref()
                    .map(|it| format!("0x{}", hex::encode(&it.y)))
                    .unwrap_or("NA".to_string());
                rsx! {
                    div {
                        class: "proof-detail-scroll",
                        span { key: 0, "{x}\u{00A0}" }
                        span { key: 1, "{y}\u{00A0}" }
                    }
                }
            }
        }
    }
}

impl EntryListT for Option<Task> {
    type T = Entry;

    fn title(&self) -> String {
        "Task Overview".to_string()
    }

    fn entries(&self) -> Vec<(&str, Entry)> {
        self.as_ref()
            .map(|it| {
                vec![
                    ("Application", Entry::MD5LinkRoundedBox(it.md5.clone())),
                    ("Type", Entry::TaskType(it.task_type.clone())),
                    ("Status", Entry::TaskStatus(it.status.clone())),
                    ("Submitted at", Entry::Timestamp(it.submit_time.clone())),
                    ("Submitted by", Entry::AddressLinkRoundedBox(it.user_address.clone())),
                    (
                        "Task taken by Node",
                        Entry::AddressLinkRoundedBox(it.node_address.clone().unwrap_or("NA".to_string())),
                    ),
                    (
                        "Processing Started",
                        Entry::Timestamp(it.process_started.clone().unwrap_or("NA".to_string())),
                    ),
                    (
                        "Processing Finished",
                        Entry::Timestamp(it.process_finished.clone().unwrap_or("NA".to_string())),
                    ),
                    (
                        "Processing Time",
                        Entry::Raw(
                            calc_processing_time_secs(it.process_started.clone(), it.process_finished.clone())
                                .map(|dur| format!("{dur} seconds"))
                                .unwrap_or("NA".to_string()),
                        ),
                    ),
                    (
                        "Task Fee",
                        Entry::Raw(bytes_to_num_string(it.task_fee.clone()).unwrap_or("NA".to_string())),
                    ),
                    ("Debug Logs", Entry::Logs(it.debug_logs.clone().unwrap_or("NA".to_string()))),
                    (
                        "Guest Statics",
                        Entry::Raw(it.guest_statics.map(|x| x.to_string()).unwrap_or("NA".to_string())),
                    ),
                    (
                        "Proof Submit Mode",
                        Entry::ProofMode(
                            it.proof_submit_mode
                                .clone()
                                .unwrap_or(zkp_service_helper::interface::ProofSubmitMode::Manual),
                        ),
                    ),
                    ("Current Batch Status", Entry::BatchProof(it.auto_submit_status.clone())),
                    ("Public Inputs", Entry::LongInput(it.public_inputs.clone())),
                    ("Witness", Entry::LongInput(it.private_inputs.clone())),
                    ("External Host Table", Entry::DownloadButton(it._id.oid.clone())),
                    ("Input Context", Entry::Bytes(it.input_context.clone(), Some(8))),
                    ("Context Output", Entry::Bytes(it.output_context.clone(), Some(8))),
                    ("Single Proof Transcripts", Entry::Bytes(it.single_proof.clone(), None)),
                    ("Instances", Entry::Bytes(it.instances.clone(), None)),
                    ("Batched Proof Transcripts", Entry::Bytes(it.proof.clone(), None)),
                    ("Shadow Instances", Entry::Bytes(it.shadow_instances.clone(), None)),
                    ("Batch Instances", Entry::Bytes(it.batch_instances.clone(), None)),
                    ("Aux Data", Entry::Bytes(it.aux.clone(), None)),
                ]
            })
            .unwrap_or_default()
    }
}
