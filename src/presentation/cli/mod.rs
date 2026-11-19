use clap::Parser;


mod global;
mod commands;


use global::GlobalOpts;
use commands::Commands;


#[derive(Parser, Debug)]
#[command(name = "kiln", version, about = " Local, multilingual native build orchestrator")]
pub struct Cli {
    #[command(flatten)]
    pub global_options: GlobalOpts,


    #[command(subcommand)]
    pub command: Option<Commands>
}