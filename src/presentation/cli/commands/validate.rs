use clap::Args;


#[derive(Args, Debug, Clone)]
#[command(about = "Determine whether the project description is valid")]
pub struct ValidateArgs {}