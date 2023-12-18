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
        // let path_g = env::current_dir();
        let gitdir = PathBuf::from(path.clone()).join(".rit");
        

        if !(force == true | !gitdir.is_dir()) {
            panic!("Not a Git repository %s {}", gitdir.display())
        }

        //read config
        // let config = ini!(&gitdir.join("config").display().to_string());
        // let val = config["section"]["key"].clone().unwrap();

        Self {
            worktree: PathBuf::from(path),
            gitdir: gitdir,
        }
    }

    pub fn repo_path(&self, paths: Vec<PathBuf>) ->PathBuf {
        let finale = &mut self.gitdir.clone();        
        for pat in paths.iter() {
            finale.push(pat.display().to_string());
        }
        finale.to_path_buf()
    }

    pub fn repo_file(&self, mut paths: Vec<PathBuf>, mkdir: bool) -> PathBuf {
        let tep = paths.clone();
        if tep.len() > 1{
            paths.truncate(paths.len() - 1);
            match self.repo_dir(paths.clone(), mkdir){
                Ok(_) => return self.repo_path(tep),
                Err(e) => panic!("Error code: {}", e),
            };
        }else{
            self.repo_path(tep)
        }
    
    }

    pub fn repo_dir(&self, paths: Vec<PathBuf>, mkdir: bool) -> Result<PathBuf, i32> {
        let path = &self.repo_path(paths.clone());
        println!("{}", path.display().to_string());
        if path.exists() {
            if path.is_dir(){
                return Ok(path.to_path_buf());
            }else{
                panic!("Not a directory"); //TODO: show path
            }
        };
            
        if mkdir{
            //might edit the error manage later
            let _ = fs::create_dir_all(path.display().to_string());
            return Ok(path.to_path_buf());
        }else{
            // panic!("Not a directory {}", path.display().to_string());
            return Err(2);//will make future error codes doc for different errors
        }
    }

}

