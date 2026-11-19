use clap::Args;


#[derive(Args, Debug, Clone)]
#[command(about = "Removes generated artifacts and intermediate build state inside output roots")]
pub struct CleanArgs {}