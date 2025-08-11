use dioxus::prelude::*;
use zkp_service_helper::interface::Task;

use crate::components::card::EntirePageCard;
use crate::ZKH;

#[component]
pub fn TaskDetails(id: String) -> Element {
    tracing::info!("Task detail loading {id}");

    let mut task = use_signal(|| Option::<Task>::None);
    use_future(move || {
        let task_id = id.clone();
        async move {
            let result = ZKH.query_task_from_id(task_id).await.unwrap_or_default();
            task.set(result);
        }
    });

    rsx! {
        EntirePageCard { data: task() }
    }
}
