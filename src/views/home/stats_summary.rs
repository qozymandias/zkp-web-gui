use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaClock;
use dioxus_free_icons::icons::io_icons::IoHardwareChipSharp;
use dioxus_free_icons::icons::io_icons::IoList;
use dioxus_free_icons::icons::ld_icons::LdBinary;
use dioxus_free_icons::Icon;
use zkp_service_helper::interface::StatisticsInfo;

use crate::components::card::CardWithIcon;
use crate::components::card::SummaryCardLike;
use crate::utils::ZkEntry;
use crate::GLOBAL_PADDING;

impl SummaryCardLike for Option<StatisticsInfo> {
    type T = ZkEntry;

    fn entries(self) -> Vec<(Self::T, Self::T, Self::T, Self::T)> {
        self.map(|it| {
            vec![(
                ZkEntry::Raw(format!("{} applications", it.total_images)),
                ZkEntry::Raw(format!("{} tasks", it.total_tasks)),
                ZkEntry::Raw(format!("{} proofs", it.total_proofs)),
                ZkEntry::Raw(format!("{} seconds", 40)),
            )]
        })
        .unwrap_or_default()
    }
}

#[component]
pub fn StatsSummary<U: SummaryCardLike + Clone + PartialEq + 'static>(data: U) -> Element {
    let size = 32;
    let es = data.entries();
    let Some(entries) = es.first() else {
        return rsx! {};
    };

    rsx! {
        div { style: GLOBAL_PADDING,
            div { class: "stats-container",
                CardWithIcon {
                    title: "Applications in total",
                    text: entries.0.clone(),
                    icon: rsx! {
                        Icon { icon: IoHardwareChipSharp, width: size, height: size }
                    },
                }
                CardWithIcon {
                    title: "Total Tasks Submitted",
                    text: entries.1.clone(),
                    icon: rsx! {
                        Icon { icon: IoList, width: size, height: size }
                    },
                }
                CardWithIcon {
                    title: "Proofs generated in total",
                    text: entries.2.clone(),
                    icon: rsx! {
                        Icon { icon: LdBinary, width: size, height: size }
                    },
                }
                CardWithIcon {
                    title: "Average Proving Time",
                    text: entries.3.clone(),
                    icon: rsx! {
                        Icon { icon: FaClock, width: size, height: size }
                    },
                }
            }
        }
    }
}
