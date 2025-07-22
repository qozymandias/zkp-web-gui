use dioxus::prelude::*;
use zkp_service_helper::helper::ZkWasmServiceHelper;
use zkp_service_helper::interface::ConciseTask;

use crate::config::CONFIG;
use crate::utils::table::SimpleList;
use crate::utils::table::SimpleTable;

static ZKH: once_cell::sync::Lazy<ZkWasmServiceHelper> =
    once_cell::sync::Lazy::new(|| ZkWasmServiceHelper::new(CONFIG.api.url.clone()));

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Dashboard() -> Element {
    let mut tasks = use_signal(Vec::<ConciseTask>::new);

    use_future(move || async move {
        match ZKH.query_concise_tasks(None, None, None, None, None, None, None).await {
            Ok(data) => tasks.set(data.data),
            Err(_) => {
                tasks.set(vec![]);
            }
        }
    });

    rsx! {
        SimpleList { }
        SimpleTable { data : tasks() }
    }
}
