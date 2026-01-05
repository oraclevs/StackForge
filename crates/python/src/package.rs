//***************************************************
//* Create Python Package                           *
//***************************************************

use crate::package_files::FileContent;
use crate::utils::python_env_setup;
use anyhow::{Ok, Result};
use colored::Colorize;
use core::utils::{create_dir_and_change_dir, create_files, init_git};

pub fn package(name: &str, with_git: bool) -> Result<()> {
    let file_contents: FileContent = FileContent::new(&name);
    println!(
        "{} {}\n",
        "Creating root structure for python package:".green(),
        name.blue()
    );
    // create root directory and enter into it
    create_dir_and_change_dir(&name, &name)?;
    // create files and folders for the python package
    println!("{}","Creating files".yellow());
    let file_content = file_contents.list_all_files();
    create_files(&file_content)?;
    println!("{}","Finished creating files".green());
    // initialize git repository if requested
    if with_git {
        init_git()?;
    }
    // init python and set up venv
    let packages = vec!["setuptools", "wheel", "rich", "pyfiglet"];
    python_env_setup(&packages)?;
    Ok(())
}
