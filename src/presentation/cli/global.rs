use clap::{
    Args,
    ArgAction,
    value_parser
};


#[derive(Args, Debug, Clone)]
pub struct GlobalOpts {
    #[arg(long, value_name = "Path", help = "Explicit kiln.toml location")]
    pub manifest_path: Option<String>,

    #[arg(short, long, action = ArgAction::Count, value_parser = value_parser!(u8).range(0..=2), help = "More diagnostic detail")]
    pub verbose: u8,

    #[arg(short, long, help ="Hide routine progress")]
    pub quiet: bool
}