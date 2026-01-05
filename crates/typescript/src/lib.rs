mod backend_files;
mod config_json_models;
use crate::backend_files::BackendFiles;
use colored::Colorize;
use core::utils::{
    create_dir_and_change_dir, create_files, create_folders, init_git, thanks_print,
    write_json_file,
};
use std::process::Command;
use std::{env::set_current_dir, process::Stdio};

pub fn setup_npm() -> Result<(), std::io::Error> {
    Command::new("npm")
        .args(["install", "express", "dotenv"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Command::new("npm")
        .args(["install", "--save-dev", "@types/express", "@types/dotenv"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(())
}

pub fn typescript_express(name: &str, git: bool) -> anyhow::Result<()> {
    println!(
        "{} {}",
        "Initializing Typescript Backend Project:".green().bold(),
        name.blue().bold()
    );
    //create the root dir
    create_dir_and_change_dir(&name, "src")?;
    // enter the src folder
    set_current_dir("src")?;
    println!("{}", "Creating project files and directories".yellow());
    //create project root folders
    let root_dirs = vec![
        ("Db", ""),
        ("Controllers", ""),
        ("Models", ""),
        ("Routes", ""),
        ("Middlewares", ""),
        ("Utils", "Constants"),
        ("Utils", "Helpers"),
        ("Utils", "Types"),
        ("Config", ""),
        ("Services", "Email"),
        ("Validators", ""),
    ];
    create_folders(&root_dirs)?;
    set_current_dir("..")?;
    // files
    let backend = BackendFiles::new(name.to_string(), 5001);
    let files: Vec<(String, String)> = backend.files();
    create_files(&files)?;
    println!("{}\n", "Creating project files and directories".green());
    println!(
        "{}",
        "Setting up typescript dev environment".yellow().bold()
    );
    // set up typescript dev environment
    let tsc_command = Command::new("tsc")
        .arg("--init")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match tsc_command {
        Ok(_) => println!(),
        Err(_) => eprintln!("Could not run tsc --init, please check it tsc is installed globally"),
    }

    // configure tsconfig and package.json
    write_json_file("tsconfig.json", &backend.compiler_options_file())?;
    write_json_file("package.json", &backend.init_package_json_file())?;
    println!("{}", "Installing packages".yellow());
    match setup_npm() {
        Ok(_) => println!("{}", "Finished installing packages".green()),
        Err(e) => eprintln!("{}::{}", "Could not run npm command".yellow(), e),
    }
    println!(
        "\n{}\n",
        "Finished setting up typescript dev environment"
            .green()
            .bold()
    );
    // git
    if git {
        init_git()?;
    }
    thanks_print();
    Ok(())
}
