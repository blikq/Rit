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

    fn repo_path(&self, paths: Vec<PathBuf>) ->PathBuf {
        let mut finale = &mut self.gitdir.clone();        
        for pat in paths.iter() {
            finale.push(pat.display().to_string());
        }
        finale.to_path_buf()
    }

    fn repo_file(&self, paths: Vec<PathBuf>, mkdir: bool) {
        if self.repo_dir(paths.clone(), mkdir) {
            self.repo_path(paths);
        }
    }

    fn repo_dir(&self, paths: Vec<PathBuf>, mkdir: bool) -> bool {
        let path = &self.repo_path(paths.clone());
        for pat in paths.iter() {
            if pat.exists() {
                if pat.is_dir(){
                    return true;
                }else{
                    return false;
                }
            }else{
                panic!("directory does not exist{}", pat.display().to_string());
            }
        }
        for pat in paths.iter() {
            
            if mkdir{
                //might edit the error manage later
                fs::create_dir(pat.display().to_string()).unwrap_err();
                return true;
            }else{
                panic!("Not a directory {}", pat.display().to_string());
                
            }
        }
        return true;
    }

    // fn repo_path(&self, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    //     let mut pathsz = Vec::new();
    //     for path in paths {
    //         pathsz.push(self.gitdir.clone().join(path))
    //     }

    //     pathsz
    // }
}

