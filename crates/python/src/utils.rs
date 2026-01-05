use anyhow::{Ok, Result};
use colored::Colorize;
use core::utils::thanks_print;
use std::{
    path::Path,
    process::{Command, ExitStatus, Stdio},
};

pub fn python_env_setup(packages: &Vec<&str>) -> Result<()> {
    println!("\n{}", "Setting up Python virtual environment".yellow());

    // Determine python executable
    #[cfg(target_os = "windows")]
    let python_cmd = "py";

    #[cfg(not(target_os = "windows"))]
    let python_cmd = "python3";

    let venv_status: ExitStatus = Command::new(python_cmd)
        .args(["-m", "venv", ".venv"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if venv_status.success() {
        println!("{}\n", "Python virtual environment set up.".green());
    } else {
        println!("{}", "Failed to set up Python virtual environment.".red());
    }
    // activate venv and install required packages
    let venv_path = Path::new(".venv");
    println!("{}", "Installing required packages".yellow());
    let pip_path = if cfg!(target_os = "windows") {
        venv_path.join("Scripts").join("pip")
    } else {
        venv_path.join("bin").join("pip")
    };
    if packages.is_empty() {
        println!("{}\n", "Required packages installed.".green());
        thanks_print();
        return Ok(());
    }
    let install_status = Command::new(&pip_path)
        .arg("install")
        .args(packages)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if install_status.success() {
        #[cfg(not(target_os = "windows"))]
        Command::new("sh")
            .arg("-c")
            .arg(format!("{:?} freeze > requirements.txt", pip_path))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        #[cfg(target_os = "windows")]
        Command::new("cmd")
            .arg("/C")
            .arg(format!("{:?} freeze > requirements.txt", pip_path))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        println!("{}\n", "Required packages installed.".green());
        thanks_print();
    } else {
        println!("{}", "Failed to install required packages.".red());
    }
    Ok(())
}
