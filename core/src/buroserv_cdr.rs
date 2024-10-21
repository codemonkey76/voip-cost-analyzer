use calamine::{open_workbook, DataType, Reader, Xls, Xlsx};
use serde::Deserialize;

use crate::error::AppResult;

#[derive(Debug, Deserialize)]
pub struct BuroservCdr {
    pub cost_centre: String,
    pub service_number: String,
    pub service_narrative: Option<String>,
    pub service_type: Option<String>,
    pub date: String,
    pub time: String,
    pub number_called: String,
    pub quantity: String,
    pub rate_period: String,
    pub price: f32,
    pub price2: f32,
    pub tax_free: String,
    pub usage_type: String,
    pub usage_description: String,
}

impl BuroservCdr {
    pub fn from_file(file_path: &str) -> AppResult<Vec<BuroservCdr>> {
        let mut workbook: Xlsx<_> = open_workbook(file_path)?;
        let mut records = Vec::new();

        let range = workbook.worksheet_range("Usage Details")?;

        for row in range.rows().skip(1) {
            let cdr = BuroservCdr {
                cost_centre: row[0].get_string().unwrap_or("").to_string(),
                service_number: row[1].get_string().unwrap_or("").to_string(),
                service_narrative: row[2].get_string().map(|s| s.to_string()),
                service_type: row[3].get_string().map(|s| s.to_string()),
                date: row[4].get_string().unwrap_or("").to_string(),
                time: row[5].get_string().unwrap_or("").to_string(),
                number_called: row[6].get_string().unwrap_or("").to_string(),
                quantity: row[7].get_string().unwrap_or("").to_string(),
                rate_period: row[8].get_string().unwrap_or("").to_string(),
                price: row[9].get_float().unwrap_or(0.0) as f32,
                price2: row[10].get_float().unwrap_or(0.0) as f32,
                tax_free: row[11].get_string().unwrap_or("").to_string(),
                usage_type: row[12].get_string().unwrap_or("").to_string(),
                usage_description: row[13].get_string().unwrap_or("").to_string(),
            };
            records.push(cdr);
        }
        Ok(records)
    }
}
