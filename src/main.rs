use std::fmt::Debug;

use clap::Parser;

use crate::{domain::{errors::AppError, manifest::load::load_manifest}, presentation::cli::Cli};

mod presentation;
mod domain;


fn run() -> Result<(), AppError> {
    /* Start Main Code */

    let cli = Cli::parse();
    let config = load_manifest(cli.global_options.manifest_path.unwrap_or_else(|| "./".to_string()))?;

    println!("{:?}", config);

    /* End Main Code */

    Ok(())
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::ExitCode::from(e.exit_code() as u8)
        }
    }
}