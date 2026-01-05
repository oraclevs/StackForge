use anyhow::anyhow;
use colored::Colorize;
use core::{
    read_config_from_file,
    utils::{create_files, create_folders, init_git, thanks_print},
};
use std::{
    env::set_current_dir,
    process::{Command, Stdio},
};
mod flutter_files;
use flutter_files::FlutterFiles;

pub fn flutter(name: &str, git: bool) -> anyhow::Result<()> {
    let binding = name.trim().to_lowercase();
    let name = binding.as_str();
    let config_from_file = read_config_from_file();
    if let Err(_) = config_from_file {
        //Todo
    }
    println!(
        "\n\n{} {}",
        "Initializing Flutter Project:".green().bold(),
        name.blue().bold()
    );
    println!();
    println!("{}", "Creating project root directory".yellow().bold());
    // run flutter create command
    let flutter_create_command = Command::new("flutter")
        .args(["create", name, "--platforms=ios,android"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if flutter_create_command.is_err() {
        return Err(anyhow!(
            "{}",
            "Could not run flutter command, make sure flutter is installed".red()
        ));
    }
    println!("{}", "Project root directory created".green().bold());
    // enter root directory
    println!("\n{}", "Entering root directory\n".yellow().bold());
    set_current_dir(name)?;
    let root_dirs = vec![("assets", "fonts"), ("assets", "images")];
    create_folders(&root_dirs)?;
    // enter flutter dir lib dir and create folders
    set_current_dir("lib")?;
    // create folders
    let lib_dirs = vec![
        ("app", ""),
        ("core", "models"),
        ("core", "providers"),
        ("core", "services"),
        ("core", "themes"),
        ("core", "utils"),
        ("env", ""),
        ("responsive", ""),
        ("shared", "screens"),
        ("shared", "widgets"),
        ("shared/screens", "home"),
        ("shared/screens", "widgets"),
    ];
    create_folders(&lib_dirs)?;
    set_current_dir("..")?;
    // creating files
    let flutter_files = FlutterFiles::new(name.to_string());
    let files = flutter_files.files();
    create_files(&files)?;
    println!("\n{}", "Installing dependencies".yellow().bold());
    // install dependencies and dev dependencies
    let deps = vec![
        ("dio", "^5.9.0"),
        ("hive", "^2.2.3"),
        ("hive_flutter", "^1.1.0"),
        ("envied", "^1.3.1"),
        ("go_router", "^16.3.0"),
        ("riverpod_annotation", "^3.0.3"),
        ("flutter_riverpod", "^3.0.3"),
        ("intl", "^0.20.2"),
        ("image_picker", "^1.2.0"),
        ("material_design_icons_flutter", "^7.0.7296"),
    ];
    let dev_deps = vec![
        ("build_runner", "^2.7.1"),
        ("riverpod_lint", "^3.0.3"),
        ("envied_generator", "^1.3.1"),
        ("riverpod_generator", "^3.0.3"),
        ("flutter_native_splash", "^2.4.7"),
        ("flutter_launcher_icons", "^0.14.4"),
    ];
    flutter_files.append_flutter_dependencies(deps, dev_deps)?;
    // install packages
    let install_command = Command::new("flutter")
        .args(["pub", "get"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if install_command.is_err() {
        return Err(anyhow!("{}", "Could not install packages".red()));
    }
    println!("{}", "Finished installing dependencies\n".green().bold());
    println!("{}", "generating *.g.dart files".yellow().bold());
    // run watch build command
    let build_command = Command::new("dart")
        .args([
            "run",
            "build_runner",
            "build",
            "--delete-conflicting-outputs",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !build_command.success() {
        eprintln!("{}", "Could not generate *.g.dart files".red())
    }
    println!("{}", "*.g.dart files generated\n".green().bold());
    if git {
        init_git()?;
    }
    println!(
        "\n\n{} {}",
        "Finished setting up flutter project for".green().bold(),
        name.blue()
    );
    thanks_print();
    Ok(())
}
