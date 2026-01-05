use std::vec;

use crate::utils::{create_ws_bin, create_ws_libs, workspace_toml_content};
use anyhow::Ok;
use colored::Colorize;
use core::utils::{create_dir_and_change_dir, create_files, init_git, thanks_print};
use std::env::set_current_dir;

pub fn work_space_init(
    work_space_names: &Vec<String>,
    git: bool,
    name: &str,
) -> anyhow::Result<()> {
    println!("{} {}", "Initializing Rust Workspace:".green(), name.blue());
    // Create the main project directory
    create_dir_and_change_dir(&name, "crates")?;
    // create workspace toml file
    println!("{}", "Creating toml file for workspace".yellow());
    let toml_content: Vec<(String, String)> = vec![
        (
            String::from("Cargo.toml"),
            (workspace_toml_content(&work_space_names)),
        ),
        (String::from(".gitignore"), String::from("target/")),
    ];
    create_files(&toml_content)?;
    println!("{}\n", "Fished creating toml file for workspace".green());
    // init git if true
    if git {
        init_git()?;
    }
    set_current_dir("crates")?;
    //create all libs and the main bin
    create_ws_libs(work_space_names)?;
    // create main bin
    create_ws_bin(&name)?;
    thanks_print();
    Ok(())
}

pub fn default_project(name: &str, git: bool) -> anyhow::Result<()> {
    println!("{} {}\n", "Initializing Rust project:".green(), name.blue());
    create_ws_bin(name)?;
    set_current_dir(&name)?;
    let git_ignore: Vec<(String, String)> =
        vec![(String::from(".gitignore"), String::from("target/"))];

    create_files(&git_ignore)?;
    if git {
        init_git()?;
    }
    Ok(())
}
