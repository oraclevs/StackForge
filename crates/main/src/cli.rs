use crate::command::Language;
use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(
    name = "occ_arch",
    version = "1.0.0",
    about = "A cross-platform project scaffold builder",
    long_about = "occ_arch - project scaffold builder"
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        language: Language,
        name: String,
        #[arg(
            long,
            default_value_t = false,
            help = "Create a virtual environment (only for Python)"
        )]
        #[arg(
            long,
            default_value_t = false,
            help = "Create a backend architecture(only for python,rust,typescript)"
        )]
        backend: bool,
        #[arg(long, default_value_t = false, help = "Initialize a git repository")]
        git: bool,
        #[arg(long, default_value_t = false, help = "Create as a Python package")]
        package: bool,
        #[arg(
    long,
    help = "Create a rust workspace",
    num_args = 1..,
    value_name = "CRATE",
)]
        work_space: Vec<String>,
    },
    Upgrade {
        #[arg(
            long,
            default_value_t = false,
            help = "Upgrade all packages to the latest version"
        )]
        packages: bool,
        #[arg(
            long,
            default_value_t = false,
            help = "Upgrades the framework or language version"
        )]
        framework: bool,
    },
    Config {
        #[arg(long, help = "View the current configuration")]
        view: bool,
        #[arg(long, help = "Create a default configuration file")]
        create: bool,
    },
    List {
        #[arg(long, help = "List available templates")]
        templates: bool,

        #[arg(long, help = "List supported languages")]
        languages: bool,
    },
}
