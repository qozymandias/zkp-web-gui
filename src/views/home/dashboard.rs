use dioxus::prelude::*;
use zkp_service_helper::interface::TaskStatus;
use zkp_service_helper::interface::TaskType;

use super::AutoSubmitTaskTables;
use super::ConciseTaskTables;
use super::ProverTaskTables;
use super::TaskSummary;
use crate::components::search::Search;
use crate::components::search::SearchSelectLike;
use crate::GLOBAL_PADDING;

#[component]
pub fn Dashboard() -> Element {
    let query = use_signal(|| Option::<String>::None);
    let tasktype = use_signal(|| Option::<TaskType>::None);
    let taskstatus = use_signal(|| Option::<TaskStatus>::None);
    let trigger = use_signal(|| false);

    let inputs = use_memo(move || {
        if trigger() {
            Some((query(), tasktype(), taskstatus()))
        } else {
            None
        }
    });

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
        ConciseTaskTables { inputs }
        TaskSummary {}
        ProverTaskTables {}
        AutoSubmitTaskTables {}
    }
}

impl SearchSelectLike for Option<TaskType> {
    fn onchange(signal: &mut Signal<Self>, evt: Event<FormData>) {
        signal.set(match evt.value().as_str() {
            "Setup" => Some(TaskType::Setup),
            "Reset" => Some(TaskType::Reset),
            "Prove" => Some(TaskType::Prove),
            _ => None,
        })
    }

    fn reset_to_default(signal: &mut Signal<Self>)
    where
        Self: Sized,
    {
        signal.set(None);
    }

    fn options(&self) -> Vec<&str> {
        vec!["All", "Setup", "Reset", "Prove"]
    }

    fn is_some(&self) -> bool {
        self.is_some()
    }

    fn read(&self) -> &str {
        self.options()[match self {
            None => 0,
            Some(it) => match it {
                TaskType::Setup => 1,
                TaskType::Reset => 2,
                TaskType::Prove => 3,
            },
        }]
    }
}

impl SearchSelectLike for Option<TaskStatus> {
    fn onchange(signal: &mut Signal<Self>, evt: Event<FormData>) {
        signal.set(match evt.value().as_str() {
            "Pending" => Some(TaskStatus::Pending),
            "Processing" => Some(TaskStatus::Processing),
            "Done" => Some(TaskStatus::Done),
            "Fail" => Some(TaskStatus::Fail),
            "Unprovable" => Some(TaskStatus::Unprovable),
            "DryRunFailed" => Some(TaskStatus::DryRunFailed),
            _ => None,
        })
    }

    fn reset_to_default(signal: &mut Signal<Self>)
    where
        Self: Sized,
    {
        signal.set(None);
    }

    fn options(&self) -> Vec<&str> {
        vec![
            "All",
            "Pending",
            "Processing",
            "DryRunSuccess",
            "DryRunFailed",
            "Done",
            "Fail",
            "Unprovable",
            "Stale",
        ]
    }

    fn is_some(&self) -> bool {
        self.is_some()
    }

    fn read(&self) -> &str {
        self.options()[match self {
            None => 0,
            Some(it) => match it {
                TaskStatus::Pending => 1,
                TaskStatus::Processing => 2,
                TaskStatus::DryRunSuccess => 3,
                TaskStatus::DryRunFailed => 4,
                TaskStatus::Done => 5,
                TaskStatus::Fail => 6,
                TaskStatus::Unprovable => 7,
                TaskStatus::Stale => 8,
            },
        }]
    }
}
