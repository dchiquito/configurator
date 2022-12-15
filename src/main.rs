use std::path::PathBuf;
use clap::{Parser, Subcommand};
use home::home_dir;

mod add;
mod stage;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    commands: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a configuration file to the repository
    Add {
        /// The configuration file to add to the repository
        file: PathBuf,
    },
    /// Update the repository using modified local configuration files
    Stage {
        /// A specific file to stage
        file: Option<PathBuf>,
    },
}

#[derive(Debug)]
pub struct Context {
    repo: PathBuf,
}

impl Context {
    fn new(cli: &CLI) -> Context {
        // TODO check cli args for repo root
        let repo = std::env::var("CONFIGURATOR_REPO").unwrap_or_else(|_| panic!("$CONFIGURATOR_REPO not defined")).into();
        Context { repo }
    }
    fn absolute_to_configurator_path(&self, file: &PathBuf) -> PathBuf {
        let mut path = self.repo.clone();
        let home = home_dir().unwrap();
        if file.starts_with(&home) {
            path.push("home");
            path.push(file.strip_prefix(&home).unwrap());
        } else {
            path.push("root");
            path.push(file.strip_prefix("/").unwrap());
        }
        path
    }
}

fn main() {
    let cli = CLI::parse();
    let ctx = Context::new(&cli);
    match cli.commands {
        Commands::Add { file } => add::add(&ctx, &file),
        Commands::Stage { file } => stage::stage(&ctx, &file),
    }.unwrap();
}
