use dioxus::prelude::*;
use zkp_service_helper::interface::AutoSubmitProof;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::ProverNode;
use zkp_service_helper::interface::Round1Info;
use zkp_service_helper::interface::Round2Info;
use zkp_service_helper::interface::TaskType;

use crate::components::card::PairCardsAdjacent;
use crate::components::table::Table;
use crate::ZKH;

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

    let mut auto_submit_task_history = use_signal(Vec::<AutoSubmitProof>::new);
    use_future(move || async move {
        auto_submit_task_history.set(
            ZKH.query_auto_submit_proofs(None, None, None, None, None, Some(0), Some(5))
                .await
                .map(|res| res.data)
                .unwrap_or(vec![]),
        );
    });

    let mut round1_history = use_signal(Vec::<Round1Info>::new);
    use_future(move || async move {
        round1_history.set(
            ZKH.query_round1_info(None, None, None, None, None, None, Some(0), Some(5))
                .await
                .map(|res| res.data)
                .unwrap_or(vec![]),
        );
    });

    let mut round2_history = use_signal(Vec::<Round2Info>::new);
    use_future(move || async move {
        round2_history.set(
            ZKH.query_round2_info(None, None, None, None, None, Some(0), Some(5))
                .await
                .map(|res| res.data)
                .unwrap_or(vec![]),
        );
    });

    rsx! {
        PairCardsAdjacent { left: setups(), right: proves() }
        Table { data : provers() }
        Table { data : tasks() }
        Table { data : auto_submit_task_history() }
        Table { data : round1_history() }
        Table { data : round2_history() }
    }
}
