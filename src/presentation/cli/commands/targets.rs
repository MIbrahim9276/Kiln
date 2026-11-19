use clap::Args;

use super::shared::{
    FormatArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "List targets information")]
pub struct TargetsArgs {
    #[command(flatten)]
    pub format: FormatArg
}