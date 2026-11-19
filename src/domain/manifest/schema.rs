use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

/* ============================================================
   Root
   ============================================================ */

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RawManifest {
    pub kiln: KilnConfig,

    pub project: ProjectConfig,

    #[serde(default)]
    pub workspace: Option<WorkspaceConfig>,

    #[serde(default)]
    pub build: Option<BuildConfig>,

    #[serde(default)]
    pub profiles: HashMap<String, ProfileConfig>,

    #[serde(default)]
    pub toolchains: Option<ToolchainsConfig>,

    /// Required to be non-empty by *semantic* validation (§2).
    #[serde(default)]
    pub targets: HashMap<String, Target>,

    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,

    #[serde(default)]
    pub tests: HashMap<String, Test>,
}

/* ============================================================
   [kiln] / [project] / [workspace] / [build]
   ============================================================ */

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct KilnConfig {
    pub schema: u8,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub authors: Option<Vec<String>>,
    #[serde(default)]
    pub default_target: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub build_dir: Option<PathBuf>,
    #[serde(default)]
    pub output_dir: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BuildConfig {
    #[serde(default)]
    pub default_profile: Option<String>,
    #[serde(default)]
    pub jobs: Option<u32>,
    #[serde(default)]
    pub warnings: Option<WarningLevel>,
}

/* ============================================================
   Profiles
   ============================================================ */

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProfileConfig {
    #[serde(default)]
    pub inherits: Option<String>,
    #[serde(default)]
    pub optimization: Option<Optimization>,
    #[serde(default)]
    pub debug_info: Option<bool>,
    #[serde(default)]
    pub assertions: Option<bool>,
    #[serde(default)]
    pub lto: Option<bool>,
}

/* ============================================================
   Toolchains
   ============================================================ */

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToolchainsConfig {
    #[serde(default)]
    pub native: Option<NativeToolchain>,
    #[serde(default)]
    pub rust: Option<RustToolchain>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NativeToolchain {
    /// "cmake" in schema 1.
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub generator: Option<String>,
    #[serde(default)]
    pub compilers: Option<Compilers>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Compilers {
    #[serde(default)]
    pub c: Option<PathBuf>,
    #[serde(default)]
    pub cpp: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RustToolchain {
    /// "cargo" in schema 1.
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub toolchain: Option<String>,
}

/* ============================================================
   Targets
   ============================================================ */

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Target {
    #[serde(rename = "type")]
    pub target_type: TargetType,

    /// Omitted for `interface` targets; semantic validation enforces presence
    /// for binary targets.
    #[serde(default)]
    pub language: Option<Language>,

    #[serde(default)]
    pub sources: Option<Vec<PathBuf>>,

    /// Required when `language = "rust"` (validated semantically).
    #[serde(default)]
    pub manifest: Option<PathBuf>,

    #[serde(default)]
    pub cargo_target: Option<String>,
    #[serde(default)]
    pub cargo_features: Option<Vec<String>>,

    #[serde(default)]
    pub include_dirs: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub public_include_dirs: Option<Vec<PathBuf>>,

    #[serde(default)]
    pub dependencies: Option<Vec<String>>,
    #[serde(default)]
    pub public_dependencies: Option<Vec<String>>,

    #[serde(default)]
    pub definitions: Option<Vec<String>>,
    #[serde(default)]
    pub public_definitions: Option<Vec<String>>,

    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub output_name: Option<String>,

    /// Native-only sub-table; rejected for Rust/interface targets semantically.
    #[serde(default)]
    pub compile: Option<CompileConfig>,

    /// Executable-only sub-table; rejected otherwise semantically.
    #[serde(default)]
    pub run: Option<RunConfig>,

    /// Platform overlays: `windows`, `linux`, `macos`,
    /// `windows-x86_64`, `linux-aarch64`, etc.
    #[serde(default)]
    pub platforms: HashMap<String, TargetOverlay>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct TargetOverlay {
    #[serde(default)]
    pub sources: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub include_dirs: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub public_include_dirs: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub dependencies: Option<Vec<String>>,
    #[serde(default)]
    pub public_dependencies: Option<Vec<String>>,
    #[serde(default)]
    pub definitions: Option<Vec<String>>,
    #[serde(default)]
    pub public_definitions: Option<Vec<String>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub output_name: Option<String>,
    #[serde(default)]
    pub compile: Option<CompileConfig>,
    #[serde(default)]
    pub run: Option<RunConfig>,
    // `link` is a dependency-overlay scalar, not a target field.
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CompileConfig {
    #[serde(default)]
    pub standard: Option<String>,
    #[serde(default)]
    pub optimization: Option<Optimization>,
    #[serde(default)]
    pub warnings: Option<WarningLevel>,
    #[serde(default)]
    pub pic: Option<bool>,
    #[serde(default)]
    pub c_flags: Option<Vec<String>>,
    #[serde(default)]
    pub cpp_flags: Option<Vec<String>>,
    #[serde(default)]
    pub asm_flags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RunConfig {
    #[serde(default)]
    pub working_directory: Option<PathBuf>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

/* ============================================================
   External Dependencies
   ============================================================ */

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    pub provider: DependencyProvider,

    #[serde(default)]
    pub enabled: Option<bool>,

    #[serde(default)]
    pub include_dirs: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub definitions: Option<Vec<String>>,
    #[serde(default)]
    pub link_libraries: Option<Vec<String>>,
    #[serde(default)]
    pub environment: HashMap<String, String>,

    /* --- provider = "cmake" --- */
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub options: HashMap<String, TomlScalar>,
    #[serde(default)]
    pub cmake: Option<CmakeExtras>,

    /* --- provider = "prebuilt" --- */
    #[serde(default)]
    pub linkage: Option<Linkage>,
    #[serde(default)]
    pub link: Option<PathBuf>,
    #[serde(default)]
    pub runtime: Option<Vec<PathBuf>>,

    /* --- provider = "system" --- */
    #[serde(default)]
    pub method: Option<SystemMethod>,
    #[serde(default)]
    pub package: Option<String>,
    #[serde(default)]
    pub frameworks: Option<Vec<String>>,

    /// `[dependencies.<name>.platforms.<selector>]` overlays.
    #[serde(default)]
    pub platforms: HashMap<String, DependencyOverlay>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct DependencyOverlay {
    #[serde(default)]
    pub link: Option<PathBuf>,
    #[serde(default)]
    pub runtime: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub include_dirs: Option<Vec<PathBuf>>,
    #[serde(default)]
    pub definitions: Option<Vec<String>>,
    #[serde(default)]
    pub link_libraries: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<HashMap<String, TomlScalar>>,
    #[serde(default)]
    pub environment: Option<HashMap<String, String>>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub package: Option<String>,
    #[serde(default)]
    pub frameworks: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CmakeExtras {
    #[serde(default)]
    pub configure_args: Option<Vec<String>>,
}

/// §8.2: options may be strings, integers, or booleans.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TomlScalar {
    Bool(bool),
    Integer(i64),
    String(String),
}

/* ============================================================
   Tests
   ============================================================ */

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Test {
    pub target: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub working_directory: Option<PathBuf>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

/* ============================================================
   Enums  (with explicit renames so lowercase TOML matches)
   ============================================================ */

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WarningLevel {
    Off,
    Default,
    All,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Optimization {
    None,
    Size,
    Speed,
    Aggressive,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Executable,
    Staticlib,
    Sharedlib,
    Interface,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    C,
    Cpp,
    Rust,
    Objc,
    Objcpp,
    Asm,
    Mixed,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyProvider {
    Cmake,
    Prebuilt,
    System,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Linkage {
    Static,
    Shared,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SystemMethod {
    Cmake,
    PkgConfig,
    AppleFramework,
}