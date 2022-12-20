use crate::Context;
use clap::error::Error;
use colored::Colorize;

pub fn status(ctx: &Context) -> Result<(), Error> {
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
    Ok(())
}
