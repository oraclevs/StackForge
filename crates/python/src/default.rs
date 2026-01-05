use crate::package_files::FileContent;
use crate::utils::python_env_setup;
use colored::Colorize;
use core::utils::{create_dir_and_change_dir, create_files, init_git};

pub fn default(name: &str, with_git: bool) -> anyhow::Result<()> {
    println!(
        "{} {}",
        "Creating root structure for python project:".green(),
        name.blue()
    );
    // Create default structure here
    create_dir_and_change_dir(&name, "src")?;

    println!("{}", "Creating files".yellow());
    let file = FileContent::new(&name);
    create_files(&file.default_main_file())?;
    println!("{}", "Finished creating files".green());
    // initialize git repository if requested
    if with_git {
        init_git()?;
    }
    // init python and set up venv
    python_env_setup(&Vec::<&str>::new())?;
    Ok(())
}
