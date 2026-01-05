pub mod config_schema;
pub mod utils;
use colored::Colorize;
use config_schema::{Config, config_defaults};
use directories::ProjectDirs;
use serde_yaml;
use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

// read the config from a file in the config dir
// create a file with the default config and save it in the config dir if not there
// create some flags to view or edit the config

fn config_path() -> Option<(PathBuf, String)> {
    let proj = ProjectDirs::from("com", "stackforge", "stackforge")?;
    Some((
        proj.config_dir().join("config.yaml"),
        proj.config_dir().to_str()?.to_string(),
    ))
}

pub fn view_config() -> anyhow::Result<()> {
    let config_yaml = read_config_from_file();
    match config_yaml {
        Ok(cfg) => println!("{}: {:#?}", "Current Configuration".yellow(), cfg),
        Err(_) => {
            println!(
                "{}",
                "Could not read config file, using default configuration.".yellow()
            );
            println!(
                "{}",
                "To create a config file, run `stackforge config --create`".yellow()
            );
        }
    }
    Ok(())
}

pub fn create_default_config() -> anyhow::Result<()> {
    let config_yaml = config_defaults();
    // set the path to the config file
    let config_path = config_path();
    match config_path {
        Some((path, dir)) => {
            if path.exists() {
                println!(
                    "{} {}",
                    "Configuration file already exists at".yellow(),
                    path.to_str().unwrap().blue()
                );
                return Ok(());
            }
            create_dir_all(Path::new(&dir))?;
            let mut file = File::create(&path)?;
            file.write_all(&config_yaml.as_bytes())?;
            println!(
                "{} {}",
                "Default configuration file created at".green(),
                path.to_str().unwrap().blue()
            );
        }
        None => {
            println!("{}", "Could not determine config path".red());
            return Ok(());
        }
    }
    Ok(())
}

pub fn read_config_from_file() -> anyhow::Result<Config> {
    let config_path: (PathBuf, String) =
        config_path().ok_or_else(|| anyhow::anyhow!("Could not determine config path"))?;
    let file = File::open(config_path.0)?;
    let config: Config = serde_yaml::from_reader(file)?;
    Ok(config)
}
