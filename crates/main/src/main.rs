mod cli;
mod command;
use core::utils::welcome_print;

use clap::Parser;
use cli::{Cli, Commands};

use command::{execute_config_command, execute_init_command};
fn main() {
    let cli = Cli::parse();
    welcome_print();
    match cli.command {
        Commands::Init {
            language,
            name,
            backend,
            git,
            package,
            work_space,
        } => execute_init_command(language, name, backend, git, package, work_space).unwrap(),
        Commands::Config { view, create } => execute_config_command(view, create).unwrap(),

        _ => println!("Other commands not implemented yet"),
    }
}
