use std::future::Future;
use std::pin::Pin;

use dioxus::prelude::*;
use serde::Serialize;
use zkp_service_helper::interface::PaginationResult;

use crate::components::card::EntryLike;
use crate::utils::ZkEntry;
use crate::GLOBAL_PADDING;

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
        div { style: GLOBAL_PADDING,
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
                                    "{pagn.curr + 1} of {pagn.total + 1}"
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

    pub fn default(total: u64, mut curr: Signal<u64>, n_paginated: Option<u64>) -> Self {
        let n_pagn = n_paginated.unwrap_or(10);
        let total_p = total / n_pagn;

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
            total / n_pagn,
        )
    }
}

pub trait PaginatedTableLike: TableLike + Serialize + Clone + PartialEq + 'static {
    type Inputs: Clone + PartialEq + Serialize + 'static = ();

    type Data: TableLike + Serialize + Clone + PartialEq + 'static = Self;

    type Fut: Future<Output = PaginationResult<Self::Data>> + 'static =
        Pin<Box<dyn Future<Output = PaginationResult<Self::Data>>>>;

    fn n_per_paginated() -> u64;

    fn query_function() -> Box<dyn Fn(u64, u64, Option<Self::Inputs>) -> Self::Fut>;

    fn paginated_table_handler(
        n: u64,
        inps: Memo<Option<Self::Inputs>>,
        future: Box<dyn Fn(u64, u64, Option<Self::Inputs>) -> Self::Fut>,
    ) -> Element {
        let curr = use_signal(|| 0u64);
        let future = std::rc::Rc::new(future);
        let resource = use_resource(move || {
            let fut = future.clone();
            async move { fut(curr() * n, n, inps()).await }
        });
        let loaded_resource = match resource.state().cloned() {
            UseResourceState::Ready => Some(resource.value().unwrap()),
            _ => None,
        };

        rsx! {
            if let Some(res) = loaded_resource {
                Table {
                    data: res.data,
                    pagination: PaginationHandler::default(res.total, curr, Some(n)),
                }
            }
        }
    }
}

#[component]
pub fn PaginatedTable<T: PaginatedTableLike + PartialEq + Clone + 'static>(inputs: Memo<Option<T::Inputs>>) -> Element {
    rsx! {
        {T::paginated_table_handler(T::n_per_paginated(), inputs, T::query_function())}
    }
}

#[component]
pub fn PaginatedTableNoInputs<T: PaginatedTableLike + PartialEq + Clone + 'static>() -> Element {
    let inps = use_memo(|| Option::<T::Inputs>::None);
    rsx! {
        {T::paginated_table_handler(T::n_per_paginated(), inps, T::query_function())}
    }
}
