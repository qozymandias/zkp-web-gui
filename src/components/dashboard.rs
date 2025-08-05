use dioxus::prelude::*;
use zkp_service_helper::helper::ZkWasmServiceHelper;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::ProverNode;
use zkp_service_helper::interface::TaskType;

use crate::config::CONFIG;
use crate::utils::table::SimpleTable;
use crate::views::PairCardsAdjacent;

static ZKH: once_cell::sync::Lazy<ZkWasmServiceHelper> =
    once_cell::sync::Lazy::new(|| ZkWasmServiceHelper::new(CONFIG.api.url.clone()));

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Dashboard() -> Element {
    let mut setups = use_signal(Vec::<ConciseTask>::new);
    use_future(move || async move {
        setups.set(
            ZKH.query_concise_tasks(None, None, None, Some(TaskType::Setup), None, None, Some(5))
                .await
                .map(|res| res.data)
                .unwrap_or(vec![]),
        );
    });

    let mut proves = use_signal(Vec::<ConciseTask>::new);
    use_future(move || async move {
        proves.set(
            ZKH.query_concise_tasks(None, None, None, Some(TaskType::Prove), None, None, Some(5))
                .await
                .map(|res| res.data)
                .unwrap_or(vec![]),
        );
    });

    let mut tasks = use_signal(Vec::<ConciseTask>::new);
    use_future(move || async move {
        tasks.set(
            ZKH.query_concise_tasks(None, None, None, None, None, None, None)
                .await
                .map(|res| res.data)
                .unwrap_or(vec![]),
        );
    });

    let mut provers = use_signal(Vec::<ProverNode>::new);
    use_future(move || async move {
        provers.set(
            ZKH.query_node_statistics(None, Some(0), Some(5))
                .await
                .map(|res| res.data)
                .unwrap_or(vec![]),
        );
    });

    rsx! {
        PairCardsAdjacent { left: setups(), right: proves() }
        SimpleTable { data : provers() }
        SimpleTable { data : tasks() }
    }
}
