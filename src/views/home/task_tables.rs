use std::future::Future;
use std::pin::Pin;

use dioxus::prelude::*;
use zkp_service_helper::interface::AutoSubmitProof;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::PaginationResult;
use zkp_service_helper::interface::ProverNode;
use zkp_service_helper::interface::Round1Info;
use zkp_service_helper::interface::Round2Info;

use crate::components::table::PaginatedTable;
use crate::components::table::PaginatedTableLike;
use crate::components::table::TableLike;
use crate::utils::serde_to_string;
use crate::utils::AddressKind;
use crate::utils::AddressStyle;
use crate::utils::TimestampStyle;
use crate::utils::UnwrapOrEmpty;
use crate::utils::UnwrapOrNA;
use crate::utils::ZkEntry;
use crate::ZKH;

impl TableLike for Vec<ProverNode> {
    fn title(&self) -> &str {
        "Prover List"
    }

    fn headers(&self) -> Vec<&str> {
        vec![
            "Top Node Addresses",
            "Successful Tasks",
            "Failed Tasks",
            "Total Tasks",
            "Last Proof Time",
            "Last Proof Timestamp",
        ]
    }

    fn rows(&self) -> Vec<Vec<ZkEntry>> {
        self.iter()
            .map(|row| {
                vec![
                    ZkEntry::Address(row.address.clone(), AddressStyle::Dashboard, AddressKind::Node),
                    ZkEntry::Raw(row.statistics.successful_tasks.to_string()),
                    ZkEntry::Raw(row.statistics.failed_tasks.to_string()),
                    ZkEntry::Raw(row.statistics.total_tasks.to_string()),
                    ZkEntry::Raw(
                        row.statistics
                            .proof_timing_stats
                            .as_ref()
                            .map(|t| format!("{:.4}", t.latest_time_taken_secs))
                            .unwrap_or_na(),
                    ),
                    ZkEntry::Timestamp(
                        row.last_attempted_task.as_ref().map(|t| t.timestamp.clone()),
                        TimestampStyle::Full,
                    ),
                ]
            })
            .collect()
    }
}

impl PaginatedTableLike for Vec<ProverNode> {
    fn n_per_paginated() -> u64 {
        5
    }

    fn query_function() -> fn(page: u64, per: u64) -> Self::Fut {
        fn fetch(page: u64, per: u64) -> Pin<Box<dyn Future<Output = PaginationResult<Vec<ProverNode>>>>> {
            Box::pin(async move { ZKH.query_node_statistics(None, Some(page), Some(per)).await.unwrap_or_empty() })
        }
        fetch
    }
}

impl TableLike for Vec<ConciseTask> {
    fn title(&self) -> &str {
        "Task History"
    }

    fn headers(&self) -> Vec<&str> {
        vec![
            "Task Id",
            "Application Image",
            "Published By",
            "Type",
            "Submit At",
            "Status",
        ]
    }

    fn rows(&self) -> Vec<Vec<ZkEntry>> {
        self.iter()
            .map(|row| {
                vec![
                    ZkEntry::MaybeAddress(Some(row._id.oid.clone()), AddressStyle::Dashboard, AddressKind::Task),
                    ZkEntry::Address(row.md5.clone(), AddressStyle::Dashboard, AddressKind::Image),
                    ZkEntry::Address(row.user_address.clone(), AddressStyle::Dashboard, AddressKind::User),
                    ZkEntry::TaskType(row.task_type.clone()),
                    ZkEntry::Timestamp(Some(row.submit_time.clone()), TimestampStyle::Simple),
                    ZkEntry::TaskStatus(row.status.clone()),
                ]
            })
            .collect()
    }
}

impl PaginatedTableLike for Vec<ConciseTask> {
    fn n_per_paginated() -> u64 {
        10
    }

    fn query_function() -> fn(page: u64, per: u64) -> Self::Fut {
        fn fetch(page: u64, per: u64) -> Pin<Box<dyn Future<Output = PaginationResult<Vec<ConciseTask>>>>> {
            Box::pin(async move {
                ZKH.query_concise_tasks(None, None, None, None, None, Some(page), Some(per))
                    .await
                    .unwrap_or_empty()
            })
        }
        fetch
    }
}

impl TableLike for Vec<AutoSubmitProof> {
    fn title(&self) -> &str {
        "Auto Submit Proof Task History"
    }

    fn headers(&self) -> Vec<&str> {
        vec![
            "Proof Task ID",
            "Batch Status",
            "Target Proof Submitted",
            "Network",
            "Batch Finished",
        ]
    }

    fn rows(&self) -> Vec<Vec<ZkEntry>> {
        self.iter()
            .map(|row| {
                vec![
                    ZkEntry::MaybeAddress(row._id.clone().map(|it| it.oid), AddressStyle::Dashboard, AddressKind::Task),
                    ZkEntry::Raw(serde_to_string(&row.status).ok().unwrap_or_na()),
                    ZkEntry::Timestamp(row.batch_started.clone(), TimestampStyle::Full),
                    ZkEntry::Raw(row.auto_submit_network_chain_id.to_string()),
                    ZkEntry::Timestamp(row.batch_finished.clone(), TimestampStyle::Full),
                ]
            })
            .collect()
    }
}

impl PaginatedTableLike for Vec<AutoSubmitProof> {
    fn n_per_paginated() -> u64 {
        5
    }

    fn query_function() -> fn(page: u64, per: u64) -> Self::Fut {
        fn fetch(page: u64, per: u64) -> Pin<Box<dyn Future<Output = PaginationResult<Vec<AutoSubmitProof>>>>> {
            Box::pin(async move {
                ZKH.query_auto_submit_proofs(None, None, None, None, None, Some(page), Some(per))
                    .await
                    .unwrap_or_empty()
            })
        }
        fetch
    }
}

impl TableLike for Vec<Round1Info> {
    fn title(&self) -> &str {
        "Round 1 Proof History"
    }

    fn headers(&self) -> Vec<&str> {
        vec![
            "Round 1 Proof ID",
            "Batch Status",
            "Target Proof Submitted",
            "Network",
            "Batch Finished",
        ]
    }

    fn rows(&self) -> Vec<Vec<ZkEntry>> {
        self.iter()
            .map(|row| {
                vec![
                    ZkEntry::MaybeAddress(row._id.clone().map(|it| it.oid), AddressStyle::Dashboard, AddressKind::Task),
                    ZkEntry::Raw(serde_to_string(&row.status).ok().unwrap_or_na()),
                    ZkEntry::Timestamp(row.batch_started.clone(), TimestampStyle::Full),
                    ZkEntry::Raw(row.auto_submit_network_chain_id.to_string()),
                    ZkEntry::Timestamp(row.batch_finished.clone(), TimestampStyle::Full),
                ]
            })
            .collect()
    }
}

impl PaginatedTableLike for Vec<Round1Info> {
    fn n_per_paginated() -> u64 {
        5
    }

    fn query_function() -> fn(page: u64, per: u64) -> Self::Fut {
        fn fetch(page: u64, per: u64) -> Pin<Box<dyn Future<Output = PaginationResult<Vec<Round1Info>>>>> {
            Box::pin(async move {
                ZKH.query_round1_info(None, None, None, None, None, None, Some(page), Some(per))
                    .await
                    .unwrap_or_empty()
            })
        }
        fetch
    }
}

impl TableLike for Vec<Round2Info> {
    fn title(&self) -> &str {
        "Round 2 Proof History"
    }

    fn headers(&self) -> Vec<&str> {
        vec!["Round 2 Proof ID", "Batch Finished At", "Aggregator Verifier"]
    }

    fn rows(&self) -> Vec<Vec<ZkEntry>> {
        self.iter()
            .map(|row| {
                vec![
                    ZkEntry::MaybeAddress(row._id.clone().map(|it| it.oid), AddressStyle::Dashboard, AddressKind::Task),
                    ZkEntry::Timestamp(row.batched_time.clone(), TimestampStyle::Full),
                    ZkEntry::Raw(row.registered_tx_hash.clone().unwrap_or_na()),
                ]
            })
            .collect()
    }
}

impl PaginatedTableLike for Vec<Round2Info> {
    fn n_per_paginated() -> u64 {
        5
    }

    fn query_function() -> fn(page: u64, per: u64) -> Self::Fut {
        fn fetch(page: u64, per: u64) -> Pin<Box<dyn Future<Output = PaginationResult<Vec<Round2Info>>>>> {
            Box::pin(async move {
                ZKH.query_round2_info(None, None, None, None, None, Some(page), Some(per))
                    .await
                    .unwrap_or_empty()
            })
        }
        fetch
    }
}

#[component]
pub fn TaskTables() -> Element {
    rsx! {
        PaginatedTable::<Vec<ProverNode>> {}
        PaginatedTable::<Vec<ConciseTask>> {}
        PaginatedTable::<Vec<AutoSubmitProof>> {}
        PaginatedTable::<Vec<Round1Info>> {}
        PaginatedTable::<Vec<Round2Info>> {}
    }
}
