use dioxus::prelude::*;

use crate::components::dashboard::Dashboard;

#[component]
pub fn Home() -> Element {
    rsx! {
        Dashboard {}
    }
}
