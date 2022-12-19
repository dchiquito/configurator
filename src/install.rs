use crate::Context;
use clap::error::Error;
use inquire::Confirm;
use std::path::PathBuf;

pub fn install(ctx: &Context, file: &Option<PathBuf>, all: bool) -> Result<(), Error> {
    if let Some(file) = file {
        let system_file = std::fs::canonicalize(file)?;
        let repo_file = ctx.absolute_to_configurator_path(&system_file);
        if Confirm::new(&format!("Overwrite {}?", &system_file.display()))
            .with_default(true)
            .prompt()
            .unwrap()
        {
            std::fs::create_dir_all(&system_file.parent().unwrap())?;
            std::fs::copy(&repo_file, &system_file)?;
        }
    } else {
        let mut was_file_installed = false;
        for repo_file in ctx.all_configuration_files() {
            let system_file = ctx.configurator_to_absolute_path(&repo_file);
            if ctx.are_files_different(&repo_file, &system_file)
                && (all
                    || Confirm::new(&format!("Overwrite {}?", &system_file.display()))
                        .with_default(false)
                        .prompt()
                        .unwrap())
            {
                std::fs::create_dir_all(&system_file.parent().unwrap())?;
                std::fs::copy(&repo_file, &system_file)?;
                println!("Installed {}", &system_file.display());
                was_file_installed = true;
            }
        }
        if !was_file_installed {
            println!("Everything is already up to date!");
        }
    }
    Ok(())
}
