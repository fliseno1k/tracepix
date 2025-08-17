use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tracepix", version, about, long_about = None)]
pub struct TracepixCliArguments {
    pub reference_img: PathBuf,

    pub target_img: PathBuf,

    #[arg(short, long)]
    pub otput_path: Option<PathBuf>,
}
