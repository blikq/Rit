use std::env;
mod commands;
mod repository;
mod create_repo;

use commands::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Git::new(&args);
    match command{
        Git::Init => {let path = args.get(2); 
            let _ = match path{
                Some(p) => create_repo::repo_create(p.to_string()),
                None => create_repo::repo_create(".".to_string()),
        };},
        _ => panic!("Not yet implemented")
    }
    // match command {
    //     Command::Add => println!("norm work"),
    //     Command::Help => println!("help"),
    //     _ => println!("weird shit happened"),
    // }
    // repository::GitRepository::__init__("as".to_string(), true);

}

