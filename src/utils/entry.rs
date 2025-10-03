use crate::components::card::EntryLike;
use crate::utils::bytes_to_bigint;
use crate::utils::enum_to_string;
use crate::utils::shorten_address;
use crate::utils::shorten_md5;
use crate::utils::task_status_to_background_color;
use crate::utils::timestamp_formatted;
use crate::utils::TimestampStyle;
use dioxus::prelude::*;
use zkp_service_helper::interface::ImageChecksum;
use zkp_service_helper::interface::TaskStatus;
use zkp_service_helper::interface::TaskType;

#[derive(Clone, PartialEq)]
pub enum AddressKind {
    User,
    Image,
    PrefixedImage,
    Node,
    Task,
    PrefixedTask,
}

#[derive(Clone, PartialEq)]
pub enum AddressStyle {
    Dashboard,
    Detailed,
}

#[derive(Clone, PartialEq, Default)]
pub enum ZkEntry {
    Raw(String),
    #[default]
    Empty,
    LongInput(Vec<String>),
    Logs(Option<String>),
    Address(String, AddressStyle, AddressKind),
    MaybeAddress(Option<String>, AddressStyle, AddressKind),
    Timestamp(Option<String>, TimestampStyle),
    DownloadButton(String),
    Bytes(Vec<u8>, Option<usize>),
    Checksum(Option<ImageChecksum>),
    TaskType(TaskType),
    TaskStatus(TaskStatus),
}

impl EntryLike for ZkEntry {
    fn into_cell(self) -> Element {
        match self {
            Self::Raw(cell) => {
                rsx! {
                    div { "{cell}" }
                }
            }
            Self::Empty => Self::Raw("N/A".to_string()).into_cell(),
            Self::LongInput(cells) => {
                rsx! {
                    div { class: "proof-detail-scroll",
                        {
                            cells.into_iter().enumerate().map(|(i, cell)| rsx! {
                                span { key: "{i}", "{cell}\u{00A0}" }
                            })
                        }
                    }
                }
            }
            Self::Logs(logs) => logs
                .map(|cell| Self::LongInput(cell.split("\n").map(ToString::to_string).collect()))
                .unwrap_or_default()
                .into_cell(),
            Self::Address(addr, style, kind) => {
                rsx! {
                    div {
                        class: match style {
                            AddressStyle::Dashboard => "user-link-no-box",
                            AddressStyle::Detailed => "user-link-pill-box",
                        },
                        Link {
                            to: match kind {
                                AddressKind::User => {
                                    crate::Route::UserDetails {
                                        id: addr.clone(),
                                    }
                                }
                                AddressKind::Image | AddressKind::PrefixedImage => {
                                    crate::Route::ImageDetails {
                                        id: addr.clone(),
                                    }
                                }
                                AddressKind::Node => {
                                    crate::Route::NodeDetails {
                                        id: addr.clone(),
                                    }
                                }
                                AddressKind::Task | AddressKind::PrefixedTask => {
                                    crate::Route::TaskDetails {
                                        id: addr.clone(),
                                    }
                                }
                            },
                            {
                                match kind {
                                    AddressKind::User => {
                                        match style {
                                            AddressStyle::Dashboard => shorten_address(&addr),
                                            AddressStyle::Detailed => addr,
                                        }
                                    }
                                    AddressKind::PrefixedImage => format!("MD5 {}", shorten_md5(addr)),
                                    AddressKind::PrefixedTask => format!("Task ID {addr}"),
                                    _ => addr,
                                }
                            }
                        }
                    }
                }
            }
            Self::MaybeAddress(addr, style, kind) => addr
                .map(|cell| Self::Address(cell, style, kind))
                .unwrap_or_default()
                .into_cell(),
            Self::Timestamp(ts, sty) => ts
                .map(|cell| {
                    rsx! {
                        div { {timestamp_formatted(&cell, sty)} }
                    }
                })
                .unwrap_or(Self::default().into_cell()),
            Self::DownloadButton(_cell) => {
                rsx! {
                    div { class: "user-link-pill-box",
                        a { "Download External Host Table Data" }
                    }
                }
            }
            Self::Bytes(cells, chunksize) => {
                rsx! {
                    div { class: "proof-detail-scroll",
                        {
                            bytes_to_bigint(&cells, chunksize)
                                .into_iter()
                                .map(|cell| format!("{cell:#X}"))
                                .enumerate()
                                .map(|(i, cell)| rsx! {
                                    span { key: "{i}", "{cell}\u{00A0}" }
                                })
                        }
                    }
                }
            }
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
                    div { class: "proof-detail-scroll",
                        span { key: 0, "{x}\u{00A0}" }
                        span { key: 1, "{y}\u{00A0}" }
                    }
                }
            }
            Self::TaskStatus(cell) => {
                rsx! {
                    div {
                        class: "status-rounded-box",
                        background_color: task_status_to_background_color(cell),
                        {enum_to_string(&cell)}
                    }
                }
            }
            Self::TaskType(cell) => Self::Raw(enum_to_string(&cell)).into_cell(),
        }
    }
}
