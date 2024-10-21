use std::collections::HashSet;

use buroserv_cdr::BuroservCdr;
use cdr::Cdr;
use chrono::{NaiveDate, NaiveTime};
use error::AppResult;

pub mod buroserv_cdr;
pub mod cdr;
pub mod error;

pub fn filter_buroserv_cdrs(
    fusionpbx_cdrs: Vec<Cdr>,
    buroserv_cdrs: Vec<BuroservCdr>,
) -> AppResult<Vec<BuroservCdr>> {
    let fusionpbx_keys: HashSet<(String, String)> = fusionpbx_cdrs
        .iter()
        .map(|cdr| {
            let date_time_parts: Vec<&str> = cdr.end_stamp.split_whitespace().collect();
            let date = date_time_parts[0].to_string();
            let time = date_time_parts.get(1).unwrap_or(&"").to_string();
            (date, time)
        })
        .collect();

    let filtered_cdrs: Vec<BuroservCdr> = buroserv_cdrs
        .into_iter()
        .filter(|buroserv_cdr| {
            let date = normalize_date(&buroserv_cdr.date).unwrap_or_default();
            let time = convert_to_24h_time(&buroserv_cdr.time).unwrap_or_default();
            fusionpbx_keys.contains(&(date, time))
        })
        .collect();

    Ok(filtered_cdrs)
}

fn normalize_date(date_str: &str) -> AppResult<String> {
    let parsed_date = NaiveDate::parse_from_str(date_str, "%d/%m/%Y")?;
    Ok(parsed_date.format("%Y-%m-%d").to_string())
}

fn convert_to_24h_time(time_str: &str) -> AppResult<String> {
    let parsed_time = NaiveTime::parse_from_str(time_str, "%I:%M:%S %p")?;
    Ok(parsed_time.format("%H:%M:%S").to_string())
}
