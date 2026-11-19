use clap::Args;

use super::shared::{
    TargetArgVec,
    FormatWithDotArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Displays the target graph")]
pub struct GraphArgs {
    #[command(flatten)]
    pub target: TargetArgVec,

    #[command(flatten)]
    pub format: FormatWithDotArg
}