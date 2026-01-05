use crate::{
    backend_files::BackendFiles,
    utils::{create_ws_bin, create_ws_libs},
};
use colored::Colorize;
use core::utils::{
    create_dir_and_change_dir, create_files, create_folders, init_git, thanks_print,
};
use std::{env::set_current_dir, vec};

pub fn backend_init(name: &str, git: bool) -> anyhow::Result<()> {
    println!(
        "{} {}",
        "Initializing Rust Backend Project:".green(),
        name.blue()
    );
    // create root project
    create_dir_and_change_dir(name, "crates")?;
    // create crates crates
    let crates = vec![
        "domain".to_string(),
        "api".to_string(),
        "infra".to_string(),
        "shared".to_string(),
    ];

    set_current_dir("crates")?;
    create_ws_libs(&crates)?;
    // create service binary
    set_current_dir("..")?;
    create_folders(&vec![("services", "")])?;
    set_current_dir("services")?;
    let bins = vec!["monolith", "auth-service", "billing-service"];
    for b in bins {
        create_ws_bin(b)?;
    }
    set_current_dir("..")?;
    println!("{}", "Creating files and directories".yellow());
    // create dirs
    let dirs = vec![("docker", ""), ("config", ""), ("migrations", "")];
    create_folders(&dirs)?;
    // writes config files
    let backend_files = &BackendFiles::new(name.to_string()).files();
    create_files(backend_files)?;
    set_current_dir("..")?;
    println!("{}", "Finish creating files and directories".green());
    if git {
        init_git()?;
    }
    println!("{}", "Rust backend project is ready!".green());
    thanks_print();

    Ok(())
}
