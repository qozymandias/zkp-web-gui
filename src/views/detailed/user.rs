use dioxus::prelude::*;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::Subscription;
use zkp_service_helper::interface::User;

use crate::components::card::EntryListCard;
use crate::components::card::EntryListLike;
use crate::components::table::Table;
use crate::utils::enum_to_string;
use crate::utils::hex_to_num_string;
use crate::utils::AddressKind;
use crate::utils::AddressStyle;
use crate::utils::UnwrapOrNA;
use crate::utils::ZkEntry;
use crate::GLOBAL_PADDING;
use crate::ZKH;

#[derive(Clone, PartialEq)]
struct DetailedUser {
    user: Option<User>,
    subscription: Option<Subscription>,
    tasks_submitted: Option<u64>,
}

impl EntryListLike for DetailedUser {
    type T = ZkEntry;

    fn title(&self) -> String {
        "User Details".to_string()
    }

    fn entries(&self) -> Vec<(&str, Self::T)> {
        let Some(user) = &self.user else {
            return vec![];
        };
        vec![
            (
                "Address",
                ZkEntry::Address(user.user_address.clone(), AddressStyle::Detailed, AddressKind::User),
            ),
            (
                "Balance",
                ZkEntry::Raw(
                    hex_to_num_string(&user.credits)
                        .map(|it| format!("{it} credits"))
                        .unwrap_or_na(),
                ),
            ),
            (
                "Current Subscription",
                ZkEntry::Raw(
                    self.subscription
                        .as_ref()
                        .map(|it| enum_to_string(&it.status))
                        .unwrap_or("None".to_string()),
                ),
            ),
            (
                "Total Tasks Submitted",
                ZkEntry::Raw(self.tasks_submitted.map(|it| it.to_string()).unwrap_or_na()),
            ),
        ]
    }
}

#[component]
pub fn UserDetails(id: String) -> Element {
    tracing::info!("User detail loading {id}");

    let id_for_user = id.clone();
    let mut user = use_signal(|| Option::<User>::None);
    use_future(move || {
        let address = id_for_user.clone();
        async move {
            user.set(
                ZKH.query_user(address)
                    .await
                    .inspect_err(|e| tracing::error!("{e}"))
                    .ok()
                    .flatten(),
            );
        }
    });

    let id_for_sub = id.clone();
    let mut sub = use_signal(|| Option::<Subscription>::None);
    use_future(move || {
        let address = id_for_sub.clone();
        async move {
            sub.set(
                ZKH.query_user_subscription(address)
                    .await
                    .inspect_err(|e| tracing::error!("{e}"))
                    .ok()
                    .flatten(),
            );
        }
    });

    let id_for_tasks = id.clone();
    let mut tasks = use_signal(Vec::<ConciseTask>::new);
    let mut tasks_submitted = use_signal(|| Option::<u64>::None);
    use_future(move || {
        let address = id_for_tasks.clone();
        async move {
            let res = ZKH.query_concise_tasks(Some(address), None, None, None, None, None, None).await;
            tasks_submitted.set(res.as_ref().map(|res| res.total).ok());
            tasks.set(res.map(|res| res.data).unwrap_or(vec![]));
        }
    });

    rsx! {
        div { style: GLOBAL_PADDING,
            div { id: "detail-header",
                div { "User" }
                div { id: "right-div", "{id}" }
            }
        }
        div { class: "node-details-wrapper",
            EntryListCard {
                data: DetailedUser {
                    user: user(),
                    subscription: sub(),
                    tasks_submitted: tasks_submitted(),
                },
                card_class: "transparent-border",
                header_class: "node-details-header",
                lcol_class: "node-details-col",
            }
        }
        Table { data: tasks() }
    }
}
