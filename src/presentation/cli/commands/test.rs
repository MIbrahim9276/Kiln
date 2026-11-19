use clap::Args;

use super::shared::{
    ProfileArg,
    JobsArg,
    NoBuildArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Builds the executable targets and runs declared tests")]
pub struct TestArgs {
    #[command(flatten)]
    pub profile: ProfileArg,

    #[command(flatten)]
    pub jobs: JobsArg,

    #[command(flatten)]
    pub no_build: NoBuildArg
}