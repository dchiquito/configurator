use crate::Context;
use clap::error::Error;

pub fn list(ctx: &Context) -> Result<(), Error> {
    for repo_file in ctx.all_configuration_files() {
        let system_file = ctx.configurator_to_absolute_path(&repo_file);
        println!("{}", system_file.display());
    }
    Ok(())
}
