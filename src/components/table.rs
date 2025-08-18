use dioxus::prelude::*;

use crate::components::card::EntryLike;
use crate::utils::ZkEntry;

pub trait TableLike {
    fn title(&self) -> &str;
    fn headers(&self) -> Vec<&str>;
    fn rows(&self) -> Vec<Vec<ZkEntry>>;
}

#[component]
pub fn Table<T: TableLike + PartialEq + Clone + 'static>(data: T, pagination: Option<PaginationHandler>) -> Element {
    let title = data.title();
    let headers = data.headers();
    let n = headers.len();
    let rows = data.rows();

    rsx! {
        div { style: "padding: 0rem 2rem;",
            h1 { "{title}" }
            table { style: "border-collapse: collapse; width: 100%;",
                thead {
                    tr {
                        {headers.into_iter().map(|it| rsx! {
                            th { class: "table-row table-header-color", "{it}" }
                        })}
                    }
                }
                tbody {
                    {rows.into_iter().enumerate().map(|(i, row)| rsx! {
                        tr {
                            {row.into_iter().map(|entry| rsx! {
                                td { class: format!("table-row table-row-{}-color", if i % 2 != 0 { "even" } else { "odd" }),
                                    {entry.into_cell()}
                                }
                            })}
                        }
                    })}
                }
                if let Some(pagn) = pagination {
                    tfoot {
                        tr {
                            td { colspan: "{n}",
                                div { class: "table-row table-header-color pagination-button",
                                    button { onclick: pagn.begin, "<<" }
                                    button { onclick: pagn.prev, "<" }
                                    "{pagn.curr} of {pagn.total}"
                                    button { onclick: pagn.next, ">" }
                                    button { onclick: pagn.last, ">>" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub const N_PAGINATED: u64 = 10;

#[derive(Clone, PartialEq)]
pub struct PaginationHandler {
    begin: EventHandler<MouseEvent>,
    last: EventHandler<MouseEvent>,
    prev: EventHandler<MouseEvent>,
    next: EventHandler<MouseEvent>,
    curr: u64,
    total: u64,
}

impl PaginationHandler {
    pub fn new(
        begin: EventHandler<MouseEvent>,
        last: EventHandler<MouseEvent>,
        prev: EventHandler<MouseEvent>,
        next: EventHandler<MouseEvent>,
        curr: u64,
        total: u64,
    ) -> Self {
        Self { begin, last, prev, next, curr, total }
    }

    pub fn default(total: u64, mut curr: Signal<u64>) -> Self {
        let total_p = total / N_PAGINATED;

        PaginationHandler::new(
            EventHandler::new(move |_| {
                curr.set(0);
            }),
            EventHandler::new(move |_| {
                curr.set(total_p);
            }),
            EventHandler::new(move |_| {
                curr.set(std::cmp::max(0, curr() - 1));
            }),
            EventHandler::new(move |_| {
                curr.set(std::cmp::min(total_p, curr() + 1));
            }),
            curr(),
            total / N_PAGINATED,
        )
    }
}

