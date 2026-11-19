use clap::Subcommand;


mod new;
mod init;
mod info;
mod validate;
mod configure;
mod build;
mod run;
mod test;
mod clean;
mod targets;
mod graph;
mod plan;
mod doctor;
mod toolchain;
mod artifacts;

mod shared;


use new::NewArgs;
use init::InitArgs;
use info::InfoArgs;
use validate::ValidateArgs;
use configure::ConfigureArgs;
use build::BuildArgs;
use run::RunArgs;
use test::TestArgs;
use clean::CleanArgs;
use targets::TargetsArgs;
use graph::GraphArgs;
use plan::PlanArgs;
use doctor::DoctorArgs;
use toolchain::ToolchainArgs;


#[derive(Subcommand, Debug)]
pub enum Commands {
    New(NewArgs),
    Init(InitArgs),
    Info(InfoArgs),
    Validate(ValidateArgs),
    Configure(ConfigureArgs),
    Build(BuildArgs),
    Run(RunArgs),
    Test(TestArgs),
    Clean(CleanArgs),
    Targets(TargetsArgs),
    Graph(GraphArgs),
    Plan(PlanArgs),
    Doctor(DoctorArgs),
    Toolchain(ToolchainArgs)
}