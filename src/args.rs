use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(required=true)]
    pub files: Vec<String>,

    #[arg(long, default_value = "")]
    pub format: String,
}
