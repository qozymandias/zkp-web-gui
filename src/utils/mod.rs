use table::CellStyle;

pub mod request;
pub mod table;

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
