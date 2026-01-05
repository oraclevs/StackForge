use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize,Serialize)]
pub struct PackageJsonFile {
    name: String,
    version: String,
    description: String,
    main: String,
    r#type: String,
    scripts: Scripts,
    license: String,
}

#[derive(Debug, Deserialize,Serialize)]
struct Scripts {
    build: String,
    test: String,
    dev: String,
}

impl PackageJsonFile {
    pub fn new(
        name: String,
        version: String,
        description: String,
        main: String,
        r#type: String,
        script_build: String,
        script_test: String,
        script_dev: String,
        license: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            main,
            r#type,
            scripts: Scripts {
                build: script_build,
                test: script_test,
                dev: script_dev,
            },
            license,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TsConfig {
    #[serde(rename = "compilerOptions")]
    pub compiler_options: CompilerOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerOptions {
    pub module: String,
    pub target: String,
    pub types: Vec<String>,

    #[serde(rename = "sourceMap")]
    pub source_map: bool,

    pub declaration: bool,

    #[serde(rename = "declarationMap")]
    pub declaration_map: bool,

    #[serde(rename = "noUncheckedIndexedAccess")]
    pub no_unchecked_indexed_access: bool,

    #[serde(rename = "exactOptionalPropertyTypes")]
    pub exact_optional_property_types: bool,

    pub strict: bool,
    pub jsx: String,

    #[serde(rename = "verbatimModuleSyntax")]
    pub verbatim_module_syntax: bool,

    #[serde(rename = "isolatedModules")]
    pub isolated_modules: bool,

    #[serde(rename = "noUncheckedSideEffectImports")]
    pub no_unchecked_side_effect_imports: bool,

    #[serde(rename = "moduleDetection")]
    pub module_detection: String,

    #[serde(rename = "skipLibCheck")]
    pub skip_lib_check: bool,

    #[serde(rename = "moduleResolution")]
    pub module_resolution: String,

    #[serde(rename = "esModuleInterop")]
    pub es_module_interop: bool,

    #[serde(rename = "allowSyntheticDefaultImports")]
    pub allow_synthetic_default_imports: bool,

    #[serde(rename = "resolveJsonModule")]
    pub resolve_json_module: bool,

    #[serde(rename = "rootDir")]
    pub root_dir: String,

    #[serde(rename = "outDir")]
    pub out_dir: String,

    #[serde(rename = "noImplicitAny")]
    pub no_implicit_any: bool,

    #[serde(rename = "strictNullChecks")]
    pub strict_null_checks: bool,

    #[serde(rename = "strictFunctionTypes")]
    pub strict_function_types: bool,

    #[serde(rename = "alwaysStrict")]
    pub always_strict: bool,

    #[serde(rename = "noUnusedLocals")]
    pub no_unused_locals: bool,
}
