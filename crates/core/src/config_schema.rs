use serde::Deserialize;

pub fn config_defaults() -> &'static str {
    r#"
---
# a configuration file for occ_arch

Flutter:
  projectName: oracle # the default  name of the project if no project name is provided in the args
  architectureType: RiverPod  #only the exits for now, don't change it
  gitInit: true # true if you want git to be initialized in the project false if not
  packages:  # this holds the list of flutter packages that will be installed to the project
    dependencies: #list of flutter packages that will be installed as dependence 
      - envied
      - dio 
      - flutter_riverpod 
      - riverpod_annotation 
      - go_router 
      - flutter_animate 
      - iconly
    devDependencies: # list ofload_str flutter packages that will be installed as dev dependence 
      - envied_generator 
      - riverpod_generator 
      - riverpod_lint custom_lint 
      - build_runner 
      - flutter_native_splash 
      - flutter_launcher_icons
  platforms: # the platform for which the flutter project will be created for
    - android
    - ios
    # - windows
    # - linux
    # - web
#   makeDirs: # create additional folders inside the project 
#     - dir: 
#         name: email
#         files:
#           - name: email.dart
#             content: "this is the content pof the file"
#           - name: send.dart
#     - dir:
#         name: out_mail


ExpressTs:
  projectName: occ_server # the default  name of the project if no project name is provided in the args
  architectureType: MVC  #only the exits for now, don't change it
  gitInit: true # true if you want git to be initialized in the project false if not
  database: mongo_db # the only option that exits for now, don't change it 
  port: 5031
  withDocker: true #highly recommended
  withMakeFile: true # added make file with some useful commands 
  packages:  # this holds the list of nodeJs packages that will be installed to the project
    dependencies: #list of nodeJs packages that will be installed as dependence
      - express
      - bcryptjs
      - body-parser
      - cloudinary
      - cookie-parser
      - cors
      - dotenv
      - express-rate-limit
      - jsonwebtoken
      - mongoose
      - multer
      - nodemailer
      - pug
      - request
      - stripe
      - swagger-jsdoc
      - swagger-ui-express
      - useragent
      - zod
      - zod-validation-error
    devDependencies: # list of nodeJs packages that will be installed as dev dependence 
      - "@types/bcryptjs"
      - "@types/body-parser"
      - "@types/cookie-parser"
      - "@types/cors"
      - "@types/dotenv"
      - "@types/express"
      - "@types/express-rate-limit"
      - "@types/jsonwebtoken"
      - "@types/mongoose"
      - "@types/multer"
      - "@types/nodemailer"
      - "@types/pug"
      - "@types/request"
      - "@types/swagger-jsdoc"
      - "@types/swagger-ui-express"
      - "@types/useragent"
      - "nodemon"
      - "pug-cli"
      - "ts-node"
      - "typescript"
      - "eslint"
  # makeDirs: # create additional folders inside the project 
  #   - dir: 
  #       name: o_email
  #       files:
  #         - name: email.ts
  #           content: "//this is the content of the file"
  #         - name: send.ts
  #   - dir:
  #       name: out_mail
    "#
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "Flutter")]
    pub flutter: Option<FlutterConfig>,

    #[serde(rename = "ExpressTs")]
    pub express_ts: Option<ExpressTsConfig>,
}

#[derive(Debug, Deserialize)]
pub struct FlutterConfig {
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "architectureType")]
    pub architecture_type: String,
    #[serde(rename = "gitInit")]
    pub git_init: bool,
    pub packages: FlutterPackages,
    pub platforms: Vec<String>,
    #[serde(default, rename = "makeDirs")]
    pub make_dirs: Option<Vec<MakeDir>>,
}
#[derive(Debug, Deserialize)]
pub struct FlutterPackages {
    pub dependencies: Vec<String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct ExpressTsConfig {
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "architectureType")]
    pub architecture_type: String,
    #[serde(rename = "gitInit")]
    pub git_init: bool,
    pub database: String,
    pub port: u16,
    #[serde(rename = "withDocker")]
    pub with_docker: bool,
    #[serde(rename = "withMakeFile")]
    pub with_make_file: bool,
    pub packages: NodePackages,

    #[serde(default, rename = "makeDirs")]
    pub make_dirs: Option<Vec<MakeDir>>,
}
#[derive(Debug, Deserialize)]
pub struct NodePackages {
    pub dependencies: Vec<String>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct MakeDir {
    pub dir: DirSpec,
}

#[derive(Debug, Deserialize)]
pub struct DirSpec {
    pub name: String,

    #[serde(default)]
    pub files: Vec<FileSpec>,
}

#[derive(Debug, Deserialize)]
pub struct FileSpec {
    pub name: String,
    pub content: String,
}
