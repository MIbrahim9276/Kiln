use clap::Args;


#[derive(Args, Debug, Clone)]
#[command(about = "Create a new directory named <NAME> and initialize it")]
pub struct NewArgs {
    #[arg(value_name = "NAME")]
    pub name: String
}