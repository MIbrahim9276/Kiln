use clap::Args;

use super::shared::{
    FormatArg
};


#[derive(Args, Debug, Clone)]
#[command(about = "Shows project information")]
pub struct InfoArgs {
    #[command(flatten)]
    pub format: FormatArg
}