use dioxus::prelude::*;
use zkp_service_helper::interface::TaskStatus;
use zkp_service_helper::interface::TaskType;

use super::TaskSummary;
use super::TaskTables;
use crate::components::search::Search;
use crate::components::search::SearchSelectLike;
use crate::utils::serde_to_string;
use crate::GLOBAL_PADDING;

#[component]
pub fn Dashboard() -> Element {
    let trigger = use_signal(|| false);
    let taskstatus = use_signal(|| Option::<TaskStatus>::None);
    let tasktype = use_signal(|| Option::<TaskType>::None);
    let query = use_signal(|| Option::<String>::None);

    rsx! {
        div { style: GLOBAL_PADDING,
            Search {
                title: "The ZKWASM Task Explorer",
                placeholder: "Enter an MD5 hash, 0x address, or task ID",
                input_handler: query,
                trigger_handler: trigger,
                sel1: tasktype,
                sel2: taskstatus,
            }
        }
        if trigger() {
            TaskTables { inps: (query(), tasktype(), taskstatus()) }
        } else {
            { tracing::info!("UnTriggered"); }
            TaskTables { inps: (None, None, None) }
        }
        TaskSummary {}
    }
}

impl SearchSelectLike for Signal<Option<TaskType>> {
    fn onchange(&mut self, evt: Event<FormData>) {
        self.set(match evt.value().as_str() {
            "Setup" => Some(TaskType::Setup),
            "Prove" => Some(TaskType::Prove),
            _ => None,
        })
    }

    fn options(&self) -> Vec<&str> {
        vec!["All", "Setup", "Prove", "Deploy"]
    }
}

impl SearchSelectLike for Signal<Option<TaskStatus>> {
    fn onchange(&mut self, evt: Event<FormData>) {
        self.set(match evt.value().as_str() {
            "Pending" => Some(TaskStatus::Pending),
            "Processing" => Some(TaskStatus::Processing),
            "Done" => Some(TaskStatus::Done),
            "Fail" => Some(TaskStatus::Fail),
            "Unprovable" => Some(TaskStatus::Unprovable),
            "DryRunFailed" => Some(TaskStatus::DryRunFailed),
            _ => None,
        })
    }

    fn options(&self) -> Vec<&str> {
        vec![
            "All",
            "Pending",
            "Processing",
            "Done",
            "Fail",
            "Unprovable",
            "DryRunFailed",
        ]
    }
}
