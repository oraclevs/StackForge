
# 🧱 StackForge

**StackForge** is a cross-platform, configuration-driven project scaffold builder written in **Rust**.

It helps you initialize consistent, production-ready project structures across multiple languages and frameworks — with templates, defaults, and automation you control.

StackForge is designed for developers who value **structure, repeatability, and speed**.

## ✨ Features

* 🚀 Initialize projects for multiple languages and stacks
* 🧩 Config-driven behavior using a YAML configuration file
* 🛠 Optional Git initialization
* 📦 Package & dependency bootstrapping
* 🧱 Backend architecture scaffolding (where supported)
* 🦀 Rust-powered: fast, safe, and cross-platform
* 🖥 Works on **Linux, macOS, and Windows**

## 📦 Installation

### Build from source (recommended)

```bash
git clone https://github.com/oraclevs/stackforge.git
cd stackforge
cargo build --release
```

Copy the binary:

```bash
sudo cp target/release/occ_arch /usr/local/bin/stackforge
```

## 🚀 Usage

StackForge is a **CLI-first tool**.

```bash
stackforge <COMMAND> [OPTIONS]
```

## 🧱 Commands Overview

### `init`

Create a new project scaffold.

```bash
stackforge init <LANGUAGE> <NAME> [OPTIONS]
```

#### Options

| Flag                      | Description                                                 |
| ------------------------- | ----------------------------------------------------------- |
| `--git`                   | Initialize a Git repository                                 |
| `--package`               | Create as a Python package                                  |
| `--backend`               | Create backend architecture (Python, Rust, TypeScript only) |
| `--work-space <CRATE...>` | Create a Rust workspace with multiple crates                |
| `--virtual-env`           | Create a Python virtual environment                         |

#### Examples

```bash
# Create a Python project
stackforge init python my_app --git

# Create a Python package with backend structure
stackforge init python api_server  --backend

# Create a Rust workspace
stackforge init rust core --work-space auth api db

# Create a TypeScript backend project
stackforge init typescript server --backend --git
```

### `upgrade`

Upgrade packages or frameworks.

```bash
stackforge upgrade [OPTIONS]
```

#### Options

| Flag          | Description                           |
| ------------- | ------------------------------------- |
| `--packages`  | Upgrade project packages              |
| `--framework` | Upgrade framework or language version |

Example:

```bash
stackforge upgrade --packages
```

### `config`

Manage StackForge configuration.

```bash
stackforge config [OPTIONS]
```

#### Options

| Flag       | Description                         |
| ---------- | ----------------------------------- |
| `--view`   | View the current configuration      |
| `--create` | Create a default configuration file |

Example:

```bash
stackforge config --create
stackforge config --view
```

### `list`

List supported features.

```bash
stackforge list [OPTIONS]
```

#### Options

| Flag          | Description              |
| ------------- | ------------------------ |
| `--languages` | List supported languages |
| `--templates` | List available templates |

Example:

```bash
stackforge list --languages
```

## ⚙️ Configuration

StackForge behavior is controlled via a **YAML configuration file**.

### Config file location

The configuration file is stored in the OS-specific config directory:

| OS      | Path                                                         |
| ------- | ------------------------------------------------------------ |
| Linux   | `~/.config/stackforge/stackforge/config.yaml`                         |
| macOS   | `~/Library/Application Support/com.stackforge.StackForge/config.yaml` |
| Windows | `%APPDATA%\stackforge\stackforge\config.yaml`                         |

You can edit this file manually or via CLI flags.

## 🧠 Philosophy

StackForge follows a few core ideas:

* **Configuration over hard-coding**
* **Explicit structure beats magic**
* **Scaffolding should reflect how *you* actually build projects**
* **Rust for correctness and long-term maintainability**

## 🛣 Roadmap (non-binding)

Based on typical scaffold tool evolution patterns.

* Plugin system for custom templates
* Interactive mode
* More language targets (Go, Java, C#)
* Template marketplace
* Built-in Makefile / Docker presets

## 🧑‍💻 Development

```bash
cargo run -- <COMMAND>
```

Example:

```bash
cargo run -- init rust demo --git
```

## 📜 License

MIT License.
