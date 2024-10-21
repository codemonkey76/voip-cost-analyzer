use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File from service provider
    #[arg(long)]
    pub service_provider_file: String,

    /// File from FusionPBX
    #[arg(long)]
    pub fusion_pbx_file: String,
}
