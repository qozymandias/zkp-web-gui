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
            style: "padding: 1rem;",
            h1 { "{title}" }
            table {
                style: "border-collapse: collapse; width: 100%;",
                thead {
                    tr { { headers.into_iter().map(|it| rsx!{ th { class: "table-row-inner", "{it}" } }) } }
                }
                tbody { { rows.into_iter().map(|row| rsx!{ tr { {
                    row.into_iter().map(|entry| rsx!{
                        td { class: "table-row-inner", { entry.into_cell() } }
                    })
                } } }) } }
            }
        }
    }
}
