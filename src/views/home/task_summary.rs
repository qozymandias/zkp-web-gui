use dioxus::prelude::*;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::StatisticsInfo;
use zkp_service_helper::interface::TaskType;

use crate::components::card::SummaryCard;
use crate::components::card::SummaryCardLike;
use crate::utils::AddressKind;
use crate::utils::AddressStyle;
use crate::utils::QueryFunctionHandler;
use crate::utils::TimestampStyle;
use crate::utils::ZkEntry;
use crate::views::home::StatsSummary;
use crate::ZKH;

impl QueryFunctionHandler for StatisticsInfo {
    type Data = Option<Self>;

    fn init_state() -> Self::Data {
        None
    }

    async fn query(_: ()) -> anyhow::Result<Self::Data> {
        ZKH.query_statistics().await.map(Some)
    }
}

#[derive(Clone, PartialEq)]
struct ConciseTaskSummary {
    data: Vec<ConciseTask>,
}

impl QueryFunctionHandler for ConciseTaskSummary {
    type Input = TaskType;
    type Data = Vec<ConciseTask>;

    fn init_state() -> Self::Data {
        vec![]
    }

    async fn query(inp: TaskType) -> anyhow::Result<Self::Data> {
        ZKH.query_concise_tasks(None, None, None, Some(inp), None, None, Some(5))
            .await
            .map(|res| res.data)
    }
}

impl SummaryCardLike for ConciseTaskSummary {
    type T = ZkEntry;

    fn entries(self) -> Vec<(Self::T, Self::T, Self::T, Self::T)> {
        self.data
            .into_iter()
            .map(|it| {
                (
                    ZkEntry::Address(it.md5, AddressStyle::Detailed, AddressKind::PrefixedImage),
                    ZkEntry::Address(it._id.oid, AddressStyle::Dashboard, AddressKind::PrefixedTask),
                    ZkEntry::Address(it.user_address, AddressStyle::Dashboard, AddressKind::User),
                    ZkEntry::Timestamp(Some(it.submit_time), TimestampStyle::Simple),
                )
            })
            .collect()
    }
}

#[component]
pub fn TaskSummary() -> Element {
    let stats = StatisticsInfo::fetch_resource(());
    let setups = ConciseTaskSummary::fetch_resource(TaskType::Setup);
    let proves = ConciseTaskSummary::fetch_resource(TaskType::Prove);

    rsx! {
        StatsSummary { data: stats() }
        div { id: "adjacent-task-summaries", style: "padding: 0rem 4rem;",
            SummaryCard {
                data: ConciseTaskSummary {
                    data: setups(),
                },
                header: "Latest Setups",
                header_class: "aqua",
            }
            SummaryCard {
                data: ConciseTaskSummary {
                    data: proves(),
                },
                header: "Latest Proofs",
                header_class: "light-blue",
            }
        }
    }
}
