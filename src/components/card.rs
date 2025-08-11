use dioxus::prelude::*;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::Task;

use crate::utils::bytes_to_bigint;
use crate::utils::bytes_to_num_string;
use crate::utils::calc_processing_time_secs;
use crate::utils::serde_to_string;
use crate::utils::shorten_addresss;
use crate::utils::shorten_md5;
use crate::utils::timestamp_formatted;

#[derive(PartialEq, Clone, Props)]
pub struct SummaryEntryProps {
    pub md5: String,
    pub task_id: String,
    pub address: String,
    pub date: String,
}

#[component]
pub fn SummaryEntry(props: SummaryEntryProps) -> Element {
    rsx! {
        div {
            class: "setup-entry",
            div {
                style: "flex-direction: column; gap: 4px;",
                div {
                    class: "md5-pill",
                    "MD5 {props.md5}"
                }
                div {
                    class: "task-id",
                    "TaskID {props.task_id}"
                }
            }
            div {
                style: "text-align: right; flex-direction: column; gap: 4px;",
                div { class: "address", "{props.address}" }
                div { class: "date", "{props.date}" }
            }
        }
    }
}

#[component]
pub fn SummaryView(entries: Vec<(String, String, String, String)>) -> Element {
    rsx! {
        div {
            {entries
                .iter()
                .map(|(md5, task_id, address, date)| { rsx!( SummaryEntry {md5, task_id, address, date} )})
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    pub header: String,
    pub header_class: Option<String>,
    pub body: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "card",
            h1 {
                class: "card-header",
                {props.header}
            }
            div {
                class: "card-body",
                {props.body}
            }
        }
    }
}

#[component]
pub fn PairCardsAdjacent(left: Vec<ConciseTask>, right: Vec<ConciseTask>) -> Element {
    let lefts = left
        .into_iter()
        .map(|d| {
            (
                shorten_md5(d.md5),
                d._id.oid,
                shorten_addresss(d.user_address),
                timestamp_formatted(&d.submit_time),
            )
        })
        .collect::<Vec<_>>();
    let rights = right
        .into_iter()
        .map(|d| {
            (
                shorten_md5(d.md5),
                d._id.oid,
                shorten_addresss(d.user_address),
                timestamp_formatted(&d.submit_time),
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        div {
            style: "padding: 1rem; display: flex; flex-direction: row;",
            div {
                style: "flex: 1; padding: 5px;",
                Card {
                    header: "Latest Setups",
                    header_class: "aqua",
                    body: rsx! { SummaryView { entries: lefts } }
                }
            }
            div {
                style: "flex: 1; padding: 5px;",
                Card {
                    header: "Latest Proofs",
                    header_class: "light-blue",
                    body: rsx! { SummaryView { entries: rights } }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum EntryType {
    Raw(String),
    Logs(String),
    TaskType(zkp_service_helper::interface::TaskType),
    TaskStatus(zkp_service_helper::interface::TaskStatus),
    ProofMode(zkp_service_helper::interface::ProofSubmitMode),
    BatchProof(Option<zkp_service_helper::interface::AutoSubmitStatus>),
    Timestamp(String),
    MD5LinkRoundedBox(String),
    AddressLinkRoundedBox(String),
    LongInput(Vec<String>),
    DownloadButton(String),
    Bytes(Vec<u8>, Option<usize>),
}

impl EntryType {
    pub fn into_cell(self) -> Element {
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
            Self::MD5LinkRoundedBox(cell) | Self::AddressLinkRoundedBox(cell) => rsx! {
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
            Self::TaskType(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
            Self::TaskStatus(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
            Self::ProofMode(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
            Self::BatchProof(cell) => Self::Raw(serde_to_string(&cell).unwrap_or("NA".to_string())).into_cell(),
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
        }
    }
}

pub trait EntriesDetailed {
    fn title(&self) -> String;
    fn entries(&self) -> Vec<(&str, EntryType)>;
}

impl EntriesDetailed for Option<Task> {
    fn title(&self) -> String {
        "Task Overview".to_string()
    }

    fn entries(&self) -> Vec<(&str, EntryType)> {
        self.as_ref()
            .map(|it| {
                vec![
                    ("Application", EntryType::MD5LinkRoundedBox(it.md5.clone())),
                    ("Type", EntryType::TaskType(it.task_type.clone())),
                    ("Status", EntryType::TaskStatus(it.status.clone())),
                    ("Submitted at", EntryType::Timestamp(it.submit_time.clone())),
                    ("Submitted by", EntryType::AddressLinkRoundedBox(it.user_address.clone())),
                    (
                        "Task taken by Node",
                        EntryType::AddressLinkRoundedBox(it.node_address.clone().unwrap_or("NA".to_string())),
                    ),
                    (
                        "Processing Started",
                        EntryType::Timestamp(it.process_started.clone().unwrap_or("NA".to_string())),
                    ),
                    (
                        "Processing Finished",
                        EntryType::Timestamp(it.process_finished.clone().unwrap_or("NA".to_string())),
                    ),
                    (
                        "Processing Time",
                        EntryType::Raw(
                            calc_processing_time_secs(it.process_started.clone(), it.process_finished.clone())
                                .map(|dur| format!("{dur} seconds"))
                                .unwrap_or("NA".to_string()),
                        ),
                    ),
                    (
                        "Task Fee",
                        EntryType::Raw(bytes_to_num_string(it.task_fee.clone()).unwrap_or("NA".to_string())),
                    ),
                    ("Debug Logs", EntryType::Logs(it.debug_logs.clone().unwrap_or("NA".to_string()))),
                    (
                        "Guest Statics",
                        EntryType::Raw(it.guest_statics.map(|x| x.to_string()).unwrap_or("NA".to_string())),
                    ),
                    (
                        "Proof Submit Mode",
                        EntryType::ProofMode(
                            it.proof_submit_mode
                                .clone()
                                .unwrap_or(zkp_service_helper::interface::ProofSubmitMode::Manual),
                        ),
                    ),
                    ("Current Batch Status", EntryType::BatchProof(it.auto_submit_status.clone())),
                    ("Public Inputs", EntryType::LongInput(it.public_inputs.clone())),
                    ("Witness", EntryType::LongInput(it.private_inputs.clone())),
                    ("External Host Table", EntryType::DownloadButton(it._id.oid.clone())),
                    ("Input Context", EntryType::Bytes(it.input_context.clone(), Some(8))),
                    ("Context Output", EntryType::Bytes(it.output_context.clone(), Some(8))),
                    ("Single Proof Transcripts", EntryType::Bytes(it.single_proof.clone(), None)),
                    ("Instances", EntryType::Bytes(it.instances.clone(), None)),
                    ("Batched Proof Transcripts", EntryType::Bytes(it.proof.clone(), None)),
                    ("Shadow Instances", EntryType::Bytes(it.shadow_instances.clone(), None)),
                    ("Batch Instances", EntryType::Bytes(it.batch_instances.clone(), None)),
                    ("Aux Data", EntryType::Bytes(it.aux.clone(), None)),
                ]
            })
            .unwrap_or_default()
    }
}

#[component]
pub fn EntirePageCard<T: EntriesDetailed + PartialEq + Clone + 'static>(data: T) -> Element {
    let title = data.title();
    let entries = data.entries();

    let body = rsx! {
        div {
            { entries
                .into_iter()
                .map(|(label, value)| {
                    rsx!{
                        div {
                            id: "detailed-entry",
                            div {
                                style: "width: 18%;",
                                "{label}"
                            }
                            div {
                                { value.into_cell() }
                            }
                        }

                    }})
            }
        }
    };

    rsx! {
        div {
            style: "padding: 1rem; display: flex; flex-direction: row;",
            div {
                style: "flex: 1; padding: 5px;",
                Card {
                    header: "{title}",
                    header_class: "aqua",
                    body: body,
                }
            }
        }
    }
}
