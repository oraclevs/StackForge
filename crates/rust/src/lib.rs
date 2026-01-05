mod backend_files;
mod backend_setup;
mod setup;
mod utils;

use backend_setup::backend_init;
use setup::{default_project, work_space_init};

pub fn rust(
    name: &str,
    backend: bool,
    git: bool,
    _package: bool,
    work_space: Vec<String>,
) -> anyhow::Result<()> {
    if backend {
        backend_init(name, git)?;
        return Ok(());
    }
    if !work_space.is_empty() {
        work_space_init(&work_space, git, &name)?;
        return Ok(());
    } else {
        default_project(&name, git)?;
    }
    Ok(())
}
