use clap::error::Error;
use file_diff::diff;
use home::home_dir;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Context {
    repo: PathBuf,
}

impl Context {
    pub fn new(repo: &Option<PathBuf>) -> Context {
        if let Some(repo) = repo {
            Context { repo: repo.clone() }
        } else {
            let repo = std::env::var("CONFIGURATOR_REPO")
                .unwrap_or_else(|_| panic!("$CONFIGURATOR_REPO not defined"))
                .into();
            Context { repo }
        }
    }
    pub fn absolute_to_configurator_path(&self, file: &PathBuf) -> PathBuf {
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
    pub fn configurator_to_absolute_path(&self, file: &PathBuf) -> PathBuf {
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
    pub fn all_configuration_files(&self) -> Vec<PathBuf> {
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
    pub fn are_files_different(&self, a: &PathBuf, b: &PathBuf) -> bool {
        !diff(a.to_str().unwrap(), b.to_str().unwrap())
    }
    pub fn copy(&self, src: &PathBuf, dest: &PathBuf) -> Result<(), Error> {
        Copier::new(self).add_file(src, dest).commit()
    }
}

/// Stores a list of files to copy all at once.
/// This prevents multiple redundant file copies in the event of privilege escalation.
pub struct Copier {
    ctx: Context,
    files_to_copy: Vec<(PathBuf, PathBuf)>,
}

impl Copier {
    pub fn new(ctx: &Context) -> Copier {
        Copier {
            ctx: ctx.clone(),
            files_to_copy: vec![],
        }
    }
    pub fn add_file(&mut self, src: &PathBuf, dest: &PathBuf) -> &mut Copier {
        self.files_to_copy.push((src.clone(), dest.clone()));
        self
    }
    pub fn commit(&self) -> Result<(), Error> {
        for (src, dest) in self.files_to_copy.iter() {
            std::fs::create_dir_all(&dest.parent().unwrap()).unwrap();
            std::fs::copy(&src, &dest)?;
        }
        Ok(())
    }
}
