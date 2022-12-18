use crate::Context;
use clap::error::Error;
use std::path::PathBuf;

pub fn add(ctx: &Context, file: &PathBuf) -> Result<(), Error> {
    let system_file = std::fs::canonicalize(file)?;
    let repo_file = ctx.absolute_to_configurator_path(&system_file);
    std::fs::copy(&system_file, &repo_file)?;
    println!("File added: {}", repo_file.display());
    Ok(())
}
