use dioxus::prelude::*;
use zkp_service_helper::interface::AutoSubmitProof;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::ProverNode;
use zkp_service_helper::interface::Round1Info;
use zkp_service_helper::interface::Round2Info;
use zkp_service_helper::interface::TaskType;

use crate::components::card::Card;
use crate::components::table::Table;
use crate::utils::shorten_addresss;
use crate::utils::shorten_md5;
use crate::utils::timestamp_formatted;
use crate::ZKH;

#[derive(PartialEq, Clone, Props)]
pub struct SummaryEntryProps {
    pub md5: String,
    pub task_id: String,
    pub address: String,
    pub date: String,
}

#[component]
pub fn SummaryEntry(props: SummaryEntryProps) -> Element {
    rsx! {
        div {
            id: "setup-entry",
            div {
                id: "left",
                div {
                    class: "md5-pill",
                    "MD5 {props.md5}"
                }
                div {
                    class: "task-id",
                    "TaskID {props.task_id}"
                }
            }
            div {
                id: "right",
                div { class: "address", "{props.address}" }
                div { class: "date", "{props.date}" }
            }
        }
    }
}

#[component]
pub fn SummaryView(entries: Vec<(String, String, String, String)>) -> Element {
    rsx! {
        div {
            { entries
                .iter()
                .map(|(md5, task_id, address, date)| { rsx!( SummaryEntry {md5, task_id, address, date} )})
            }
        }
    }
}

#[component]
pub fn AdjacentTaskSummaries(left: Vec<ConciseTask>, right: Vec<ConciseTask>) -> Element {
    let lefts = left
        .into_iter()
        .map(|d| {
            (
                shorten_md5(d.md5),
                d._id.oid,
                shorten_addresss(d.user_address),
                timestamp_formatted(&d.submit_time),
            )
        })
        .collect::<Vec<_>>();
    let rights = right
        .into_iter()
        .map(|d| {
            (
                shorten_md5(d.md5),
                d._id.oid,
                shorten_addresss(d.user_address),
                timestamp_formatted(&d.submit_time),
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        div {
            id: "adjacent-task-summaries",
            div {
                Card {
                    header: "Latest Setups",
                    header_class: "aqua",
                    body: rsx! { SummaryView { entries: lefts } }
                }
            }
            div {
                Card {
                    header: "Latest Proofs",
                    header_class: "light-blue",
                    body: rsx! { SummaryView { entries: rights } }
                }
            }
        }
    }
}

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
        AdjacentTaskSummaries { left: setups(), right: proves() }
        Table { data : provers() }
        Table { data : tasks() }
        Table { data : auto_submit_task_history() }
        Table { data : round1_history() }
        Table { data : round2_history() }
    }
}
