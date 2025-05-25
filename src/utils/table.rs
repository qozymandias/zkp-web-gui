use dioxus::prelude::*;

use crate::types::task::ConciseTask;

pub trait TableLike {
    fn title(&self) -> String;
    fn headers(&self) -> Vec<String>;
    fn rows(&self) -> Vec<Vec<String>>;
}

#[derive(Clone, PartialEq)]
pub struct MockTable;

impl TableLike for MockTable {
    fn title(&self) -> String {
        "Mock table example table".to_string()
    }

    fn headers(&self) -> Vec<String> {
        ["Task Id", "Address", "Status"].iter().map(|it| it.to_string()).collect()
    }

    fn rows(&self) -> Vec<Vec<String>> {
        vec![["111", "222", "Hello"].iter().map(|it| it.to_string()).collect()]
    }
}

impl TableLike for Vec<ConciseTask> {
    fn title(&self) -> String {
        "Task History".to_string()
    }

    fn headers(&self) -> Vec<String> {
        [
            "Task Id",
            "Application Image",
            "Published By",
            "Type",
            "Submit At",
            "Status",
        ]
        .iter()
        .map(|it| it.to_string())
        .collect()
    }

    fn rows(&self) -> Vec<Vec<String>> {
        let mut out = vec![];
        for row in self {
            out.push(vec![
                row.id.as_ref().map(|id| id.oid.clone()).unwrap_or("Unknown".to_string()),
                row.md5.clone(),
                row.user_address.clone(),
                format!("{:?}", row.task_type),
                row.submit_time.clone(),
                format!("{:?}", row.status),
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
                        {headers.iter().map(|h| rsx!(
                            th {
                                id: "table-row",
                                "{h}"
                            }
                        ))}
                    }
                }
                tbody {
                    {rows.iter().map(|row| rsx!(
                        tr {
                            {row.iter().map(|cell| rsx!(
                                td {
                                    id: "table-row",
                                    div {
                                        id: "table-links",
                                        a {
                                            href: "https://dioxuslabs.com/learn/0.6/",
                                            "{cell}"
                                        }
                                    }
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
