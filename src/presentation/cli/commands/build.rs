use clap::Args;

use super::shared::{
    TargetArgVec,
    ProfileArg,
    JobsArg,
    FreshArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Resolves, validates, configures, and delegates compilation to backends")]
pub struct BuildArgs {
    #[command(flatten)]
    pub target: TargetArgVec,

    #[command(flatten)]
    pub profile: ProfileArg,

    #[command(flatten)]
    pub jobs: JobsArg,

    #[command(flatten)]
    pub fresh: FreshArg
}