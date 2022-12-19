use crate::Context;
use clap::error::Error;

pub fn stage(ctx: &Context) -> Result<(), Error> {
    println!("Staging all configuration files");
    for repo_file in ctx.all_configuration_files() {
        let system_file = ctx.configurator_to_absolute_path(&repo_file);
        if ctx.are_files_different(&repo_file, &system_file) {
            println!("Updating {}", system_file.display());
            std::fs::copy(&system_file, &repo_file)?;
        }
    }
    Ok(())
}
