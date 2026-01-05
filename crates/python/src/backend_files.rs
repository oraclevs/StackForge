use clap::builder::Str;

pub struct BackendFiles {
    name: String,
}

impl BackendFiles {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    fn app_init(&self) -> String {
        String::new()
    }
    fn config_base(&self) -> String {
        format!(
            r#"
# Base settings shared by all environments

APP_NAME={name}
APP_ENV=base

# Server
HOST=0.0.0.0
PORT=8080

# Logging
LOG_LEVEL=info

# Database (default, can be overridden)
DATABASE_URL=postgresql+asyncpg://postgres:postgres@db:5432/{name}

# Auth
JWT_ALGORITHM=HS256
JWT_EXPIRE_MINUTES=60

        "#,
            name = self.name
        )
    }
    fn config_dev(&self) -> String {
        format!(
            r#"
# Development overrides

APP_ENV=dev

# Logging
LOG_LEVEL=debug

# Database (local/dev)
DATABASE_URL=postgresql+asyncpg://postgres:postgres@localhost:5432/{}_dev

# Auth (development only)
JWT_SECRET=dev_secret_change_me

    "#,
            self.name
        )
    }
    fn config_prod(&self) -> String {
        format!(
            r#"
# Production overrides

APP_ENV=prod

# Logging
LOG_LEVEL=info

# Database (production)
DATABASE_URL=postgresql+asyncpg://postgres:${{POSTGRES_PASSWORD}}@db:5432/{}

# Auth (must be injected securely)
JWT_SECRET=${{JWT_SECRET}}

        "#,
            self.name
        )
    }
    fn docker_file(&self) -> String {
        r#"
FROM python:3.12-slim

# System deps
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install Python dependencies
COPY pyproject.toml .
RUN pip install --no-cache-dir --upgrade pip \
    && pip install --no-cache-dir .

# Copy application
COPY app ./app

EXPOSE 8080

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8080"]

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
    container_name: {name}_api
    ports:
      - "8080:8080"
    env_file:
      - ../config/dev.env
    depends_on:
      - db

  db:
    image: postgres:16
    container_name: {name}_db
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: {name}_dev
    volumes:
      - db_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  db_data:

    "#,
            name = self.name
        )
    }
    fn py_project_toml(&self) -> String {
        format!(
            r#"
[project]
name = "{}"
version = "0.1.0"
description = "FastAPI backend scaffold (monolith → microservices)"
requires-python = ">=3.11"

dependencies = [
  "fastapi>=0.110",
  "uvicorn[standard]>=0.27",
  "pydantic>=2.6",
  "pydantic-settings>=2.2",
  "sqlalchemy>=2.0",
  "asyncpg>=0.29",
  "python-jose>=3.3",
  "passlib[bcrypt]>=1.7",
]

[tool.black]
line-length = 88

[tool.ruff]
select = ["E", "F", "I"]
fix = true

        "#,
            self.name
        )
    }
    fn make_file(&self) -> String {
        r#"
.PHONY: run dev docker-up docker-down lint format clean

# Run locally (expects virtualenv)
run:
	uvicorn app.main:app --reload --host 0.0.0.0 --port 8080

# Run with dev env
dev:
	export $$(cat config/dev.env | xargs) && make run

# Docker
docker-up:
	docker compose up --build

docker-down:
	docker compose down

# Formatting & linting
format:
	black app
	ruff check app --fix

lint:
	ruff check app

# Cleanup
clean:
	find . -type d -name "__pycache__" -exec rm -rf {} +

        "#
        .to_string()
    }
    pub fn files(&self) -> Vec<(String, String)> {
        vec![
            ("app/__init__.py".to_string(), self.app_init()),
            ("config/base.env".to_string(), self.config_base()),
            ("config/dev.env".to_string(), self.config_dev()),
            ("config/prod.env".to_string(), self.config_prod()),
            ("docker/Dockerfile".to_string(), self.docker_file()),
            (
                "docker/docker-compose.yaml".to_string(),
                self.docker_compose(),
            ),
            ("pyproject.toml".to_string(), self.py_project_toml()),
            ("Makefile".to_string(), self.make_file()),
        ]
    }
}
