use clap::Args;

use super::shared::{
    TargetArg,
    ProfileArg,
    NoBuildArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Resolve the executable target, build it, and execute the resulting program")]
pub struct RunArgs {
    #[command(flatten)]
    pub target: TargetArg,

    #[command(flatten)]
    pub profile: ProfileArg,

    #[command(flatten)]
    pub no_build: NoBuildArg,

    #[arg(last = true, value_name = "APP ARGS", help = "Arguments passed to app")]
    pub args: Vec<String>
}