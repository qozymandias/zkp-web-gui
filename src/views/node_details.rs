use dioxus::prelude::*;
use zkp_service_helper::interface::ProverNode;
use zkp_service_helper::interface::ProverNodeTimeRangeStats;

use crate::components::card::EntryListCard;
use crate::components::card::EntryListLike;
use crate::utils::webtime_to_rfc3339;
use crate::utils::UnwrapOrNA;
use crate::utils::ZkEntry;
use crate::ZKH;

#[derive(Clone, PartialEq)]
struct GeneralNodeDetails {
    node: Option<ProverNode>,
}

impl EntryListLike for GeneralNodeDetails {
    type T = ZkEntry;

    fn title(&self) -> String {
        "Prover Node Statistics".to_string()
    }

    fn entries(&self) -> Vec<(&str, Self::T)> {
        let Some(node) = &self.node else {
            return vec![];
        };
        let stats = &node.statistics;
        vec![
            ("Address", ZkEntry::NodeAddress(node.address.clone())),
            (
                "Latest Node Version Used",
                ZkEntry::Raw(node.version_info.as_ref().map(|it| it.version.clone()).unwrap_or_na()),
            ),
            ("Prover Level", ZkEntry::ProverLevel(node.prover_level.clone())),
            (
                "Status",
                ZkEntry::Raw(
                    node.online_activity
                        .as_ref()
                        .map(|it| if it.online { "Online" } else { "Offline" }.to_string())
                        .unwrap_or_na(),
                ),
            ),
            ("Performance Track", ZkEntry::Raw(node.performance_track.clone())),
            (
                "Last Attempted Task",
                ZkEntry::MaybeTaskId(node.last_attempted_task.as_ref().map(|it| it.task_id.oid.clone())),
            ),
            (
                "Last Attempted Task Timestamp",
                ZkEntry::MaybeTimestamp(node.last_attempted_task.as_ref().map(|it| it.timestamp.clone())),
            ),
            ("Total Tasks Taken", ZkEntry::Raw(stats.total_tasks.to_string())),
            (
                "Total Successful Tasks Completed ",
                ZkEntry::Raw(stats.successful_tasks.to_string()),
            ),
        ]
    }
}

#[derive(Clone, PartialEq)]
struct FailureNodeDetails {
    node: Option<ProverNode>,
}

impl EntryListLike for FailureNodeDetails {
    type T = ZkEntry;

    fn title(&self) -> String {
        "Failure Statistics".to_string()
    }

    fn entries(&self) -> Vec<(&str, Self::T)> {
        let Some(stats) = self.node.as_ref().map(|it| &it.statistics) else {
            return vec![];
        };

        vec![
            ("Failed Tasks", ZkEntry::Raw(stats.failed_tasks.to_string())),
            ("Timed Out Tasks", ZkEntry::Raw(stats.timed_out_count.to_string())),
            (
                "Last Timed Out Timestamp",
                ZkEntry::MaybeTimestamp(stats.last_timed_out.clone()),
            ),
            (
                "Last Timed Out Task Id",
                ZkEntry::MaybeTaskId(stats.last_timed_out_task_id.as_ref().map(|it| it.oid.clone())),
            ),
            (
                "Last Failed Task Timestamp",
                ZkEntry::MaybeTimestamp(stats.last_failed_ts.clone()),
            ),
            (
                "Last Failed Task Id",
                ZkEntry::MaybeTaskId(stats.last_failed_task_id.as_ref().map(|it| it.oid.clone())),
            ),
            (
                "Last Failed Task Logs",
                ZkEntry::Logs(stats.last_failed_task_log.unwrap_or_na()),
            ),
        ]
    }
}

#[derive(Clone, PartialEq)]
struct SuccessfulSetupNodeStats {
    node: Option<ProverNode>,
}

impl EntryListLike for SuccessfulSetupNodeStats {
    type T = ZkEntry;

    fn title(&self) -> String {
        "Successful Setup Statistics".to_string()
    }

    fn entries(&self) -> Vec<(&str, Self::T)> {
        let stats = self.node.as_ref().and_then(|it| it.statistics.setup_timing_stats.as_ref());
        vec![
            (
                "Last Successful Task",
                ZkEntry::MaybeTaskId(stats.and_then(|it| it.latest_task_id.as_ref().map(|it| it.oid.clone()))),
            ),
            (
                "Latest Processing Time",
                ZkEntry::Raw(
                    stats
                        .map(|it| format!("{:.4} seconds", it.latest_time_taken_secs))
                        .unwrap_or_na(),
                ),
            ),
            (
                "Latest Timestamp",
                ZkEntry::MaybeTimestamp(stats.and_then(|it| it.latest_timestamp.clone())),
            ),
        ]
    }
}

#[derive(Clone, PartialEq)]
struct SuccessfulProveNodeStats {
    node: Option<ProverNode>,
}

impl EntryListLike for SuccessfulProveNodeStats {
    type T = ZkEntry;

    fn title(&self) -> String {
        "Successful Proof Statistics".to_string()
    }

    fn entries(&self) -> Vec<(&str, Self::T)> {
        let stats = self.node.as_ref().and_then(|it| it.statistics.proof_timing_stats.as_ref());
        vec![
            (
                "Last Successful Task",
                ZkEntry::MaybeTaskId(stats.and_then(|it| it.latest_task_id.as_ref().map(|it| it.oid.clone()))),
            ),
            (
                "Latest Processing Time",
                ZkEntry::Raw(
                    stats
                        .map(|it| format!("{:.4} seconds", it.latest_time_taken_secs))
                        .unwrap_or_na(),
                ),
            ),
            (
                "Latest Timestamp",
                ZkEntry::MaybeTimestamp(stats.and_then(|it| it.latest_timestamp.clone())),
            ),
        ]
    }
}

#[derive(Clone, PartialEq)]
struct LastMonthsNodeStats {
    timerange_stats: Option<ProverNodeTimeRangeStats>,
}

impl EntryListLike for LastMonthsNodeStats {
    type T = ZkEntry;

    fn title(&self) -> String {
        "Last Month Statistics".to_string()
    }

    fn entries(&self) -> Vec<(&str, Self::T)> {
        let Some(stats) = &self.timerange_stats else {
            return vec![];
        };

        vec![
            ("Successful Tasks Number", ZkEntry::Raw(stats.stats.successful.to_string())),
            ("Failed Tasks Number", ZkEntry::Raw(stats.stats.failed.to_string())),
            ("Timed Out Tasks Number", ZkEntry::Raw(stats.stats.timed_out.to_string())),
        ]
    }
}

#[component]
pub fn NodeDetails(id: String) -> Element {
    tracing::info!("Node detail loading {id}");

    let mut node = use_signal(|| Option::<ProverNode>::None);
    let id_for_node = id.clone();
    use_future(move || {
        let address = id_for_node.clone();
        async move {
            node.set(
                ZKH.query_node_statistics(Some(address), None, Some(1))
                    .await
                    .inspect_err(|e| tracing::error!("{e}"))
                    .ok()
                    .and_then(|res| res.data.first().cloned()),
            );
        }
    });

    let id_for_stats = id.clone();
    let mut stats = use_signal(|| Option::<ProverNodeTimeRangeStats>::None);
    use_future(move || {
        let address = id_for_stats.clone();
        let now = web_time::SystemTime::now();
        let then = now - web_time::Duration::from_secs(4 * 7 * 24 * 60 * 60);
        async move {
            stats.set(
                ZKH.query_prover_node_timerange_stats(address, webtime_to_rfc3339(then), webtime_to_rfc3339(now))
                    .await
                    .inspect_err(|e| tracing::error!("{e}"))
                    .ok(),
            );
        }
    });

    let node_details = node();
    let address = id.clone();
    rsx! {
        div {
            style: "padding: 2rem;",
            div {
                id: "detail-header",
                div {
                    "{address}"
                }
            },
        }
        div {
            class: "node-details-wrapper",
            EntryListCard {
                data: GeneralNodeDetails {
                    node: node_details.clone(),
                },
                card_class: "transparent-border",
                header_class: "node-details-header",
                lcol_class: "node-details-col",
            }
        }
        div {
            class: "node-details-wrapper",
            EntryListCard {
                data: FailureNodeDetails {
                    node: node_details.clone(),
                },
                card_class: "transparent-border",
                header_class: "node-details-header",
                lcol_class: "node-details-col",
            }
        }
        div {
            style: "padding: 0.1rem 2rem;",
            EntryListCard {
                data: SuccessfulSetupNodeStats {
                    node: node_details.clone(),
                },
                card_class: "transparent-border",
                header_class: "node-details-header",
                lcol_class: "node-details-col",
            }
        }
        div {
            style: "padding: 0.1rem 2rem;",
            EntryListCard {
                data: SuccessfulProveNodeStats {
                    node: node_details.clone(),
                },
                card_class: "transparent-border",
                header_class: "node-details-header",
                lcol_class: "node-details-col",
            }
        }
        div {
            style: "padding: 0.1rem 2rem;",
            EntryListCard {
                data: LastMonthsNodeStats {
                    timerange_stats: stats(),
                },
                card_class: "transparent-border",
                header_class: "node-details-header",
                lcol_class: "node-details-col",
            }
        }
    }
}
