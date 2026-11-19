use clap::Args;

use super::shared::{
    ProfileArg,
    FormatArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Read artifact metadata")]
pub struct ArtifactsArgs {
    #[command(flatten)]
    pub profile: ProfileArg,

    #[command(flatten)]
    pub format: FormatArg
}