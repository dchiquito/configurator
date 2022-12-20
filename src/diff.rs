use crate::Context;
use clap::error::Error;
use colored::Colorize;
use diffy::{create_patch, PatchFormatter};
use std::path::PathBuf;

fn print_diff(repo_file: &PathBuf, system_file: &PathBuf) -> Result<(), Error> {
    if !system_file.exists() {
        println!(
            "{}",
            format!("{} does not exist", system_file.display()).red()
        );
    } else {
        let repo_contents = std::fs::read(repo_file)?;
        let repo_contents = String::from_utf8_lossy(&repo_contents);
        let system_contents = std::fs::read(system_file)?;
        let system_contents = String::from_utf8_lossy(&system_contents);
        let patch = create_patch(&system_contents, &repo_contents);
        let f = PatchFormatter::new().with_color();
        print!("{}", f.fmt_patch(&patch));
    }
    Ok(())
}

pub fn diff(ctx: &Context, file: &Option<PathBuf>) -> Result<(), Error> {
    if let Some(file) = file {
        if !file.exists() {
            println!("{}", format!("{} does not exist", file.display()).red());
        } else {
            let system_file = std::fs::canonicalize(file)?;
            let repo_file = ctx.absolute_to_configurator_path(&system_file);
            if ctx.are_files_different(&system_file, &repo_file) {
                print_diff(&repo_file, &system_file)?;
            }
        }
    } else {
        for repo_file in ctx.all_configuration_files() {
            let system_file = ctx.configurator_to_absolute_path(&repo_file);
            if ctx.are_files_different(&system_file, &repo_file) {
                println!("{}", &system_file.display().to_string().bold());
                print_diff(&repo_file, &system_file)?;
            }
        }
    }
    Ok(())
}
