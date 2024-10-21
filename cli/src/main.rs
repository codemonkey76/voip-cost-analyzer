use args::Args;
use clap::Parser;
use core::{
    buroserv_cdr::BuroservCdr,
    cdr::Cdr,
    error::{AppError, AppResult},
    filter_buroserv_cdrs,
};
mod args;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> AppResult<()> {
    let args = Args::parse();
    println!("FusionPBX File: {}", args.fusion_pbx_file);
    println!("ServiceProvider File: {}", args.service_provider_file);

    let cdrs = Cdr::from_file(&args.fusion_pbx_file)?;
    println!("Imported {} CDR's from FusionPBX CSV File", cdrs.len());
    let buroserv_cdrs = BuroservCdr::from_file(&args.service_provider_file)?;
    println!(
        "Imported {} CDR's from Buroserv Spreadsheet",
        buroserv_cdrs.len()
    );
    let filtered = filter_buroserv_cdrs(cdrs, buroserv_cdrs)?;
    dbg!(&filtered);
    println!(
        "Filtered list down to {} CDR's that are common in both",
        filtered.len()
    );

    Ok(())
}
