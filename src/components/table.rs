use dioxus::prelude::*;

use crate::components::card::EntryLike;
use crate::utils::ZkEntry;

pub trait TableLike {
    fn title(&self) -> &str;
    fn headers(&self) -> Vec<&str>;
    fn rows(&self) -> Vec<Vec<ZkEntry>>;
}

#[component]
pub fn Table<T: TableLike + PartialEq + Clone + 'static>(data: T) -> Element {
    let title = data.title();
    let headers = data.headers();
    let rows = data.rows();
    rsx! {
        div {
            style: "padding: 0rem 2rem;",
            h1 { "{title}" }
            table {
                style: "border-collapse: collapse; width: 100%;",
                thead {
                    tr { { headers.into_iter().map(|it| rsx!{ 
                        th { 
                            class: "table-row table-header-color",
                            "{it}" 
                        } }) 
                    } }
                }
                tbody { 
                    { rows.into_iter().enumerate().map(|(i, row)| rsx!{ tr { {
                        row.into_iter().map(|entry| rsx!{
                            td { 
                                class: format!("table-row table-row-{}-color", if i % 2 != 0 { "even" } else { "odd" }),
                                { entry.into_cell() }
                            }
                        })
                    } } }) }
                }
                tfoot { 
                    tr {
                        td { colspan: "2",
                            div {
                                style: "display: flex; justify-content: center; gap: 1rem;",

                                button { onclick: move |_| { /* prev */ }, "Previous" }
                                button { onclick: move |_| { /* next */ }, "Next" }
                            }
                        }
                    }
                }
            }
        }
    }
}
