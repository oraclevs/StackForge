use anyhow::Ok;
use colored::Colorize;
use std::process::{Command, ExitStatus, Stdio};

pub fn create_ws_libs(work_space_names: &Vec<String>) -> anyhow::Result<()> {
    for lib_name in work_space_names.iter() {
        let lib_create_cmd: ExitStatus = Command::new("cargo")
            .arg("new")
            .arg(lib_name)
            .args(["--vcs", "none", "--lib"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if lib_create_cmd.success() {
            println!(
                "{}::{}",
                "Library has been created".yellow(),
                lib_name.blue()
            )
        } else {
            eprintln!("{}", "Failed to create library")
        }
    }
    Ok(())
}
pub fn create_ws_bin(name: &str) -> anyhow::Result<()> {
    let bin_create_command = Command::new("cargo")
        .args(["new", &name, "--vcs", "none"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if bin_create_command.success() {
        println!("{}::{}", "Binary has been created".yellow(), &name.green());
    } else {
        eprintln!("{}", "Failed to create binary".red())
    }
    Ok(())
}

pub fn workspace_toml_content(work_space_names: &Vec<String>) -> String {
    format!(
        r#"
[workspace]
resolver = "3"
members = [
    {work_space}
]
"#,
        work_space = work_space_names
            .iter()
            .map(|name| format!("\"crates/{}\"", name))
            .collect::<Vec<String>>()
            .join(",\n    ")
    )
}
