use dioxus::prelude::*;
use zkp_service_helper::interface::AutoSubmitProof;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::ProverNode;
use zkp_service_helper::interface::Round1Info;
use zkp_service_helper::interface::Round2Info;

use crate::config::CONFIG;
use crate::utils::link_color;
use crate::utils::link_formatted;
use crate::utils::task_status_to_background_color;
use crate::utils::timestamp_formatted;

use super::serde_to_string;

#[derive(Clone, Debug)]
pub enum CellStyle {
    TaskLink,
    ShortLink,
    ImageLink,
    Raw,
    Timestamp,
    RoundColoredBox,
}

#[derive(Clone, Debug)]
pub struct HeaderType {
    pub name: String,
    pub style: CellStyle,
}

impl Default for HeaderType {
    fn default() -> Self {
        HeaderType {
            name: "Unknown".to_string(),
            style: CellStyle::Raw,
        }
    }
}

impl HeaderType {
    fn make_cell(&self, cell: &str) -> Element {
        match self.style {
            CellStyle::TaskLink | CellStyle::ShortLink | CellStyle::ImageLink => rsx! {
                div {
                    id: "table-links",
                    a {
                        color: link_color(&self.style),
                        href: CONFIG.into_href(vec!["task", cell]),
                        { link_formatted(cell, &self.style) }
                    }
                }
            },
            CellStyle::Raw => rsx! {
                div {
                    text_align: "center",
                    "{cell}"
                }
            },
            CellStyle::Timestamp => rsx! {
                div {
                    text_align: "center",
                    { timestamp_formatted(cell) }
                }
            },
            CellStyle::RoundColoredBox => rsx! {
                div {
                    id: "status-rounded-box", background_color: task_status_to_background_color(cell),
                    "{cell}"
                }
            },
        }
    }
}

pub trait TableLike {
    fn title(&self) -> String;
    fn headers(&self) -> Vec<HeaderType>;
    fn rows(&self) -> Vec<Vec<String>>;

    fn into_header_type(inp: Vec<(&str, CellStyle)>) -> Vec<HeaderType> {
        inp.into_iter()
            .map(|(nm, sty)| HeaderType {
                name: nm.to_string(),
                style: sty,
            })
            .collect()
    }
}

impl TableLike for Vec<ConciseTask> {
    fn title(&self) -> String {
        "Task History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Task Id", CellStyle::TaskLink),
            ("Application Image", CellStyle::ImageLink),
            ("Published By", CellStyle::ShortLink),
            ("Type", CellStyle::Raw),
            ("Submit At", CellStyle::Timestamp),
            ("Status", CellStyle::RoundColoredBox),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row._id.oid.clone(),
                    row.md5.clone(),
                    row.user_address.clone(),
                    serde_to_string(&row.task_type).unwrap_or("Unknown".to_string()),
                    row.submit_time.clone(),
                    serde_to_string(&row.status).unwrap_or("Unknown".to_string()),
                ]
            })
            .collect()
    }
}

impl TableLike for Vec<ProverNode> {
    fn title(&self) -> String {
        "Prover List".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Top Node Addresses", CellStyle::TaskLink),
            ("Successful Tasks", CellStyle::Raw),
            ("Failed Tasks", CellStyle::Raw),
            ("Total Tasks", CellStyle::Raw),
            ("Last Proof Time", CellStyle::Raw),
            ("Last Proof Timestamp", CellStyle::Timestamp),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row.address.clone(),
                    row.statistics.successful_tasks.to_string(),
                    row.statistics.failed_tasks.to_string(),
                    row.statistics.total_tasks.to_string(),
                    row.statistics.last_timed_out.clone().unwrap_or("NA".to_string()),
                    row.last_attempted_task
                        .as_ref()
                        .map(|t| t.timestamp.clone())
                        .unwrap_or("NA".to_string()),
                ]
            })
            .collect()
    }
}

impl TableLike for Vec<AutoSubmitProof> {
    fn title(&self) -> String {
        "Auto Submit Proof Task History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Proof Task ID", CellStyle::TaskLink),
            ("Batch Status", CellStyle::Raw),
            ("Target Proof Submitted", CellStyle::Timestamp),
            ("Network", CellStyle::Raw),
            ("Batch Finished", CellStyle::Raw),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row._id.clone().map(|it| it.oid).unwrap_or("NA".to_string()),
                    serde_to_string(&row.status).unwrap_or("NA".to_string()),
                    row.batch_started.clone().unwrap_or("Not Started".to_string()),
                    row.auto_submit_network_chain_id.to_string(),
                    row.batch_finished.clone().unwrap_or("Not Finished".to_string()),
                ]
            })
            .collect()
    }
}

impl TableLike for Vec<Round1Info> {
    fn title(&self) -> String {
        "Round 1 Proof History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Round 1 Proof ID", CellStyle::TaskLink),
            ("Batch Status", CellStyle::Raw),
            ("Target Proof Submitted", CellStyle::Timestamp),
            ("Network", CellStyle::Raw),
            ("Batch Finished", CellStyle::Raw),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row._id.clone().map(|it| it.oid).unwrap_or("NA".to_string()),
                    serde_to_string(&row.status).unwrap_or("NA".to_string()),
                    row.batch_started.clone().unwrap_or("Not Started".to_string()),
                    row.auto_submit_network_chain_id.to_string(),
                    row.batch_finished.clone().unwrap_or("Not Finished".to_string()),
                ]
            })
            .collect()
    }
}

impl TableLike for Vec<Round2Info> {
    fn title(&self) -> String {
        "Round 2 Proof History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        [
            ("Round 2 Proof ID", CellStyle::TaskLink),
            ("Batch Finished At", CellStyle::Raw),
            ("Aggregator Verifier ", CellStyle::Raw),
        ]
        .into_iter()
        .map(|(nm, sty)| HeaderType {
            name: nm.to_string(),
            style: sty,
        })
        .collect()
    }

    fn rows(&self) -> Vec<Vec<String>> {
        let mut out = vec![];
        for row in self {
            out.push(vec![
                row._id.clone().map(|it| it.oid).unwrap_or("NA".to_string()),
                row.batched_time.clone().unwrap_or("Not Finished".to_string()),
                row.registered_tx_hash.clone().unwrap_or("NA".to_string()),
            ]);
        }
        out
    }
}

#[component]
pub fn SimpleTable<T: TableLike + PartialEq + Clone + 'static>(data: T) -> Element {
    let title = data.title();
    let headers = data.headers();
    let rows = data.rows();
    rsx! {
        div {
            style: "padding: 1rem;",
            h1 { "{title}" }
            table {
                style: "border-collapse: collapse; width: 100%;",
                thead {
                    tr {
                        {headers.iter().map(|h| {
                            let name = h.name.clone();
                            rsx!(
                            th {
                                id: "table-row",
                                "{name}"
                            }
                        )})}
                    }
                }
                tbody {
                    {rows.iter().map(|row| rsx!(
                        tr {
                            {row.iter().enumerate().map(|(i, cell)| rsx!(
                                td {
                                    id: "table-row",
                                    { headers.get(i).cloned().unwrap_or_else(||
                                        {
                                            tracing::info!("Missing header\nCell is {cell:?}\n Header is {headers:?}\n Rows are {rows:?}\n");
                                            HeaderType::default()
                                        }).make_cell(cell) }
                                }
                            ))}
                        }
                    ))}
                }
            }
        }
    }
}

#[component]
pub fn SimpleList() -> Element {
    // TODO: this is placeholder code - rename file to structures or something
    rsx! {
        div {
            style: "display: flex;",
            div {
                id: "links",
                style: "flex: 1; padding: 1rem;",
                "Left column"
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
            div {
                id: "links",
                style: "flex: 1; padding: 1rem;",
                "Right column"
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            }
        }
    }
}

#[component]
pub fn SimpleText(input: String) -> Element {
    rsx! {
        div {
            style: "display: flex;",
            "Hello: {input}"
        }
    }
}
