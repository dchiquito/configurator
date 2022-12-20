use std::path::PathBuf;

use crate::Context;
use clap::error::Error;
use colored::Colorize;

pub fn status(ctx: &Context, all: bool) -> Result<(), Error> {
    if all {
        for repo_file in ctx.all_configuration_files() {
            let system_file = ctx.configurator_to_absolute_path(&repo_file);
            if !ctx.are_files_different(&repo_file, &system_file) {
                print!(" {} ", "🗸".green());
                println!("{}", &system_file.display());
            } else {
                print!(" {} ", "🗶".red());
                println!("{}", &system_file.display().to_string().bold());
            }
        }
    } else {
        let different_files: Vec<PathBuf> = ctx
            .all_configuration_files()
            .iter()
            .map(|repo_file| {
                (
                    repo_file.clone(),
                    ctx.configurator_to_absolute_path(&repo_file),
                )
            })
            .filter(|(repo_file, system_file)| ctx.are_files_different(repo_file, system_file))
            .map(|(_, system_file)| system_file)
            .collect();
        if different_files.is_empty() {
            println!("{}", "Everything is up to date!".green());
        } else {
            for system_file in different_files.iter() {
                if system_file.exists() {
                    println!("{}", format!("modified: {}", &system_file.display()).red());
                } else {
                    println!("{}", format!("missing:  {}", &system_file.display()).red());
                }
            }
        }
    }
    Ok(())
}
