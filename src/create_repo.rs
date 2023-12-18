#![allow(unused_variables)]
use std::{path::{Path, PathBuf}, io::{Error, Write}, fs, collections::HashMap};
use std::fs::OpenOptions;
use crate::repository::GitRepository;
use ini::ini;
use config::{*};



pub fn repo_create(path: String) -> Result<GitRepository, Error>{
    //"""Create a new repository at path."""
    let repo = GitRepository::new(path.clone(), true);

    let rit = PathBuf::from(".rit");
    if !(rit.exists()){
        if !(rit.is_dir()){
            fs::create_dir_all(".rit");
        }
    }
    
    if repo.worktree.exists(){
        if !(repo.worktree.is_dir()){
            panic!("Not a directory! {}", path)
        };
        if repo.gitdir.exists() & repo.gitdir.read_dir()?.next().is_none(){

        }
    }else{
        let _ = fs::create_dir(repo.worktree.display().to_string());
    }

    repo.repo_dir(vec![PathBuf::from("branches")], true);
    repo.repo_dir(vec![PathBuf::from("objects")], true);
    repo.repo_dir(vec![PathBuf::from("refs"), PathBuf::from("tags")], true);
    repo.repo_dir(vec![PathBuf::from("refs"), PathBuf::from("heads")], true);

    let gitdir = repo.gitdir.clone();
    // println!(" am here {}", repo.repo_file(vec![PathBuf::from("description")], true).display().to_string());
    //.git/description
    // println!("{}", gitdir.display().to_string());
    let mut d_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(repo.repo_file(vec![PathBuf::from("description")], false))
    .expect("Couldn't open file!");

    let description = "Unnamed repository; edit this file 'description' to name the repository.\n";

    d_file.write_all(description.as_bytes()).expect("Couldn't write to file!");

    // .git/HEAD

    let mut h_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(repo.repo_file(vec![PathBuf::from("HEAD")], false))
    .expect("Couldn't open file!");

    let HEAD = "ref: refs/heads/master\n";

    h_file.write_all(HEAD.as_bytes()).expect("Couldn't write to file!");
    repo_default_config(&repo);
    // // Creating config file
    
    // let mut c_file = OpenOptions::new()
    // .create_new(true)
    // .append(true)
    // .open(repo.repo_file(vec![gitdir.clone(), PathBuf::from("config")], false))
    // .expect("Couldn't open file!");

    // let config = ini!(&gitdir.join("config").display().to_string());

    Ok(repo)
}

fn repo_default_config(repo: &GitRepository){
    let defauly_config = r"[core]
        bare = false
        filemode = false
        repositoryformatversion = 0
    ";
    let mut c_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(repo.repo_file(vec![PathBuf::from("config")], false))
    .expect("Couldn't open file!");

    c_file.write_all(defauly_config.as_bytes()).expect("Couldn't write to file!");

}