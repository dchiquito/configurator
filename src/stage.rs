use std::path::PathBuf;
use clap::error::Error;
use crate::Context;

pub fn stage(ctx: &Context, file: &Option<PathBuf>) -> Result<(), Error> {
    if let Some(file) = file {
        let system_file = std::fs::canonicalize(file)?;
        let repo_file = ctx.absolute_to_configurator_path(&system_file);
        std::fs::create_dir_all(repo_file.parent().unwrap())?;
        std::fs::copy(&system_file, &repo_file)?;
        println!("File staged: {}", repo_file.display());
    } else {
        println!("Staging all configuration files");
        for repo_file in ctx.all_configuration_files() {
            let system_file = ctx.configurator_to_absolute_path(&repo_file);
            println!("{} -> {}", system_file.display(), repo_file.display());
            std::fs::copy(&system_file, &repo_file)?;
        }
    }
    Ok(())
}