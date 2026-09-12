use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "yuntuns",
    version = "0.1.0",
    about = "Yunotools developer utilities"
)]
pub struct Args {
    #[arg(short = 'c', long = "copyast")]
    pub copyast: bool,

    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    #[arg(value_name = "OUTPUT")]
    pub output: Option<String>,
}
