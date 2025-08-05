use dioxus::prelude::*;
use zkp_service_helper::interface::ConciseTask;

use crate::utils::shorten_addresss;
use crate::utils::shorten_md5;
use crate::utils::timestamp_formatted;
use crate::utils::HeaderType;

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
            class: "setup-entry",
            div {
                style: "flex-direction: column; gap: 4px;",
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
                style: "text-align: right; flex-direction: column; gap: 4px;",
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
            {entries
                .iter()
                .map(|(md5, task_id, address, date)| { rsx!( SummaryEntry {md5, task_id, address, date} )})
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    pub header: String,
    pub header_class: Option<String>,
    pub body: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "card",
            h1 {
                class: "card-header",
                {props.header}
            }
            div {
                class: "card-body",
                {props.body}
            }
        }
    }
}

#[component]
pub fn PairCardsAdjacent(left: Vec<ConciseTask>, right: Vec<ConciseTask>) -> Element {
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
            style: "padding: 1rem; display: flex; flex-direction: row;",
            div {
                style: "flex: 1; padding: 5px;",
                Card {
                    header: "Latest Setups",
                    header_class: "aqua",
                    body: rsx! { SummaryView { entries: lefts } }
                }
            }
            div {
                style: "flex: 1; padding: 5px;",
                Card {
                    header: "Latest Proofs",
                    header_class: "light-blue",
                    body: rsx! { SummaryView { entries: rights } }
                }
            }
        }
    }
}


pub trait DetailedCardLike {
    fn title(&self) -> String;
    fn headers(&self) -> Vec<HeaderType>;
    fn rows(&self) -> Vec<Vec<String>>;
}

#[component]
pub fn DetailedCard<T: DetailedCardLike + PartialEq + Clone + 'static>(data: T) -> Element {

    todo!()

}
