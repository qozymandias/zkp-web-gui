use dioxus::prelude::*;
use zkp_service_helper::interface::ConciseTask;

use crate::utils::link_color;
use crate::utils::link_formatted;
use crate::utils::timestamp_formatted;
use crate::utils::task_status_to_background_color;
use crate::config::CONFIG;

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
}

impl TableLike for Vec<ConciseTask> {
    fn title(&self) -> String {
        "Task History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        [
            ("Task Id", CellStyle::TaskLink),
            ("Application Image", CellStyle::ImageLink),
            ("Published By", CellStyle::ShortLink),
            ("Type", CellStyle::Raw),
            ("Submit At", CellStyle::Timestamp),
            ("Status", CellStyle::RoundColoredBox),
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
                row._id.oid.clone(),
                row.md5.clone(),
                row.user_address.clone(),
                serde_to_string(&row.task_type).unwrap_or("Unknown".to_string()),
                row.submit_time.clone(),
                serde_to_string(&row.status).unwrap_or("Unknown".to_string()),
            ]);
        }
        out
    }
}

#[component]
pub fn SimpleTable<T: TableLike + PartialEq + 'static>(data: T) -> Element {
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
