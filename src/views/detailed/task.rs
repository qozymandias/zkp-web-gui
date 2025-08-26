use dioxus::prelude::*;
use zkp_service_helper::interface::ProofSubmitMode;
use zkp_service_helper::interface::Task;

use crate::components::card::EntryListCard;
use crate::components::card::EntryListLike;
use crate::utils::bytes_to_num_string;
use crate::utils::calc_processing_time_secs;
use crate::utils::serde_to_string;
use crate::utils::AddressKind;
use crate::utils::AddressStyle;
use crate::utils::TimestampStyle;
use crate::utils::UnwrapOrNA;
use crate::utils::ZkEntry;
use crate::GLOBAL_PADDING;
use crate::ZKH;

impl EntryListLike for Option<Task> {
    type T = ZkEntry;

    fn title(&self) -> String {
        "Task Overview".to_string()
    }

    fn entries(&self) -> Vec<(&str, ZkEntry)> {
        self.as_ref()
            .map(|it| {
                vec![
                    (
                        "Application",
                        ZkEntry::Address(it.md5.clone(), AddressStyle::Detailed, AddressKind::Image),
                    ),
                    ("Type", ZkEntry::TaskType(it.task_type.clone())),
                    ("Status", ZkEntry::TaskStatus(it.status.clone())),
                    (
                        "Submitted at",
                        ZkEntry::Timestamp(Some(it.submit_time.clone()), TimestampStyle::Full),
                    ),
                    (
                        "Submitted by",
                        ZkEntry::Address(it.user_address.clone(), AddressStyle::Detailed, AddressKind::User),
                    ),
                    (
                        "Task taken by Node",
                        ZkEntry::MaybeAddress(it.node_address.clone(), AddressStyle::Detailed, AddressKind::Node),
                    ),
                    (
                        "Processing Started",
                        ZkEntry::Timestamp(it.process_started.clone(), TimestampStyle::Full),
                    ),
                    (
                        "Processing Finished",
                        ZkEntry::Timestamp(it.process_finished.clone(), TimestampStyle::Full),
                    ),
                    (
                        "Processing Time",
                        ZkEntry::Raw(
                            calc_processing_time_secs(it.process_started.clone(), it.process_finished.clone())
                                .map(|dur| format!("{dur} seconds"))
                                .unwrap_or("NA".to_string()),
                        ),
                    ),
                    (
                        "Task Fee",
                        ZkEntry::Raw(bytes_to_num_string(it.task_fee.clone()).unwrap_or_na()),
                    ),
                    ("Debug Logs", ZkEntry::Logs(it.debug_logs.clone())),
                    (
                        "Guest Statics",
                        ZkEntry::Raw(it.guest_statics.map(|x| x.to_string()).unwrap_or_na()),
                    ),
                    (
                        "Proof Submit Mode",
                        ZkEntry::Raw(
                            serde_to_string(it.proof_submit_mode.as_ref().unwrap_or(&ProofSubmitMode::Manual))
                                .ok()
                                .unwrap_or_na(),
                        ),
                    ),
                    (
                        "Current Batch Status",
                        ZkEntry::Raw(
                            it.auto_submit_status
                                .as_ref()
                                .and_then(|it| serde_to_string(it).ok())
                                .unwrap_or_na(),
                        ),
                    ),
                    ("Public Inputs", ZkEntry::LongInput(it.public_inputs.clone())),
                    ("Witness", ZkEntry::LongInput(it.private_inputs.clone())),
                    ("External Host Table", ZkEntry::DownloadButton(it._id.oid.clone())),
                    ("Input Context", ZkEntry::Bytes(it.input_context.clone(), Some(8))),
                    ("Context Output", ZkEntry::Bytes(it.output_context.clone(), Some(8))),
                    ("Single Proof Transcripts", ZkEntry::Bytes(it.single_proof.clone(), None)),
                    ("Instances", ZkEntry::Bytes(it.instances.clone(), None)),
                    ("Batched Proof Transcripts", ZkEntry::Bytes(it.proof.clone(), None)),
                    ("Shadow Instances", ZkEntry::Bytes(it.shadow_instances.clone(), None)),
                    ("Batch Instances", ZkEntry::Bytes(it.batch_instances.clone(), None)),
                    ("Aux Data", ZkEntry::Bytes(it.aux.clone(), None)),
                ]
            })
            .unwrap_or_default()
    }
}

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
        task().as_ref().map(|task| task._id.oid.clone()).unwrap_or("NA".to_string()),
    );
    let right = task()
        .as_ref()
        .and_then(|task| task.node_address.clone())
        .unwrap_or("NA".to_string());
    rsx! {
        div { style: GLOBAL_PADDING,
            div { id: "detail-header",
                div { "{left}" }
                div { id: "right-div", "{right}" }
            }
        }
        EntryListCard { data: task(), lcol_class: "task-details-col" }
    }
}
