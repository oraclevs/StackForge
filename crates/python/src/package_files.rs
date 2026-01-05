pub struct FileContent {
    pub package_name: String,
}

impl FileContent {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }
}
impl FileContent {
    pub fn setup_py(&self) -> String {
        format!(
            r#"from setuptools import setup, find_packages
import os

here = os.path.abspath(os.path.dirname(__file__))
readme_path = os.path.join(here, "{name}", "README.md")

with open(readme_path, "r", encoding="utf-8") as f:
    long_description = f.read()

setup(
    name="{name}",
    version="1.0.0",
    packages=find_packages(),
    include_package_data=True,
    package_data={{ "{name}": ["README.md"] }},
    entry_points={{
        "console_scripts": [
            "{name} = {name}.__init__:main",
        ],
    }},
    install_requires=['pyfiglet','rich'],
    author="Oracle",
    author_email="occ.softdev@gmail.com",
    description="A CLI tool",
    long_description=long_description,
    long_description_content_type="text/markdown",
    classifiers=[
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.6',
)
"#,
            name = self.package_name
        )
    }

    pub fn gitignore(&self) -> &'static str {
        r#"build
dist
.venv
__pycache__
*.pyc
"#
    }

    pub fn manifest_in(&self) -> String {
        format!(
            r#"recursive-include {} *.md
"#,
            self.package_name
        )
    }

    pub fn pyproject_toml(&self) -> &'static str {
        r#"[build-system]
requires = [
    "setuptools",
    "wheel",
    "rich",
]
build-backend = "setuptools.build_meta"
"#
    }

    pub fn readme(&self) -> String {
        format!(
            r#"# {name}
**{name}** is a powerful Python CLI tool developed by Oracle
"#,
            name = self.package_name
        )
    }

    pub fn package_init_file(&self) -> &'static str {
        r#"#!/usr/bin/env python3
# Placeholder __init__.py
"#
    }

    pub fn list_all_files(&self) -> Vec<(String, String)> {
        vec![
            ("setup.py".to_string(), self.setup_py()),
            (".gitignore".to_string(), self.gitignore().to_string()),
            ("MANIFEST.in".to_string(), self.manifest_in()),
            (
                "pyproject.toml".to_string(),
                self.pyproject_toml().to_string(),
            ),
            (format!("{}/README.md", self.package_name), self.readme()),
            (
                format!("{}/__init__.py", self.package_name),
                self.package_init_file().to_string(),
            ),
        ]
    }

    pub fn default_main_file(&self) -> Vec<(String, String)> {
        vec![
            (
                String::from("src/main.py"),
                r#"def main():
    print("Hello, World!")
"#
                .to_string(),
            ),
            (String::from("requirements.txt"), String::new()),
        ]
    }
}
