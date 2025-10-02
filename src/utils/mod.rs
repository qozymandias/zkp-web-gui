use zkp_service_helper::interface::PaginationResult;
use zkp_service_helper::interface::TaskStatus;

pub mod config;
mod entry;
pub use entry::AddressKind;
pub use entry::AddressStyle;
pub use entry::ZkEntry;

mod signal;
pub use signal::QueryFunctionHandler;

pub trait UnwrapOrNA {
    fn unwrap_or_na(&self) -> String;
}

impl UnwrapOrNA for Option<String> {
    fn unwrap_or_na(&self) -> String {
        self.clone().unwrap_or("N/A".to_string())
    }
}

pub trait UnwrapOrEmpty<T: serde::Serialize> {
    fn unwrap_or_empty(self) -> PaginationResult<Vec<T>>;
}

impl<T: serde::Serialize> UnwrapOrEmpty<T> for anyhow::Result<PaginationResult<Vec<T>>> {
    fn unwrap_or_empty(self) -> PaginationResult<Vec<T>> {
        self.unwrap_or_else(|e| {
            tracing::error!("{e}");
            PaginationResult { data: Vec::<T>::new(), total: 0 }
        })
    }
}

pub fn shorten_md5(it: String) -> String {
    let l = it.len();
    format!("{}...{}", &it[0..7], &it[l - 6..l])
}

pub fn shorten_address(it: String) -> String {
    let l = it.len();
    format!("{}...{}", &it[0..8], &it[l - 4..l])
}

#[derive(Clone, PartialEq)]
pub enum TimestampStyle {
    Simple,
    Full,
}

pub fn timestamp_formatted(ts_str: &str, style: TimestampStyle) -> String {
    let ts = ts_str
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap()
        .with_timezone(&chrono::Local);
    ts.format(match style {
        TimestampStyle::Simple => {
            if ts.date_naive() == chrono::Local::now().date_naive() {
                "%-I:%M:%S %P"
            } else {
                "%d/%m/%Y"
            }
        }
        TimestampStyle::Full => "%d/%m/%Y, %-I:%M:%S %P",
    })
    .to_string()
}

pub fn webtime_to_rfc3339(time: web_time::SystemTime) -> String {
    fn convert(ts: web_time::SystemTime) -> anyhow::Result<String> {
        let dur = ts.duration_since(web_time::UNIX_EPOCH)?;
        let secs = dur.as_secs().try_into()?;
        let nanos = dur.subsec_nanos();
        let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos)
            .ok_or_else(|| anyhow::anyhow!("Invalid secs {secs} or nanos {nanos}"))?;
        Ok(dt.to_rfc3339())
    }
    convert(time).inspect_err(|e| tracing::error!("{e}")).ok().unwrap_or_na()
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

pub fn hex_to_num_string(inp: &str) -> Option<String> {
    use num_traits::Num;
    inp.strip_prefix("0x").and_then(|data| {
        num_bigint::BigUint::from_str_radix(data, 16)
            .map(|it| it.to_string())
            .inspect_err(|e| tracing::error!("{e}"))
            .ok()
    })
}

pub fn task_status_to_background_color(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Pending => "#CA9B00",
        TaskStatus::Processing => "#CA9B00",
        TaskStatus::DryRunSuccess => "#CA9B00",
        TaskStatus::DryRunFailed => "#CA9B00",
        TaskStatus::Done => "#3E8166",
        TaskStatus::Fail => "#DD6B00",
        TaskStatus::Unprovable => "#894E50",
        TaskStatus::Stale => "#636363",
    }
}

pub fn enum_to_string<T: serde::Serialize>(obj: &T) -> String {
    serde_json::to_string(obj)
        .expect("Enum object should Serialize")
        .trim_start_matches('"')
        .trim_end_matches('"')
        .to_string()
}

pub fn enum_from_string<T: serde::de::DeserializeOwned>(str: &str) -> T {
    serde_json::from_str::<T>(&format!("\"{str}\"")).expect("Enum string should Deserialize")
}
