use core::panic;
use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::Read;
use ini::ini;

// #[derive(Copy, Clone)]
pub struct GitRepository{
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
    
}

impl GitRepository {
    pub fn new(path: String, mut force: bool) -> Self{
        let gitdir; 
        let path_g = env::current_dir();
        match path_g {
            Ok(path) => {gitdir = PathBuf::from(path.display().to_string()).join(".git")},
            Err(err) => panic!("Error getting current directory: {}", err)
        }

        if force == true | !gitdir.is_dir() {
            panic!("Not a Git repository %s {}", gitdir.display())
        }

        //read config
        let config = ini!(&gitdir.join("config.ini").display().to_string());
        let val = config["section"]["key"].clone().unwrap();

        Self {
            worktree: PathBuf::from(path),
            gitdir: gitdir,
        }
    }

    fn repo_file(&self, paths: Vec<Pathbuf>, mkdir: bool) {
        
    }

    fn repo_dir(&self, paths: Vec<PathBuf>, mkdir: bool) {
        let path = &self.repo_path(&paths);
        for pat in paths.iter() {
            if pat.exists() {
                if pat.is_dir(){
                    
                }else{
                    if mkdir{
                        //might edit the error manage later
                        fs::create_dir(pat.display().to_string()).unwrap_err();
                    
                    }else{
                        panic!("Not a directory {}", pat.display().to_string())
                    }
                }
            }else{
                panic!("directory does not exist{}", pat.display().to_string());
            }
        }

        
    }

    fn repo_path(&self, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
        let mut pathsz = Vec::new();
        for path in paths {
            pathsz.push(self.gitdir.clone().join(path))
        }

        pathsz
    }
}

