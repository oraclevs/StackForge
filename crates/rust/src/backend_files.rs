pub struct BackendFiles {
    name: String,
}

impl BackendFiles {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    fn root_cargo_toml(&self) -> String {
        r#"
[workspace]
members = [
  "crates/domain",
  "crates/api",
  "crates/infra",
  "crates/shared",
  "services/monolith",
  "services/billing-service",
  "services/auth-service"
]
resolver = "2"
"#
        .to_string()
    }
    fn git_ignore(&self) -> String {
        r#"
# Rust
/target/
**/*.rs.bk
Cargo.lock

# IDEs
.vscode/
.idea/
*.swp

# Environment
.env

# Docker
docker/.env
docker/*.log

# OS
.DS_Store
Thumbs.db

    "#
        .to_string()
    }

    fn make_file(&self) -> String {
        r#"
# Build all binaries
build:
	cargo build --release

# Run monolith
run:
	cargo run -p monolith

# Run a specific service (replace SERVICE with auth-service, billing-service, etc.)
run-service:
	cargo run -p $(SERVICE)

# Run Docker compose
docker-up:
	docker compose up --build

docker-down:
	docker compose down

# Format and lint
fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Clean project
clean:
	cargo clean

        "#
        .to_string()
    }
    fn docker_file(&self) -> String {
        r#"
# --- Stage 1: Build ---
FROM rust:1.75 as builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY services ./services
RUN cargo build --release

# --- Stage 2: Minimal runtime ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary (example for monolith)
COPY --from=builder /app/target/release/monolith /usr/bin/monolith

EXPOSE 8080
CMD ["monolith"]

    "#
        .to_string()
    }

    fn docker_compose(&self) -> String {
        format!(
            r#"
version: "3.9"

services:
  api:
    build: .
    container_name: {name}
    ports:
      - "8080:8080"
    env_file:
      - ../.env
    depends_on:
      - db

  db:
    image: postgres:16
    container_name: {name}_db
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: {name}db
    volumes:
      - db_data:/var/lib/postgresql/data

volumes:
  db_data:

    "#,
            name = self.name
        )
    }

    fn docker_ignore(&self) -> String {
        r#"
# Rust build artifacts
target/
**/*.rs.bk

# Environment files
.env

# Git
.git
.gitignore

# IDE files
.vscode/
.idea/
*.swp

"#
        .to_string()
    }
    fn config_base(&self) -> String {
        format!(
            r#"
# Base configuration shared across all environments
[server]
port = 8080

[database]
url = "postgres://postgres:postgres@db/{}"

[logging]
level = "info"

[auth]
jwt_secret = "replace_me_with_secure_secret"
token_expiration_minutes = 60

    "#,
            self.name
        )
    }
    fn config_dev(&self) -> String {
        format!(
            r#"
# Development environment overrides
[server]
port = 8080

[database]
url = "postgres://postgres:postgres@localhost/{}_dev"

[logging]
level = "debug"

[auth]
jwt_secret = "dev_secret_key"
    "#,
            self.name
        )
    }
    fn config_prod(&self) -> String {
        format!(
            r#"
# Production environment overrides
[server]
port = 8080

[database]
url = "postgres://postgres:${{POSTGRES_PASSWORD}}@db/{}"

[logging]
level = "info"

[auth]
jwt_secret = "${{JWT_SECRET}}"

    "#,
            self.name
        )
    }
    pub fn files(&self) -> Vec<(String, String)> {
        vec![
            ("Cargo.toml".to_string(), self.root_cargo_toml()),
            (".gitignore".to_string(), self.git_ignore()),
            ("Makefile".to_string(), self.make_file()),
            ("docker/Dockerfile".to_string(), self.docker_file()),
            (
                "docker/docker-compose.yaml".to_string(),
                self.docker_compose(),
            ),
            ("docker/.dockerignore".to_string(), self.docker_ignore()),
            ("migrations/0001_init.sql".to_string(), "".to_string()),
            ("config/base.toml".to_string(), self.config_base()),
            ("config/dev.toml".to_string(), self.config_dev()),
            ("config/prod.toml".to_string(), self.config_prod()),
        ]
    }
}
