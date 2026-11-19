use clap::{Args, Subcommand};

use super::shared::FormatArg;


#[derive(Args, Debug, Clone)]
#[command(about = "Inspect discovered toolchain information")]
pub struct ToolchainArgs {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Args, Debug, Clone)]
#[command(about = "List discovered toolchains")]
pub struct ListArgs {}

#[derive(Args, Debug, Clone)]
#[command(about = "Show information about toolchain")]
pub struct ShowArgs {
    #[arg(value_name = "NAME")]
    pub name: String,

    #[command(flatten)]
    pub format: FormatArg
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    List(ListArgs),
    Show(ShowArgs)
}