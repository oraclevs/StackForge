use colored::Colorize;
use core::{create_default_config, view_config};
use flutter::flutter;
use python::python;
use rust::rust;
use typescript::typescript_express;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Language {
    Python,
    Flutter,
    Rust,
    Typescript,
}

pub fn execute_init_command(
    language: Language,
    name: String,
    backend: bool,
    git: bool,
    package: bool,
    work_space: Vec<String>,
) -> anyhow::Result<()> {
    match language {
        Language::Python => match python(&name, backend, git, package) {
            Ok(_) => println!(),
            Err(e) => eprintln!("{} {:?}", "An Error occurred".red(), e),
        },
        Language::Flutter => match flutter(&name, git) {
            Ok(_) => println!(),
            Err(e) => eprintln!("{} {:?}", "An Error occurred".red(), e),
        },
        Language::Rust => match rust(&name, backend, git, package, work_space) {
            Ok(_) => println!(),
            Err(e) => eprintln!("{} {:?}", "An Error occurred".red(), e),
        },
        Language::Typescript => match typescript_express(&name, git) {
            Ok(_) => println!(),
            Err(e) => eprintln!("{} {:?}", "An Error occurred".red(), e),
        },
    }
    Ok(())
}

pub fn execute_config_command(view: bool, create: bool) -> anyhow::Result<()> {
    if view {
        match view_config() {
            Ok(_) => println!(),
            Err(e) => eprintln!("{} {:?}", "An Error occurred".red(), e),
        }
    } else if create {
        match create_default_config() {
            Ok(_) => println!(),
            Err(e) => eprintln!("{} {:?}", "An Error occurred".red(), e),
        }
    } else {
        println!(
            "No action specified. Use --view to view the configuration or --create to create a default configuration file."
        );
    }
    Ok(())
}
