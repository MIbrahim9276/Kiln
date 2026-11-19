use clap::Args;

use super::shared::{
    TargetArgVec,
    ProfileArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Resolve profile, platform overlays, target graph, selected toolchains, and provider capabilities")]
pub struct ConfigureArgs {
    #[command(flatten)]
    pub target: TargetArgVec,

    #[command(flatten)]
    pub profile: ProfileArg
}