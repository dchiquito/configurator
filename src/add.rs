use crate::Context;
use clap::error::Error;
use colored::Colorize;
use std::path::PathBuf;

pub fn add(ctx: &Context, file: &PathBuf) -> Result<(), Error> {
    let system_file = std::fs::canonicalize(file)?;
    let repo_file = ctx.absolute_to_configurator_path(&system_file);
    if ctx.are_files_different(&system_file, &repo_file) {
        std::fs::create_dir_all(&repo_file.parent().unwrap())?;
        std::fs::copy(&system_file, &repo_file)?;
        println!("Added {}", repo_file.display().to_string().bold());
    } else {
        println!("{}", "Nothing to update!".green());
    }
    Ok(())
}
