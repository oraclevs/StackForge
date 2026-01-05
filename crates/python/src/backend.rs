use crate::{backend_files::BackendFiles, utils::python_env_setup};
use colored::Colorize;
use core::utils::{create_dir_and_change_dir, create_files, create_folders, init_git};
use std::env::set_current_dir;

pub fn backend(name: &str, git: bool) -> anyhow::Result<()> {
    println!(
        "{} {}",
        "Initializing Python Backend Project:".green(),
        name.blue()
    );
    // create root dir
    create_dir_and_change_dir(name, "app")?;
    set_current_dir("app")?;
    let app_dirs = vec![
        ("core", ""),
        ("api", ""),
        ("domain", ""),
        ("infra", ""),
        ("services", ""),
    ];
    println!("{}", "Creating project directories".yellow());
    create_folders(&app_dirs)?;
    set_current_dir("..")?;
    let root_dirs = vec![("migrations", "versions"), ("config", ""), ("docker", "")];
    create_folders(&root_dirs)?;
    // create files
    let files = BackendFiles::new(name.to_string()).files();
    create_files(&files)?;
    println!("{}", "finished Creating project directories".green());
    // init git
    if git {
        init_git()?;
    }
    python_env_setup(&Vec::new())?;
    Ok(())
}
