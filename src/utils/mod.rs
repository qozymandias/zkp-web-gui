pub mod table;

use serde::Serialize;
use table::CellStyle;
use zkp_service_helper::interface::TaskStatus;

pub fn timestamp_formatted(ts: &str) -> String {
    ts.parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap()
        .with_timezone(&chrono::Local)
        .format("%-I:%M:%S %P")
        .to_string()
}

pub fn link_formatted(link: &str, style: &CellStyle) -> String {
    match style {
        CellStyle::ShortLink => {
            let l = link.len();
            format!("{}...{}", &link[0..7], &link[l - 4..l])
        }
        _ => link.to_string(),
    }
}

pub fn link_color(style: &CellStyle) -> &str {
    match style {
        CellStyle::ImageLink => "white",
        _ => "#51BDFB",
    }
}

fn str_to_task_status(value: &str) -> TaskStatus {
    match value {
        "Pending" => TaskStatus::Pending,
        "Processing" => TaskStatus::Processing,
        "DryRunSuccess" => TaskStatus::DryRunSuccess,
        "DryRunFailed" => TaskStatus::DryRunFailed,
        "Done" => TaskStatus::Done,
        "Fail" => TaskStatus::Fail,
        "Unprovable" => TaskStatus::Unprovable,
        "Stale" => TaskStatus::Stale,
        _ => unreachable!("Invalid value {value}, cannot be converted to TaskStatus"),
    }
}

pub fn task_status_to_background_color(status: &str) -> &'static str {
    match str_to_task_status(status) {
        TaskStatus::Pending => "#CA9B00",
        TaskStatus::Processing => "#CA9B00",
        TaskStatus::DryRunSuccess => "#CA9B00",
        TaskStatus::DryRunFailed => "#DD6B00",
        TaskStatus::Done => "#3E8166",
        TaskStatus::Fail => "#2C3841",
        TaskStatus::Unprovable => "#894E50",
        TaskStatus::Stale => "#636363",
    }
}

pub fn serde_to_string<T: Serialize>(obj : &T) -> anyhow::Result<String> {
    Ok(match serde_json::to_value(obj)? {
        serde_json::Value::Bool(v) => v.to_string(),
        serde_json::Value::Number(v) => v.to_string(),
        serde_json::Value::String(v) => v,
        _ => return Err(anyhow::anyhow!("Must be primitive object type")),
    })
}
