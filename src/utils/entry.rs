use dioxus::prelude::*;
use zkp_service_helper::interface::AutoSubmitStatus;
use zkp_service_helper::interface::ImageChecksum;
use zkp_service_helper::interface::ProofSubmitMode;
use zkp_service_helper::interface::ProverLevel;
use zkp_service_helper::interface::TaskStatus;
use zkp_service_helper::interface::TaskType;

use crate::components::card::EntryLike;
use crate::utils::bytes_to_bigint;
use crate::utils::serde_to_string;
use crate::utils::timestamp_formatted;

#[derive(Clone, PartialEq)]
pub enum ZkEntry {
    Raw(String),
    Logs(String),
    TaskType(TaskType),
    TaskStatus(TaskStatus),
    ProofMode(ProofSubmitMode),
    BatchProof(Option<AutoSubmitStatus>),
    ProverLevel(ProverLevel),
    Timestamp(String),
    MaybeTimestamp(Option<String>),
    MD5(String),
    NodeAddress(String),
    UserAddress(String),
    MaybeTaskId(Option<String>),
    LongInput(Vec<String>),
    DownloadButton(String),
    Bytes(Vec<u8>, Option<usize>),
    Checksum(Option<ImageChecksum>),
}

impl EntryLike for ZkEntry {
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
            Self::MaybeTimestamp(cell) => match cell {
                Some(ts) => Self::Timestamp(ts).into_cell(),
                None => Self::Raw("N/A".to_string()).into_cell(),
            },
            Self::MD5(cell) => rsx! {
                div {
                    id: "link-pill-box",
                    Link {
                        color: "white",
                        to: crate::Route::ImageDetails { id : cell.to_string() },
                        "{cell}"
                    }
                }
            },
            Self::NodeAddress(cell) => rsx! {
                div {
                    id: "link-pill-box",
                    Link {
                        color: "white",
                        to: crate::Route::NodeDetails { id : cell.to_string() },
                        "{cell}"
                    }
                }
            },
            Self::UserAddress(cell) => rsx! {
                div {
                    id: "link-pill-box",
                    Link {
                        color: "white",
                        to: crate::Route::NodeDetails { id : cell.to_string() },
                        "{cell}"
                    }
                }
            },
            Self::MaybeTaskId(cell) => match cell {
                Some(c) => rsx! {
                    div {
                        id: "link-pill-box",
                        Link {
                            color: "white",
                            to: crate::Route::TaskDetails { id : c.clone() },
                            "{c}"
                        }
                    }
                },
                None => Self::Raw("N/A".to_string()).into_cell(),
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
            Self::ProverLevel(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
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
