use std::fs::File;

use serde::Deserialize;

use crate::error::AppResult;

#[derive(Debug, Deserialize)]
pub struct Cdr {
    pub domain_uuid: String,
    pub sip_call_id: Option<String>,
    pub extension: Option<String>,
    pub start_stamp: String,
    pub end_stamp: String,
    pub start_date_formatted: String,
    pub start_time_formatted: String,
    pub start_epoch: String,
    pub hangup_cause: String,
    pub duration: u32,
    pub billmsec: u32,
    pub record_path: Option<String>,
    pub record_name: Option<String>,
    pub xml_cdr_uuid: String,
    pub bridge_uuid: Option<String>,
    pub direction: String,
    pub billsec: u32,
    pub caller_id_name: String,
    pub caller_id_number: String,
    pub caller_destination: String,
    pub source_number: Option<String>,
    pub destination_number: String,
    pub leg: String,
    pub cc_side: Option<String>,
    pub accountcode: Option<String>,
    pub answer_stamp: String,
    pub sip_hangup_disposition: Option<String>,
    pub pdd_ms: u32,
    pub rtp_autio_in_mos: Option<f32>,
    pub tta: i32,
}

impl Cdr {
    pub fn from_file(file_path: &str) -> AppResult<Vec<Self>> {
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let record: Cdr = result?;
            records.push(record);
        }

        Ok(records)
    }
}
