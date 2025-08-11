use dioxus::prelude::*;
use serde::Serialize;
use zkp_service_helper::interface::TaskStatus;

pub fn shorten_md5(it: String) -> String {
    let l = it.len();
    format!("{}...{}", &it[0..7], &it[l - 6..l])
}

pub fn shorten_addresss(it: String) -> String {
    let l = it.len();
    format!("{}...{}", &it[0..8], &it[l - 4..l])
}

pub fn timestamp_formatted(ts: &str) -> String {
    ts.parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap()
        .with_timezone(&chrono::Local)
        .format("%-I:%M:%S %P")
        .to_string()
}

pub fn calc_processing_time_secs(start_in: Option<String>, end_in: Option<String>) -> Option<f64> {
    start_in.zip(end_in).and_then(|(start, end)| {
        start
            .parse::<chrono::DateTime<chrono::Utc>>()
            .inspect_err(|e| tracing::error!("{e}"))
            .ok()
            .zip(
                end.parse::<chrono::DateTime<chrono::Utc>>()
                    .inspect_err(|e| tracing::error!("{e}"))
                    .ok(),
            )
            .and_then(|(s, e)| {
                let s_st: std::time::SystemTime = s.into();
                let e_st: std::time::SystemTime = e.into();

                e_st.duration_since(s_st)
                    .inspect_err(|e| tracing::error!("{e}"))
                    .ok()
                    .map(|x| x.as_secs_f64())
            })
    })
}

pub fn bytes_to_num_string(bytes: Option<Vec<u8>>) -> Option<String> {
    bytes.map(|b| num_bigint::BigUint::from_bytes_le(&b).to_string())
}

pub fn bytes_to_bigint(data: &[u8], chunksize: Option<usize>) -> Vec<num_bigint::BigUint> {
    data.chunks(chunksize.unwrap_or(32))
        .map(num_bigint::BigUint::from_bytes_le)
        .collect()
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

pub fn serde_to_string<T: Serialize>(obj: &T) -> anyhow::Result<String> {
    Ok(match serde_json::to_value(obj)? {
        serde_json::Value::Bool(v) => v.to_string(),
        serde_json::Value::Number(v) => v.to_string(),
        serde_json::Value::String(v) => v,
        _ => return Err(anyhow::anyhow!("Must be primitive object type")),
    })
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellStyle {
    TaskLink,
    ShortLink,
    ImageLink,
    Raw,
    Timestamp,
    RoundColoredBox,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeaderType {
    pub name: String,
    pub style: CellStyle,
}

impl Default for HeaderType {
    fn default() -> Self {
        HeaderType {
            name: "Unknown".to_string(),
            style: CellStyle::Raw,
        }
    }
}

impl HeaderType {
    pub fn make_cell(&self, cell: &str) -> Element {
        match self.style {
            CellStyle::TaskLink | CellStyle::ShortLink | CellStyle::ImageLink => rsx! {
                div {
                    id: "table-links",
                    Link {
                        color: link_color(&self.style),
                        to: crate::Route::TaskDetails { id : cell.to_string() },
                        { link_formatted(cell, &self.style) }
                    }
                }
            },
            CellStyle::Raw => rsx! {
                div {
                    text_align: "center",
                    "{cell}"
                }
            },
            CellStyle::Timestamp => rsx! {
                div {
                    text_align: "center",
                    { timestamp_formatted(cell) }
                }
            },
            CellStyle::RoundColoredBox => rsx! {
                div {
                    id: "status-rounded-box", background_color: task_status_to_background_color(cell),
                    "{cell}"
                }
            },
        }
    }

    pub fn get_header_and_make_cell(headers: &[HeaderType], i: usize, cell: &str) -> Element {
        headers
            .get(i)
            .cloned()
            .unwrap_or_else(|| {
                tracing::info!("Missing header\nCell is {cell:?}\nHeader is {headers:?}\nIndex {i}\n");
                HeaderType::default()
            })
            .make_cell(cell)
    }
}
