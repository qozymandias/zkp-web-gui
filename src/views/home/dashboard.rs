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
                input: query,
                trigger,
                sel1: tasktype,
                sel2: taskstatus,
            }
        }
        TaskSummary {}
        ProverTaskTables {}
        ConciseTaskTables { inputs }
        AutoSubmitTaskTables {}
    }
}

macro_rules! enum_string_conversions {
    () => {
        fn to_string(it: &Self) -> String {
            $crate::utils::enum_to_string(it)
        }

        fn from_string(it: String) -> Self {
            $crate::utils::enum_from_string(&it)
        }
    };
}

impl SearchSelectLike for TaskType {
    fn raw_options() -> Vec<Self> {
        vec![TaskType::Setup, TaskType::Reset, TaskType::Prove]
    }

    enum_string_conversions!();
}

impl SearchSelectLike for TaskStatus {
    fn raw_options() -> Vec<Self> {
        vec![
            TaskStatus::Pending,
            TaskStatus::Processing,
            TaskStatus::DryRunSuccess,
            TaskStatus::DryRunFailed,
            TaskStatus::Done,
            TaskStatus::Fail,
            TaskStatus::Unprovable,
            TaskStatus::Stale,
        ]
    }

    enum_string_conversions!();
}
