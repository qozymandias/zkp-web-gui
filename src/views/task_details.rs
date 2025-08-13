use dioxus::prelude::*;
use zkp_service_helper::interface::Task;

use crate::components::card::EntryListCard;
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

    let left = format!(
        "Task ID {}",
        task().as_ref().map(|task| task._id.oid.clone()).unwrap_or("NA".to_string())
    );
    let right = task()
        .as_ref()
        .and_then(|task| task.node_address.clone())
        .unwrap_or("NA".to_string());

    rsx! {
        div {
            style: "padding: 2rem;",
            div {
                id: "detail-header",
                div {
                    "{left}"
                }
                div {
                    id: "right-div",
                    "{right}"
                }
            },
        }
        EntryListCard { data: task(), lcol_class: "task-details-col" }
    }
}
