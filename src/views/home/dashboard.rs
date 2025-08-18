use dioxus::prelude::*;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::TaskType;

use super::TaskTables;
use crate::components::card::Card;
use crate::components::card::EntryLike;
use crate::utils::AddressKind;
use crate::utils::AddressStyle;
use crate::utils::TimestampStyle;
use crate::utils::ZkEntry;
use crate::ZKH;

fn into_summary_card(header: &str, header_class: &str, inp: Vec<ConciseTask>) -> Element {
    let entries = inp
        .into_iter()
        .map(|it| {
            (
                ZkEntry::Address(it.md5, AddressStyle::Detailed, AddressKind::PrefixedImage),
                ZkEntry::Address(it._id.oid, AddressStyle::Dashboard, AddressKind::PrefixedTask),
                ZkEntry::Address(it.user_address, AddressStyle::Dashboard, AddressKind::User),
                ZkEntry::Timestamp(Some(it.submit_time), TimestampStyle::Simple),
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        div { style: "padding: 0rem 1rem;",
            Card {
                header,
                header_class,
                body: rsx! {
                    div {
                        {
                            entries
                                .into_iter()
                                .map(|(lt, lb, rt, rb)| {
                                    rsx! {
                                        div { class: "detailed-entry",
                                            div {
                                                {lt.into_cell()}
                                                {lb.into_cell()}
                                            }
                                            div { style: "text-align: right",
                                                {rt.into_cell()}
                                                {rb.into_cell()}
                                            }
                                        }
                                    }
                                })
                        }
                    }
                },
            }
        }
    }
}

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

    rsx! {
        div { id: "adjacent-task-summaries",
            {into_summary_card("Latest Setups", "aqua", setups())}
            {into_summary_card("Latest Proofs", "light-blue", proves())}
        }
        TaskTables {}
    }
}
