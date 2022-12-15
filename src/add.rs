use std::path::PathBuf;
use clap::error::Error;
use crate::Context;

pub fn add(ctx: &Context, file: &PathBuf) -> Result<(), Error> {
    let system_file = std::fs::canonicalize(file)?;
    let repo_file = ctx.absolute_to_configurator_path(&system_file);
    if repo_file.exists() {
        println!("File already exists: {}", repo_file.display());
        println!("Use \"configurator stage {}\" to stage a modified file for commit", file.display());
    } else {
        std::fs::create_dir_all(repo_file.parent().unwrap())?;
        std::fs::copy(&system_file, &repo_file)?;
        println!("File added: {}", repo_file.display());
    }
    Ok(())
}
