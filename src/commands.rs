use clap::{Parser, Subcommand};
use std::path::PathBuf;
use sudo::with_env;

use crate::context::Context;

mod add;
mod diff;
mod install;
mod list;
mod stage;
mod status;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    /// The repository the configuration files are stored in
    #[arg(short, long)]
    pub repo: Option<PathBuf>,

    /// Run the command as root
    #[arg(long)]
    pub root: bool,
}

impl Cli {
    pub fn run_command(&self) -> Result<(), clap::error::Error> {
        if self.root {
            with_env(&["CONFIGURATOR"]).unwrap();
        }
        let ctx = Context::new(&self.repo);
        match &self.commands {
            Commands::Add { file } => add::add(&ctx, file),
            Commands::Stage => stage::stage(&ctx),
            Commands::Install { file, all } => install::install(&ctx, file, *all),
            Commands::List => list::list(&ctx),
            Commands::Status { all } => status::status(&ctx, *all),
            Commands::Diff { file } => diff::diff(&ctx, file),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add/update configuration files to the repository
    Add {
        /// The configuration file to add/update
        file: PathBuf,
    },
    /// Update the repository using all registered local configuration files
    Stage,
    /// Install configuration files from the repository onto the system
    Install {
        /// A specific system file to install
        file: Option<PathBuf>,
        /// Install all configuration files with no prompt
        #[arg(short, long)]
        all: bool,
    },
    /// List all configuration files currently stored in the repository
    List,
    /// Show the current installation status for all files currently stored in the repository
    Status {
        /// Show the status for all configuration files
        #[arg(short, long)]
        all: bool,
    },
    /// Show a diff between the system files and the repository files
    Diff {
        /// A specific system file to diff
        file: Option<PathBuf>,
    },
}
