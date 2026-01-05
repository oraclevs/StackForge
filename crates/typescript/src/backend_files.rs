use crate::config_json_models::{CompilerOptions, PackageJsonFile, TsConfig};
pub struct BackendFiles {
    name: String,
    port: usize,
}

impl BackendFiles {
    pub fn new(name: String, port: usize) -> Self {
        Self { name, port }
    }
    fn app_ts_file(&self) -> String {
        r#"
        
import dotenv from 'dotenv';
import express from 'express';
import type { Express, Request, Response } from 'express';

const env_path = process.env.NODE_ENV === 'production' ? '.env.prod' : '.env.dev';
dotenv.config({ path: env_path });

const PORT: number = parseInt(process.env.PORT as string, 10) || 5001;

const app: Express = express();

// Server Start up
app.get('/', (req: Request, res: Response) => {
    res.send('Welcome to Home page');
});

app
    .listen(PORT, () => {
        console.log(`running on port ${PORT}`);
    })
    .on('error', (error: Error) => console.log(error.message));

        "#
        .to_string()
    }

    fn production_docker_file(&self) -> String {
        r#"
# ---------- STAGE 1: BUILD ----------
    FROM node:24.4-bullseye-slim AS build

    # Install dependencies for native module builds
    RUN apt-get update && apt-get install -y \
        python3 \
        make \
        g++ \
     && apt-get clean \
     && rm -rf /var/lib/apt/lists/*
    
    WORKDIR /usr/src/app
    
    COPY package*.json tsconfig.json ./
    
    # Cache npm install results
    RUN --mount=type=cache,target=/usr/src/app/.npm \
      npm set cache /usr/src/app/.npm && \
      npm install
    
    COPY ./src ./src
    
    RUN npm run build

    
    # ---------- STAGE 2: PRODUCTION ----------
    FROM node:24.4-bullseye-slim AS production
    
    ENV NODE_ENV=production
    
    WORKDIR /usr/src/app
    
    COPY --from=build /usr/src/app/package*.json ./
    
    # Install only production dependencies
    RUN --mount=type=cache,target=/usr/src/app/.npm \
      npm set cache /usr/src/app/.npm && \
      npm ci --only=production
    
    # Use non-root user
    USER node
    
    # Copy the final build output
    COPY --chown=node:node --from=build /usr/src/app/src/dist ./

    # Run the application
    CMD ["node", "app.js"]
    
    EXPOSE 5000
    
    "#
        .to_string()
    }

    fn dev_docker_file(&self) -> String {
        r#"
FROM node:24.4-bullseye-slim

WORKDIR /usr/src/app

COPY package*.json ./
RUN npm install

COPY . .

CMD ["npm", "run", "dev"]
    "#
        .to_string()
    }

    fn dev_docker_compose_file(&self) -> String {
        format!(
            r#"
version: '3.8'

services:
  app:
    container_name: {project_name}-express-ts-dev
    build:
      context: .
      dockerfile: Dockerfile.dev
    volumes:
      - .:/usr/src/app
      - /usr/src/app/node_modules
    ports:
      - "{port}:{port}"
    depends_on:
      - mongo
    environment:
      - NODE_ENV=development
      - MONGO_URI=mongodb://mongo:27017/dev-db

  mongo:
    image: mongo:7
    container_name: {project_name}mongo-dev
    ports:
      - "27017:27017"
    volumes:
      - mongo_data_dev:/data/db
      # - ./mongo:/docker-entrypoint-initdb.d
    healthcheck:
      test: echo 'db.runCommand("ping").ok' | mongosh localhost:27017/myapp --quiet
      interval: 10s
      timeout: 5s
      retries: 5


volumes:
  mongo_data_dev:

    "#,
            project_name = self.name,
            port = self.port
        )
    }

    fn prod_docker_compose_file(&self) -> String {
        format!(
            r#"
        version: '3.8'

services:
  app:
    container_name: {project_name}-express-ts-prod
    build:
      context: .
      dockerfile: Dockerfile.prod
    ports:
      - "5000:5000"
    depends_on:
      - mongo
    env_file:
      - .env.prod
  mongo:
      image: mongo:7
      container_name: {project_name}mongo-prod
      ports:
        - "27017:27017"
      volumes:
        - mongo_data_prod:/data/db
        # - ./mongo:/docker-entrypoint-initdb.d
      healthcheck:
        test: echo 'db.runCommand("ping").ok' | mongosh localhost:27017/myapp --quiet
        interval: 10s
        timeout: 5s
        retries: 5
volumes:
  mongo_data_prod:

        "#,
            project_name = self.name,
        )
    }

    fn make_file(&self) -> String {
        r#"
# Makefile

.DEFAULT_GOAL := help


.PHONY: help up-dev up-prod down-dev down-prod logs-dev logs-prod
up-dev: ## Start dev environment
	docker compose -f docker-compose.dev.yml up --build

up-prod: ## Start prod environment
	docker compose -f docker-compose.prod.yml up --build

down-dev: ## Stop dev environment
	docker compose -f docker-compose.dev.yml down

down-prod: ## Stop prod environment
	docker compose -f docker-compose.prod.yml down

logs-dev: ## Tail logs in dev
	docker compose -f docker-compose.dev.yml logs -f

logs-prod: ## Tail logs in prod
	docker compose -f docker-compose.prod.yml logs -f

help: ## Show help
	@echo "Usage:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

    "#.to_string()
    }
    fn gitignore(&self) -> String {
        r#"
    src/Email/Template
tsconfig.tsbuildinfo

# Dependency directories
node_modules/
bower_components/

# Build outputs
dist/
build/
coverage/

# Logs and debug files
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*

# OS/system files
.DS_Store
Thumbs.db

# Environment variables
.env
.env.*
*.env

# Docker-related
docker-compose.override.yml
mongo/data/
mongo/init.js (optional, if it contains secrets)

# Editor & IDE config
.vscode/
.idea/
*.swp

# TypeScript cache
*.tsbuildinfo

# Optional: project-local database files
*.sqlite
*.sqlite3

# Optional: lock file consistency (if using only one)
# If using npm
# yarn.lock
# If using yarn
# package-lock.json
    "#
        .to_string()
    }

    fn docker_ignore_file(&self) -> String {
        r#"
    tsconfig.tsbuildinfo
# Node dependencies
node_modules
npm-debug.log
yarn-error.log

# Build cache / dist files (only needed if not explicitly copying)
dist

# Env files (avoid baking secrets into image)
.env
.env.*
*.env

# Editor / system files
.vscode
.idea
.DS_Store

# Git
.git
.gitignore
    "#
        .to_string()
    }
    fn dev_env_file(&self) -> String {
        format!(
            r#"
PORT={}
NODE_ENV=development
MONGO_URI=mongodb://mongo:27017/dev-db
"#,
            self.port,
        )
    }
    fn prod_env_file(&self) -> String {
        format!(
            r#"
PORT={}
NODE_ENV=production
MONGO_URI=mongodb://mongo:27017/prod-db
"#,
            self.port,
        )
    }
    pub fn compiler_options_file(&self) -> TsConfig {
        TsConfig {
            compiler_options: CompilerOptions {
                module: "NodeNext".to_string(),
                target: "ES2022".to_string(),
                types: vec![],
                source_map: true,
                declaration: true,
                declaration_map: true,
                no_unchecked_indexed_access: true,
                exact_optional_property_types: true,
                strict: true,
                jsx: "react-jsx".to_string(),
                verbatim_module_syntax: true,
                isolated_modules: true,
                no_unchecked_side_effect_imports: true,
                module_detection: "force".to_string(),
                skip_lib_check: true,
                module_resolution: "NodeNext".to_string(),
                es_module_interop: true,
                allow_synthetic_default_imports: true,
                resolve_json_module: true,
                root_dir: "./src".to_string(),
                out_dir: "./dist".to_string(),
                no_implicit_any: true,
                strict_null_checks: true,
                strict_function_types: true,
                always_strict: true,
                no_unused_locals: true,
            },
        }
    }

    pub fn init_package_json_file(&self) -> PackageJsonFile {
        PackageJsonFile::new(
            self.name.to_string(),
            "1.0.0".to_string(),
            String::new(),
            "app.js".to_string(),
            "module".to_string(),
            " tsc --build".to_string(),
            "echo \"Error: no test specified\" && exit 1".to_string(),
            "nodemon --watch src --exec node --loader ts-node/esm src/app.ts".to_string(),
            "ISC".to_string(),
        )
    }

    pub fn files(&self) -> Vec<(String, String)> {
        vec![
            ("src/app.ts".to_string(), self.app_ts_file()),
            (".gitignore".to_string(), self.gitignore()),
            (".env.dev".to_string(), self.dev_env_file()),
            (".env.prod".to_string(), self.prod_env_file()),
            ("Makefile".to_string(), self.make_file()),
            ("Dockerfile.prod".to_string(), self.production_docker_file()),
            ("Dockerfile.dev".to_string(), self.dev_docker_file()),
            (
                "docker-compose.dev.yml".to_string(),
                self.dev_docker_compose_file(),
            ),
            (
                "docker-compose.prod.yml".to_string(),
                self.prod_docker_compose_file(),
            ),
            (".dockerignore".to_string(), self.docker_ignore_file()),
        ]
    }
}
