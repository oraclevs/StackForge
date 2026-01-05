use anyhow::Result;
use colored::Colorize;
use serde::Serialize;
use std::{
    env::set_current_dir,
    fs::{File, create_dir_all, write},
    io::{self, Write},
    path::Path,
    process::{Command, ExitStatus, Stdio},
};

pub fn init_git() -> Result<()> {
    println!("\n{}", "Initializing git repository".yellow());
    let git_status: ExitStatus = Command::new("git")
        .arg("init")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if git_status.success() {
        println!("{}\n", "Git repository initialized".green());
    } else {
        println!("{}\n", "Failed to initialize git repository.".red());
    }
    Ok(())
}
pub fn create_dir_and_change_dir(dir_name: &str, src_path: &str) -> Result<()> {
    let root_path: &Path = Path::new(&dir_name);

    let src_path_name: String = if src_path.is_empty() {
        dir_name.to_string()
    } else {
        format!("{}/{}", dir_name, src_path)
    };
    // create root directory
    if !root_path.exists() {
        match create_dir_all(&src_path_name) {
            Ok(_) => println!(),
            Err(_) => println!("{}", "Could not create directory".red()),
        }
        println!("{}", "Root directory created.".green());
    } else {
        println!("{}", "Directory already exists.".yellow());
    }
    // change into the package root directory
    if let Err(err) = set_current_dir(&root_path) {
        println!(
            "{}",
            format!("Failed to enter root directory  {}", err).red()
        );
        return Err(anyhow::anyhow!(err));
    }
    println!("{}\n", "Entered root directory.".green());
    Ok(())
}

pub fn create_files(file_content: &Vec<(String, String)>) -> Result<()> {
    for (file_name, content) in file_content {
        let file_path: &Path = Path::new(&file_name);
        let mut file: File = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        println!("{} {}", "Created file:".green(), file_name.blue());
    }
    Ok(())
}

pub fn create_folders(paths: &Vec<(&str, &str)>) -> Result<()> {
    if paths.is_empty() {
        return Err(anyhow::anyhow!("no folders provided"));
    }

    for (dir_name, src_path) in paths {
        let src_path_name: String = if src_path.is_empty() {
            dir_name.to_string()
        } else {
            format!("{}/{}", dir_name, src_path)
        };

        let path: &Path = Path::new(&src_path_name);

        // create directory tree
        if !path.exists() {
            match create_dir_all(&src_path_name) {
                Ok(_) => println!("{} {}", "Created directory:".green(), src_path_name.blue()),
                Err(e) => {
                    println!("{}", "Could not create directory".red());
                    return Err(anyhow::anyhow!(e));
                }
            }
        } else {
            println!(
                "{} {}",
                "Directory already exists:".yellow(),
                src_path_name.blue()
            );
        }
    }

    Ok(())
}
pub fn write_json_file<T: Serialize, P: AsRef<Path>>(path: P, value: &T) -> Result<(), io::Error> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if let Some(parent) = path.as_ref().parent() {
        create_dir_all(parent)?;
    }

    write(path, json)?;
    Ok(())
}

pub fn thanks_print() {
    println!(
        "\n{}\n",
        " Happy hacking from StackForge 📦🥳 "
            .to_uppercase()
            .green()
            .bold()
            .on_white()
    );
}
pub fn welcome_print() {
    println!(
        "\n{}\n",
        "Welcome to StackForge Cli 🛠️  "
            .to_uppercase()
            .black()
            .bold()
            .on_bright_yellow()
    );
}
