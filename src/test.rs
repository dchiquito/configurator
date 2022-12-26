use home::home_dir;
use std::path::PathBuf;

use crate::context::Context;

pub struct Helper {
    repo_dir: PathBuf,
    system_dir: PathBuf,
    home_dir: PathBuf,
}

impl Helper {
    pub fn setup() -> (Helper, Context) {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("configurator");

        // Repository
        let mut repo_dir = temp_dir.clone();
        repo_dir.push("repo");
        std::fs::create_dir_all(&repo_dir).unwrap();

        // System files
        let mut system_dir = temp_dir.clone();
        system_dir.push("system");
        std::fs::create_dir_all(&system_dir).unwrap();

        // Home directory
        let mut home_dir = home_dir().unwrap();
        home_dir.push(".configurator");
        home_dir.push("test");
        std::fs::create_dir_all(&home_dir).unwrap();

        let ctx = Context::new(&Some(repo_dir.clone()));
        let helper = Helper {
            repo_dir,
            system_dir,
            home_dir,
        };
        (helper, ctx)
    }
    fn create_file(&self, dir: &PathBuf, name: &str, contents: &str) -> PathBuf {
        let mut file = dir.clone();
        file.push(name);
        std::fs::write(file.clone(), contents).unwrap();
        file
    }
    pub fn create_repo_file(&self, name: &str, contents: &str) -> PathBuf {
        self.create_file(&self.repo_dir, name, contents)
    }
    pub fn create_system_file(&self, name: &str, contents: &str) -> PathBuf {
        self.create_file(&self.system_dir, name, contents)
    }
    pub fn create_home_file(&self, name: &str, contents: &str) -> PathBuf {
        self.create_file(&self.home_dir, name, contents)
    }
    fn no_file(&self, dir: &PathBuf, name: &str) -> PathBuf {
        let mut file = dir.clone();
        file.push(name);
        // We don't care if this fails
        std::fs::remove_file(file.clone()).ok();
        file
    }
    pub fn no_repo_file(&self, name: &str) -> PathBuf {
        self.no_file(&self.repo_dir, name)
    }
    pub fn no_system_file(&self, name: &str) -> PathBuf {
        self.no_file(&self.system_dir, name)
    }
    pub fn no_home_file(&self, name: &str) -> PathBuf {
        self.no_file(&self.home_dir, name)
    }
    pub fn assert_eq(&self, a: &PathBuf, b: &PathBuf) {
        assert!(Context::new(&Some(self.repo_dir.clone())).are_files_different(a, b));
    }
}
