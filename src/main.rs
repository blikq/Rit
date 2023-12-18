use std::env;
mod commands;
mod repository;
mod create_repo;

use commands::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Git::new(&args);
    match command{
        Git::Init => {create_repo::repo_create(".".to_string());},
        _ => panic!("Not yet implemented")
    }
    // match command {
    //     Command::Add => println!("norm work"),
    //     Command::Help => println!("help"),
    //     _ => println!("weird shit happened"),
    // }
    // repository::GitRepository::__init__("as".to_string(), true);

}

