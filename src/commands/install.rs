use crate::context::Context;
use clap::error::Error;
use colored::Colorize;
use inquire::Confirm;
use std::path::PathBuf;

pub fn install(ctx: &Context, file: &Option<PathBuf>, all: bool) -> Result<(), Error> {
    if let Some(system_file) = file {
        let repo_file = ctx.absolute_to_configurator_path(&system_file);
        if !repo_file.exists() {
            println!(
                "{}",
                format!("{} is not in the repository", system_file.display()).red()
            );
        } else {
            if (!system_file.exists())
                || Confirm::new(&format!(
                    "Overwrite {}?",
                    &system_file.display().to_string().bold()
                ))
                .with_default(true)
                .prompt()
                .unwrap()
            {
                ctx.copy(&repo_file, &system_file)?;
            }
        }
    } else {
        let mut was_file_installed = false;
        for repo_file in ctx.all_configuration_files() {
            let system_file = ctx.configurator_to_absolute_path(&repo_file);
            if ctx.are_files_different(&repo_file, &system_file)
                && (all
                    || Confirm::new(&format!(
                        "Overwrite {}?",
                        &system_file.display().to_string().bold()
                    ))
                    .with_default(false)
                    .prompt()
                    .unwrap())
            {
                ctx.copy(&repo_file, &system_file)?;
                println!("Installed {}", &system_file.display().to_string().bold());
                was_file_installed = true;
            }
        }
        if !was_file_installed {
            println!("{}", "Everything is already up to date!".green());
        }
    }
    Ok(())
}
