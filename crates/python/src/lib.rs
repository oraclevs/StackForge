mod backend;
mod default;
mod package;
mod package_files;
mod  backend_files;
mod utils;
use colored::Colorize;

pub fn python(name: &str, backend: bool, git: bool, package: bool) -> anyhow::Result<()> {
   
    if backend {
        backend::backend(name, git)?;
        return Ok(());
    }
    if package {
        package::package(name, git)?;
    } else {
        default::default(name, git)?;
    }
    Ok(())
}
