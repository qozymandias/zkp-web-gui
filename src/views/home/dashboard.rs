use dioxus::prelude::*;

use super::TaskSummary;
use super::TaskTables;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        TaskSummary {}
        TaskTables {}
    }
}
