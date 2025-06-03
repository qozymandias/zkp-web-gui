use dioxus::prelude::*;

use crate::types::task::ConciseTask;
use crate::utils::request::get_tasklist;
use crate::utils::table::SimpleList;
use crate::utils::table::SimpleTable;

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Dashboard() -> Element {
    let mut tasks = use_signal(|| None::<Vec<ConciseTask>>);

    use_future(move || async move {
        match get_tasklist().await {
            Ok(data) => tasks.set(Some(data.result.data)),
            Err(_) => {
                tasks.set(None);
            }
        }
    });

    rsx! {
        SimpleList { }
        SimpleTable { data : tasks().unwrap_or(vec![]) }
    }
}
