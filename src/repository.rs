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
        let finale = &mut self.gitdir.clone();        
        for pat in paths.iter() {
            finale.push(pat.display().to_string());
        }
        finale.to_path_buf()
    }

    fn repo_file(&self, mut paths: Vec<PathBuf>, mkdir: bool) -> PathBuf {
        paths.truncate(paths.len() - 1);
        match self.repo_dir(paths.clone(), mkdir){
            Ok(_) => return self.repo_path(paths),
            Err(e) => panic!("Error code: {}", e),
        };
        // self.repo_path(paths);
        
    }

    fn repo_dir(&self, paths: Vec<PathBuf>, mkdir: bool) -> Result<PathBuf, i32> {
        let path = &self.repo_path(paths.clone());
        if path.exists() {
            if path.is_dir(){
                return Ok(path.to_path_buf());
            }else{
                panic!("Not a directory"); //TODO: show path
            }
        };
            
        if mkdir{
            //might edit the error manage later
            fs::create_dir(path.display().to_string()).unwrap_err();
            return Ok(path.to_path_buf());
        }else{
            // panic!("Not a directory {}", path.display().to_string());
            return Err(2);//will make future error codes doc for different errors
        }
    }

    // fn repo_path(&self, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    //     let mut pathsz = Vec::new();
    //     for path in paths {
    //         pathsz.push(self.gitdir.clone().join(path))
    //     }

    //     pathsz
    // }
}

