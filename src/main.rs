use clap::{Parser, Subcommand};
use file_diff::diff;
use home::home_dir;
use std::path::PathBuf;
use walkdir::WalkDir;

mod add;
mod diff;
mod install;
mod list;
mod stage;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    commands: Commands,

    /// The repository the configuration files are stored in
    #[arg(short, long)]
    repo: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
    List {
        #[arg(short, long)]
        simple: bool,
    },
    Diff {
        file: Option<PathBuf>,
    },
}

#[derive(Debug)]
pub struct Context {
    repo: PathBuf,
}

impl Context {
    fn new(cli: &CLI) -> Context {
        if let Some(repo) = &cli.repo {
            Context { repo: repo.clone() }
        } else {
            let repo = std::env::var("CONFIGURATOR_REPO")
                .unwrap_or_else(|_| panic!("$CONFIGURATOR_REPO not defined"))
                .into();
            Context { repo }
        }
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
    fn configurator_to_absolute_path(&self, file: &PathBuf) -> PathBuf {
        let file = std::fs::canonicalize(file).unwrap();
        let file = file.strip_prefix(&self.repo).unwrap();
        if file.starts_with("home") {
            let mut path = home_dir().unwrap();
            path.push(file.strip_prefix("home").unwrap());
            path
        } else {
            let mut path: PathBuf = "/".into();
            path.push(file.strip_prefix("root").unwrap());
            path
        }
    }
    fn all_configuration_files(&self) -> Vec<PathBuf> {
        let home = self.repo.join("home");
        let root = self.repo.join("root");
        std::fs::create_dir_all(&home).unwrap();
        std::fs::create_dir_all(&root).unwrap();
        // std::fs::read_dir(&home).unwrap().chain(std::fs::read_dir(&root).unwrap())
        WalkDir::new(&home)
            .into_iter()
            .chain(WalkDir::new(&root).into_iter())
            .map(Result::unwrap)
            .map(walkdir::DirEntry::into_path)
            .filter(|p| p.is_file())
            .collect()
    }
    fn are_files_different(&self, a: &PathBuf, b: &PathBuf) -> bool {
        !diff(a.to_str().unwrap(), b.to_str().unwrap())
    }
}

fn main() {
    let cli = CLI::parse();
    let ctx = Context::new(&cli);
    match cli.commands {
        Commands::Add { file } => add::add(&ctx, &file),
        Commands::Stage => stage::stage(&ctx),
        Commands::Install { file, all } => install::install(&ctx, &file, all),
        Commands::List { simple } => list::list(&ctx, simple),
        Commands::Diff { file } => diff::diff(&ctx, &file),
    }
    .unwrap();
}
