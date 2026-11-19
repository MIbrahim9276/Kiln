use clap::Args;

use super::shared::{
    TargetArgVec,
    ProfileArg,
    FormatArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Produces a preview of selected targets, dependency closure, provider assignments, required toolchains, effective profile, and planned stages")]
pub struct PlanArgs {
    #[command(flatten)]
    pub target: TargetArgVec,

    #[command(flatten)]
    pub profile: ProfileArg,

    #[command(flatten)]
    pub format: FormatArg
}