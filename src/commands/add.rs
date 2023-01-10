use crate::context::Context;
use clap::error::Error;
use colored::Colorize;
use std::path::PathBuf;

pub fn add(ctx: &Context, file: &PathBuf) -> Result<(), Error> {
    let system_file = std::fs::canonicalize(file)?;
    if system_file.is_dir() {
        for subfile in system_file
            .read_dir()
            .expect("Failed to list the directory contents")
        {
            add(
                ctx,
                &subfile
                    .expect("Failed to list the directory contents")
                    .path(),
            )?;
        }
    } else {
        let repo_file = ctx.absolute_to_configurator_path(&system_file);
        if ctx.are_files_different(&system_file, &repo_file) {
            ctx.copy(&system_file, &repo_file)?;
            println!("Added {}", repo_file.display().to_string().bold());
        } else {
            println!("{}", "Nothing to update!".green());
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::add;
    use crate::test::Helper;

    #[test]
    fn test_add_new_home_file() {
        let (helper, ctx) = Helper::setup();
        let home = helper.create_home_file("a.txt", "a");
        let repo = helper.no_repo_home_file("a.txt");
        add(&ctx, &home).unwrap();
        helper.assert_eq(&home, &repo);
    }
    #[test]
    fn test_add_new_system_file() {
        let (helper, ctx) = Helper::setup();
        let system = helper.create_system_file("a.txt", "a");
        let repo = helper.no_repo_system_file("a.txt");
        add(&ctx, &system).unwrap();
        helper.assert_eq(&system, &repo);
    }
    #[test]
    fn test_add_home_file_overwrite() {
        let (helper, ctx) = Helper::setup();
        let home = helper.create_home_file("a.txt", "a");
        let repo = helper.create_repo_home_file("a.txt", "b");
        add(&ctx, &home).unwrap();
        helper.assert_eq(&home, &repo);
    }
    #[test]
    fn test_add_system_file_overwrite() {
        let (helper, ctx) = Helper::setup();
        let system = helper.create_system_file("a.txt", "a");
        let repo = helper.create_repo_system_file("a.txt", "b");
        add(&ctx, &system).unwrap();
        helper.assert_eq(&system, &repo);
    }
}
