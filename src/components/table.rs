use dioxus::prelude::*;
use zkp_service_helper::interface::AutoSubmitProof;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::ProverNode;
use zkp_service_helper::interface::Round1Info;
use zkp_service_helper::interface::Round2Info;
use zkp_service_helper::interface::Task;

use crate::utils::bytes_to_num_string;
use crate::utils::calc_processing_time_secs;
use crate::utils::serde_to_string;
use crate::utils::CellStyle;
use crate::utils::HeaderType;

pub trait TableLike {
    fn title(&self) -> String;
    fn headers(&self) -> Vec<HeaderType>;
    fn rows(&self) -> Vec<Vec<String>>;

    fn into_header_type(inp: Vec<(&str, CellStyle)>) -> Vec<HeaderType> {
        inp.into_iter()
            .map(|(nm, sty)| HeaderType {
                name: nm.to_string(),
                style: sty,
            })
            .collect()
    }
}

impl TableLike for Vec<ConciseTask> {
    fn title(&self) -> String {
        "Task History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Task Id", CellStyle::TaskLink),
            ("Application Image", CellStyle::ImageLink),
            ("Published By", CellStyle::ShortLink),
            ("Type", CellStyle::Raw),
            ("Submit At", CellStyle::Timestamp),
            ("Status", CellStyle::RoundColoredBox),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row._id.oid.clone(),
                    row.md5.clone(),
                    row.user_address.clone(),
                    serde_to_string(&row.task_type).unwrap_or("Unknown".to_string()),
                    row.submit_time.clone(),
                    serde_to_string(&row.status).unwrap_or("Unknown".to_string()),
                ]
            })
            .collect()
    }
}

// impl TableLike for Option<Task> {
//     fn title(&self) -> String {
//         "Task Overview".to_string()
//     }
// 
//     fn headers(&self) -> Vec<HeaderType> {
//         Self::into_header_type(vec![
//             ("Application", CellStyle::Raw),
//             ("Type", CellStyle::Raw),
//             ("Status", CellStyle::Raw),
//             ("Submitted at", CellStyle::Raw),
//             ("Submitted by", CellStyle::Raw),
//             ("Task taken by Node", CellStyle::Raw),
//             ("Processing Started", CellStyle::Raw),
//             ("Processing Finished", CellStyle::Raw),
//             ("Processing Time", CellStyle::Raw),
//             ("Task Fee", CellStyle::Raw),
//             ("Debug Logs", CellStyle::Raw),
//             ("Guest Statics", CellStyle::Raw),
//             ("Proof Submit Mode", CellStyle::Raw),
//             ("Current Batch Status", CellStyle::Raw),
//             ("Public Inputs", CellStyle::Raw),
//             ("Witness", CellStyle::Raw),
//             ("External Host Table", CellStyle::Raw),
//             ("Input Context", CellStyle::Raw),
//             ("Context Output", CellStyle::Raw),
//             ("Single Proof Transcripts", CellStyle::Raw),
//             ("Instances", CellStyle::Raw),
//             ("Batched Proof Transcripts", CellStyle::Raw),
//             ("Shadow Instances", CellStyle::Raw),
//             ("Batch Instances", CellStyle::Raw),
//             ("Aux Data", CellStyle::Raw),
//         ])
//     }
// 
//     fn rows(&self) -> Vec<Vec<String>> {
//         self.iter()
//             .map(|row| {
//                 vec![
//                     row.md5.clone(),
//                     serde_to_string(&row.task_type).unwrap_or("NA".to_string()),
//                     serde_to_string(&row.status).unwrap_or("NA".to_string()),
//                     row.submit_time.clone(),
//                     row.user_address.clone(),
//                     row.node_address.clone().unwrap_or("NA".to_string()),
//                     row.process_started.clone().unwrap_or("NA".to_string()),
//                     row.process_finished.clone().unwrap_or("NA".to_string()),
//                     calc_processing_time_secs(row.process_started.clone(), row.process_finished.clone())
//                         .map(|dur| format!("{dur} seconds"))
//                         .unwrap_or("NA".to_string()),
//                     bytes_to_num_string(row.task_fee.clone()).unwrap_or("NA".to_string()),
//                     row.debug_logs.clone().unwrap_or("NA".to_string()),
//                     row.guest_statics.map(|x| x.to_string()).unwrap_or("NA".to_string()),
//                     row.proof_submit_mode
//                         .clone()
//                         .and_then(|x| serde_to_string(&x).ok())
//                         .unwrap_or("NA".to_string()),
//                     row.auto_submit_status
//                         .clone()
//                         .and_then(|x| serde_to_string(&x).ok())
//                         .unwrap_or("NA".to_string()),
//                     row.public_inputs.clone().join("\n"),
//                     row.private_inputs.clone().join("\n"),
//                     "Download ... ".to_string(),
//                     hex::encode(row.input_context.clone()),
//                     hex::encode(row.output_context.clone()),
//                     hex::encode(row.single_proof.clone()),
//                     hex::encode(row.instances.clone()),
//                     hex::encode(row.proof.clone()),
//                     hex::encode(row.shadow_instances.clone()),
//                     hex::encode(row.batch_instances.clone()),
//                     hex::encode(row.aux.clone()),
//                 ]
//             })
//             .collect()
//     }
// }

impl TableLike for Vec<ProverNode> {
    fn title(&self) -> String {
        "Prover List".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Top Node Addresses", CellStyle::TaskLink),
            ("Successful Tasks", CellStyle::Raw),
            ("Failed Tasks", CellStyle::Raw),
            ("Total Tasks", CellStyle::Raw),
            ("Last Proof Time", CellStyle::Raw),
            ("Last Proof Timestamp", CellStyle::Timestamp),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row.address.clone(),
                    row.statistics.successful_tasks.to_string(),
                    row.statistics.failed_tasks.to_string(),
                    row.statistics.total_tasks.to_string(),
                    row.statistics.last_timed_out.clone().unwrap_or("NA".to_string()),
                    row.last_attempted_task
                        .as_ref()
                        .map(|t| t.timestamp.clone())
                        .unwrap_or("NA".to_string()),
                ]
            })
            .collect()
    }
}

impl TableLike for Vec<AutoSubmitProof> {
    fn title(&self) -> String {
        "Auto Submit Proof Task History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Proof Task ID", CellStyle::TaskLink),
            ("Batch Status", CellStyle::Raw),
            ("Target Proof Submitted", CellStyle::Timestamp),
            ("Network", CellStyle::Raw),
            ("Batch Finished", CellStyle::Raw),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row._id.clone().map(|it| it.oid).unwrap_or("NA".to_string()),
                    serde_to_string(&row.status).unwrap_or("NA".to_string()),
                    row.batch_started.clone().unwrap_or("Not Started".to_string()),
                    row.auto_submit_network_chain_id.to_string(),
                    row.batch_finished.clone().unwrap_or("Not Finished".to_string()),
                ]
            })
            .collect()
    }
}

impl TableLike for Vec<Round1Info> {
    fn title(&self) -> String {
        "Round 1 Proof History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        Self::into_header_type(vec![
            ("Round 1 Proof ID", CellStyle::TaskLink),
            ("Batch Status", CellStyle::Raw),
            ("Target Proof Submitted", CellStyle::Timestamp),
            ("Network", CellStyle::Raw),
            ("Batch Finished", CellStyle::Raw),
        ])
    }

    fn rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|row| {
                vec![
                    row._id.clone().map(|it| it.oid).unwrap_or("NA".to_string()),
                    serde_to_string(&row.status).unwrap_or("NA".to_string()),
                    row.batch_started.clone().unwrap_or("Not Started".to_string()),
                    row.auto_submit_network_chain_id.to_string(),
                    row.batch_finished.clone().unwrap_or("Not Finished".to_string()),
                ]
            })
            .collect()
    }
}

impl TableLike for Vec<Round2Info> {
    fn title(&self) -> String {
        "Round 2 Proof History".to_string()
    }

    fn headers(&self) -> Vec<HeaderType> {
        [
            ("Round 2 Proof ID", CellStyle::TaskLink),
            ("Batch Finished At", CellStyle::Raw),
            ("Aggregator Verifier ", CellStyle::Raw),
        ]
        .into_iter()
        .map(|(nm, sty)| HeaderType {
            name: nm.to_string(),
            style: sty,
        })
        .collect()
    }

    fn rows(&self) -> Vec<Vec<String>> {
        let mut out = vec![];
        for row in self {
            out.push(vec![
                row._id.clone().map(|it| it.oid).unwrap_or("NA".to_string()),
                row.batched_time.clone().unwrap_or("Not Finished".to_string()),
                row.registered_tx_hash.clone().unwrap_or("NA".to_string()),
            ]);
        }
        out
    }
}

pub fn into_table_header(headers: &[HeaderType]) -> Element {
    rsx! {
        tr { {
            headers.iter().map(|h| h.name.clone()).map(|name| rsx!{
                th {
                    id: "table-row",
                    "{name}"
                }
            })
        } }
    }
}

pub fn into_table_body(rows: Vec<Vec<String>>, headers: &[HeaderType]) -> Element {
    rsx! { {
        rows.iter().map(|row| rsx!{
            tr { {
                row.iter().enumerate().map(|(i, cell)| rsx!{
                    td {
                        id: "table-row",
                        { HeaderType::get_header_and_make_cell(headers, i, cell) }
                    }
                })
            } }
        })
    } }
}

#[component]
pub fn Table<T: TableLike + PartialEq + Clone + 'static>(data: T) -> Element {
    let title = data.title();
    let headers = data.headers();
    let rows = data.rows();
    // assert_eq!(headers.len(), rows.len());
    rsx! {
        div {
            style: "padding: 1rem;",
            h1 { "{title}" }
            table {
                style: "border-collapse: collapse; width: 100%;",
                thead { { into_table_header(&headers) } }
                tbody { { into_table_body(rows, &headers) } }
            }
        }
    }
}

#[component]
pub fn SidewaysTable<T: TableLike + PartialEq + Clone + 'static>(data: T) -> Element {
    let title = data.title();
    let headers = data.headers();
    let rows = data.rows();
    // assert_eq!(headers.len(), rows.len());
    // rsx! {
    //     div {
    //         style: "padding: 1rem;",
    //         h1 { "{title}" }
    //         div {
    //             style: "border-collapse: collapse; width: 100%;",
    //         }
    //     }
    // }
    rsx! {
        div {
            style: "display: flex;",
            div {
                id: "links",
                style: "flex: 1; padding: 1rem;",
                "Left column"

                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
            div {
                id: "links",
                style: "flex: 1; padding: 1rem;",
                "Right column"
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            }

        }
    }
}
