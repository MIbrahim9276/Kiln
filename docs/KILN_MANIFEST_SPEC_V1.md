# Kiln Manifest Specification — v1.0

**Status:** Frozen specification (schema `1`)  
**Date:** 2026-11-17  
**Filename:** `kiln.toml`  
**Scope:** Local, multilingual native build orchestration. This document defines the configuration contract, not an implementation claim.

## 1. Purpose and design principles

Kiln coordinates existing build systems to compile and link native projects containing C, C++, Rust, assembly, Objective-C, and Objective-C++. Kiln owns project descriptions, target relationships, configuration resolution, and build orchestration; CMake/Ninja, Cargo, compilers, and linkers own compilation and linking.

1. **Declarative first.** Specify build targets and requirements, not shell recipes.
2. **One manifest per project.** A single root `kiln.toml`; Rust crates and vendor projects may retain their native manifests.
3. **Common concepts, provider-specific extensions.** Unify targets, references, platforms, profiles, and artifacts without hiding native build-system capabilities.
4. **Explicit dependencies and propagation.** Distinguish private and public requirements.
5. **Safe, deterministic, minimal defaults.** Reject unsupported configurations instead of silently guessing.

**Non-goals for schema 1:** package downloads/version solving, package registry, arbitrary custom build scripts, cross-language class/trait binding generation, distributed builds, universal cross-compilation, and a Kiln replacement for Cargo/CMake/Ninja.

## 2. Canonical structure

Top-level TOML tables:

| Table | Purpose | Required? |
|---|---|---|
| `[kiln]` | Manifest schema version | Yes |
| `[project]` | Project identity and default target | Yes |
| `[workspace]` | Build/output locations | No |
| `[build]` | Default profile, jobs, warnings | No |
| `[profiles.<name>]` | Build profile definitions | No |
| `[toolchains.native]`, `[toolchains.rust]` | Toolchain preferences | No |
| `[targets.<name>]` | Kiln-owned build targets | At least one |
| `[dependencies.<name>]` | External dependencies | No |
| `[tests.<name>]` | Named executable tests | No |

`profiles`, `targets`, `dependencies`, and `tests` are **families of named tables**, not literal declarations that must appear in an empty manifest.

### 2.1 Minimal manifest

```toml
[kiln]
schema = 1

[project]
name = "hello-world"
version = "0.1.0"

[targets.app]
type = "executable"
language = "cpp"
sources = ["src/main.cpp"]
```

A `kiln build` on a supported host builds the sole target using a discovered native toolchain. Defaults: debug profile; C17 and C++20; automatic job count; `.kiln/build` for intermediates, `.kiln/out` for deliverables.

## 3. General syntax and resolution rules

- **File encoding:** UTF-8 TOML 1.0.
- **Keys/names:** Project, profile, target, dependency, and test names match `^[a-z][a-z0-9_-]*$`. A target and an external dependency cannot have the same name.
- **Relative paths:** Relative to the directory containing `kiln.toml`, except provider-owned paths within native build systems. Normalize paths; do not require files to reside inside the project for explicitly declared prebuilt/system references, but do not delete anything outside Kiln-owned output roots.
- **Globs:** Source globs support `*` and `**`, use `/` in manifest strings, expand to sorted file lists, and must match at least one file when their containing target is enabled. Explicit source files must exist.
- **Dependencies:** Unqualified reference names resolve against the shared target/dependency namespace. Unknown references and cycles in the Kiln target graph are errors.
- **Versioning:** `schema = 1` is an integer contract independent of Kiln executable or project version. Unrecognized major schema versions are errors.
- **Unknown fields:** Errors, except arbitrary keys in documented pass-through maps (e.g. CMake cache `options`).
- **Build profiles:** Built-in `debug` and `release` exist; user-declared same-named profiles override their settings. Inheritance is resolved before target-specific settings; cycles and unknown parents are errors.
- **Platform selection:** Host target by default. Target-specific compilation requires a compatible installed toolchain and supported provider. Never silently fall back to the host target.
- **Manifest values:** Static by default; only the explicitly defined interpolation expressions in §10 are expanded.

## 4. Project and workspace

### 4.1 `[kiln]`

| Key | Type | Default | Meaning |
|---|---|---|---|
| `schema` | integer | **Required** | Must equal `1` |

### 4.2 `[project]`

| Key | Type | Default | Meaning |
|---|---|---|---|
| `name` | string | **Required** | Project identifier |
| `version` | string | `"0.1.0"` | Project version (SemVer recommended) |
| `description` | string | omitted | Human-readable summary |
| `license` | string | omitted | SPDX expression or descriptive identifier; Kiln does not validate licensing legal compliance |
| `authors` | array of strings | `[]` | Project authors |
| `default_target` | string | inferred if exactly one runnable target | Target selected by `kiln run` and other single-target operations when omitted |

### 4.3 `[workspace]`

| Key | Type | Default | Meaning |
|---|---|---|---|
| `build_dir` | path string | `".kiln/build"` | Generated configuration, intermediate artifacts, caches |
| `output_dir` | path string | `".kiln/out"` | Final executable and library outputs |

Profile and target-specific subdirectories are Kiln-managed implementation details. `clean` may delete only generated paths recorded as Kiln-owned beneath the configured directories; existing unrelated content is not automatically deletable.

## 5. Build defaults and profiles

### 5.1 `[build]`

| Key | Type | Default | Values |
|---|---|---|---|
| `default_profile` | string | `"debug"` | Named profile |
| `jobs` | nonnegative integer | `0` | `0` means automatic parallelism |
| `warnings` | string | `"default"` | `"off"`, `"default"`, `"all"` |

### 5.2 `[profiles.<name>]`

| Key | Type | Default | Values |
|---|---|---|---|
| `inherits` | string | none | Named profile |
| `optimization` | string | inherited | `none`, `size`, `speed`, `aggressive` |
| `debug_info` | boolean | inherited | `true`, `false` |
| `assertions` | boolean | inherited | `true`, `false` |
| `lto` | boolean | inherited (`false` built-in) | `true`, `false` |

Built-in values:

| Profile | optimization | debug_info | assertions | lto |
|---|---|---|---|---|
| `debug` | `none` | true | true | false |
| `release` | `speed` | false | false | false |

Inheritance applies parent values followed by child overrides. A provider that cannot implement a requested optimization must report the limitation; `lto = true` is **not** a guarantee of cross-language LTO.

```toml
[profiles.release-lto]
inherits = "release"
lto = true
```

## 6. Toolchains

Toolchains are selected once per compatible native build context; individual modules should not override `CC`/`CXX` casually.

### `[toolchains.native]`

| Key | Type | Default | Meaning |
|---|---|---|---|
| `provider` | string | `"cmake"` | Schema 1 supports `cmake` |
| `generator` | string | `"Ninja"` | CMake generator (v1 tested with Ninja) |

`[toolchains.native.compilers]` optionally contains `c` and `cpp` executable/path strings. Objective-C and Objective-C++ use a compatible Clang language mode selected by the native toolchain; declaring an unsupported platform/language combination is an error.

### `[toolchains.rust]`

| Key | Type | Default | Meaning |
|---|---|---|---|
| `provider` | string | `"cargo"` | Schema 1 supports `cargo` |
| `toolchain` | string | active Rust toolchain | e.g. `stable` |

Toolchain discovery must record effective compiler/SDK versions, target triple, linker requirements, and diagnostics. Toolchains are not installed by Kiln.

## 7. Internal targets

A target is a Kiln-owned build product. Its name identifies it in dependency lists. Target definitions support **exactly one** of:

- Native source target: `language = "c" | "cpp" | "objc" | "objcpp" | "asm" | "mixed"` and `sources`.
- Rust native artifact target: `language = "rust"`, `manifest`, and optional `cargo_target` / `cargo_features`. The crate's `Cargo.toml` must define a compatible `staticlib` or `cdylib` output; Kiln validates this.
- Interface target: `type = "interface"`, no binary artifact. It carries usage requirements and may omit `language` and `sources`.

### 7.1 Common target fields

| Key | Type | Default / rule |
|---|---|---|
| `type` | enum | **Required:** `executable`, `staticlib`, `sharedlib`, `interface` |
| `language` | enum | Required except interface target; `c`, `cpp`, `rust`, `objc`, `objcpp`, `asm`, `mixed` |
| `sources` | array of path/glob strings | Required for native binary targets (may be supplemented by platform additions) |
| `manifest` | path string | Required for `language = "rust"` |
| `cargo_target` | string | Optional Cargo library target name; default resolved from the crate |
| `cargo_features` | array of strings | `[]`, Rust-only |
| `include_dirs` | array of paths | `[]`, private compilation include paths |
| `public_include_dirs` | array of paths | `[]`, exported to dependent targets |
| `dependencies` | array of names | `[]`, private link/build requirements |
| `public_dependencies` | array of names | `[]`, exported to dependents where meaningful |
| `definitions` | array of strings | `[]`, private native preprocessor definitions |
| `public_definitions` | array of strings | `[]`, exported native preprocessor definitions |
| `enabled` | boolean | `true` |
| `output_name` | string | target name, normalized to platform naming rules |

`language = "mixed"` permits native source files in any supported native language, with per-file language detected by standard extensions (`.c`, `.cc`, `.cpp`, `.cxx`, `.m`, `.mm`, `.s`, `.S`, etc.). It does **not** embed Rust source in a native target; Rust is a separate Cargo target. Explicit, unsupported extensions are errors.

`public_dependencies` propagate public use requirements and necessary link relationships to consumers through the native provider. `dependencies` are direct/private requirements; precise linker propagation follows the resulting native target type (e.g., static archives may require transitive link closure). Kiln must not silently drop a required transitive link input.

### 7.2 `[targets.<name>.compile]` — native targets only

| Key | Type | Default / values |
|---|---|---|
| `standard` | string | Default `c17` for C, `c++20` for C++; valid explicit standard must fit target sources |
| `optimization` | string | Profile value; same enum as profiles |
| `warnings` | string | Global `[build]` value; `off`, `default`, `all` |
| `pic` | boolean | Provider-dependent default; shared libraries generally require PIC where applicable |
| `c_flags` | array of strings | `[]`, compiler-specific |
| `cpp_flags` | array of strings | `[]`, compiler-specific |
| `asm_flags` | array of strings | `[]`, assembler-specific |

Raw flags are explicit escape hatches. Kiln passes them as individual argv items to the matching compiler family; a flag cannot be assumed portable across GCC, Clang, and MSVC. Objective-C/Objective-C++ source compilation uses the toolchain's corresponding language mode and appropriate common/native flags; additional provider-specific support can be added in a later schema revision.

### 7.3 `[targets.<name>.run]` — executable targets only

| Key | Type | Default |
|---|---|---|
| `working_directory` | path string | Project root |
| `args` | array of strings | `[]` |
| `environment` | table of string values | `{}` |

Run arguments are for the resulting executable, not the compiler or provider.

## 8. External dependencies

Dependencies are named resources **consumed** by targets. Kiln v1 does not download packages or solve versions. Cargo continues to handle dependencies *within* Rust crates.

### 8.1 Common dependency fields

| Key | Type | Default / rule |
|---|---|---|
| `provider` | enum | **Required:** `cmake`, `prebuilt`, `system` |
| `enabled` | boolean | `true` |
| `include_dirs` | array of paths | `[]`, provider-supplied usage paths |
| `definitions` | array of strings | `[]`, provider-supplied preprocessor requirements |
| `link_libraries` | array of strings | `[]`, extra native link requirements (advanced) |
| `environment` | table of strings | `{}`, provider process environment overrides |

Provider-specific fields below are legal only under the corresponding provider.

### 8.2 CMake source dependency

```toml
[dependencies.sdl]
provider = "cmake"
path = "vendor/SDL"
target = "SDL3::SDL3"

[dependencies.sdl.options]
SDL_SHARED = false
SDL_STATIC = true
SDL_TESTS = false
```

| Key | Type | Rule |
|---|---|---|
| `path` | path string | Required source directory containing `CMakeLists.txt` |
| `target` | string | Required CMake target exposed by the vendor project |
| `options` | TOML scalar map | Optional CMake cache options: strings, integers, booleans (serialized appropriately) |
| `[dependencies.<name>.cmake].configure_args` | string array | Optional provider-specific CMake configure arguments, no shell evaluation |

Kiln integrates a compatible source CMake project with the native build configuration, maps dependency requirements to its CMake target, and lets CMake manage its internal source graph. For a vendor project requiring a completely separate configure/install workflow, schema 1 may report **unsupported integration** instead of guessing a build recipe.

### 8.3 Prebuilt dependency

```toml
[dependencies.audio]
provider = "prebuilt"
linkage = "shared"
include_dirs = ["vendor/audio/include"]

[dependencies.audio.platforms.windows-x86_64]
link = "vendor/audio/windows/audio.lib"
runtime = ["vendor/audio/windows/audio.dll"]

[dependencies.audio.platforms.linux-x86_64]
link = "vendor/audio/linux/libaudio.so"
runtime = ["vendor/audio/linux/libaudio.so"]
```

| Key | Type | Rule |
|---|---|---|
| `linkage` | enum | Required: `static` or `shared` |
| `link` | path string | Required for effective target platform, at root or in platform selector |
| `runtime` | array of path strings | Default `[]`; deploy/copy alongside consuming binaries where required |

A Windows `.lib` may be a static archive **or** an import library for a DLL: `linkage`, not filename extension, determines intent. Prebuilt artifacts must match target architecture/ABI. Kiln can validate known metadata but cannot guarantee all imported binary compatibility or discover every transitive runtime dependency.

### 8.4 System dependency

Supported methods:

**CMake package discovery**

```toml
[dependencies.threads]
provider = "system"
method = "cmake"
package = "Threads"
target = "Threads::Threads"
```

**Apple frameworks**

```toml
[dependencies.metal]
provider = "system"
method = "apple-framework"
frameworks = ["Metal", "QuartzCore"]
```

**pkg-config discovery**

```toml
[dependencies.zlib]
provider = "system"
method = "pkg-config"
package = "zlib"
```

| Key | Type | Rule |
|---|---|---|
| `method` | enum | Required: `cmake`, `pkg-config`, `apple-framework` |
| `package` | string | Required for CMake/pkg-config discovery |
| `target` | string | Required for CMake method |
| `frameworks` | string array | Required nonempty for Apple-framework method |

Apple framework dependencies are only enabled on supported Apple targets. A dependency not available on the selected target is an error **if selected by an enabled target**.

## 9. Platform overlays

Platforms use canonical OS identifiers `windows`, `linux`, `macos` and CPU identifiers `x86_64`, `aarch64`. Selectors have the form `[targets.NAME.platforms.OS]` and `[targets.NAME.platforms.OS-ARCH]`, likewise for dependencies.

Application order: **base configuration → matching OS overlay → matching OS-architecture overlay**.

- **Arrays** (`sources`, `dependencies`, `public_dependencies`, `definitions`, `public_definitions`, `include_dirs`, `public_include_dirs`, `runtime`, `frameworks`) append in order, deduplicated by exact normalized item where applicable.
- **Scalars** override the less-specific value (e.g. `link`, `output_name`, `enabled`).
- **Maps** (`options`, `environment`) merge by key; the more-specific value wins.
- Unsupported fields in overlays and conflicting effective configuration are errors.
- Selectors do not imply that a cross-compiler exists. Toolchain availability is validated separately.

```toml
[targets.platform]
type = "staticlib"
language = "mixed"
sources = ["src/platform/common/*.cpp"]

[targets.platform.platforms.windows]
sources = ["src/platform/windows/*.cpp"]

[targets.platform.platforms.linux]
sources = ["src/platform/linux/*.cpp"]

[targets.platform.platforms.macos]
sources = ["src/platform/macos/*.mm"]
```

Note: a dependency that is platform-only should be added to the consuming target's matching overlay, rather than included unconditionally in its base dependency list.

## 10. Environment and argument rules

**Recognized interpolation:**

- `${env:NAME}` — required environment variable `NAME`; error if missing.
- `${project.root}` — absolute manifest directory.

No arbitrary expressions, shell substitution, command substitution, or recursion. Interpolation is allowed in path, environment, and argument string values, not in TOML keys or provider identifiers. Kiln passes commands as executable + argv arrays, without implicit shell expansion. Secrets should be passed through process environment, not embedded in committed manifests; diagnostics must not print environment values marked or detected as sensitive.

Example:

```toml
[targets.sandbox.run]
working_directory = "."
args = ["--level", "test"]

[targets.sandbox.run.environment]
GAME_ASSET_PATH = "${env:GAME_ASSET_PATH}"
```

A target's run environment applies to its executable; a dependency's `environment` applies to supported vendor configure/build invocations. The system environment is inherited by default, except where reproducibility or sanitization controls explicitly override it.

## 11. Tests

```toml
[targets.unit_tests]
type = "executable"
language = "cpp"
sources = ["tests/*.cpp"]
dependencies = ["engine"]

[tests.unit]
target = "unit_tests"
args = ["--reporter", "compact"]
working_directory = "."
timeout_seconds = 60

[tests.unit.environment]
TEST_DATA = "${project.root}/tests/data"
```

| Key | Type | Default / rule |
|---|---|---|
| `target` | string | Required; must name an enabled executable target |
| `args` | string array | `[]` |
| `working_directory` | path string | Project root |
| `timeout_seconds` | positive integer | No timeout if omitted |
| `environment` | table of strings | `{}` |

`kiln test` builds the appropriate targets, executes tests, captures process exit status and timeouts, and summarizes results. Kiln does not replace the test framework used by the executable.

## 12. Build and linking model

1. Parse `kiln.toml` into typed schema-1 values.
2. Validate names, sections, provider-specific fields, paths, and references.
3. Resolve selected host/target platform, profile, overlays, and toolchains.
4. Construct target/dependency graph and detect cycles.
5. Configure native CMake targets, compatible external CMake dependencies, prebuilt imported targets, and platform/system requirements.
6. Integrate Rust Cargo-produced `staticlib`/`cdylib` artifacts as native inputs; account for required Rust native/system link dependencies.
7. Build with underlying backends in dependency order and then link final targets.
8. Record deliverables and explicitly configured runtime dependencies in Kiln output metadata.

**ABI boundary is not inferred by Kiln.** The module author supplies compatible headers/bindings, exported symbols, calling conventions, and ownership rules. A Rust crate's ordinary Rust `rlib` is *not* automatically linkable from C++.

**Native C++ / Objective-C++:** Compatible toolchains can share normal C++ interfaces; Objective-C++ `.mm` is compiled directly to native object code, not transpiled to C++. A platform backend can use C++ classes without requiring a C ABI between C++ and Objective-C++.

**Assembly:** Source selection, syntax, object format, architecture, calling convention, and CPU-feature requirements must match the target. Kiln does not rewrite assembly across architectures.

**Shared libraries:** A successful link does not guarantee runtime loadability. Kiln tracks explicitly declared runtime files and provider-discoverable dependencies but cannot promise to bundle all system libraries.

## 13. Complete example — multilingual engine

This example assumes `physics/Cargo.toml` exposes a native static library, `vendor/SDL` is a compatible CMake project, and other source files exist at the stated paths. Apple frameworks are referenced only by the macOS renderer overlay. Renderer backends may need additional SDK libraries when actually implemented.

```toml
[kiln]
schema = 1

[project]
name = "my-engine"
version = "0.1.0"
description = "A modular native game engine"
default_target = "sandbox"

[workspace]
build_dir = ".kiln/build"
output_dir = ".kiln/out"

[build]
default_profile = "debug"
jobs = 0
warnings = "default"

[profiles.debug]
optimization = "none"
debug_info = true
assertions = true

[profiles.release]
optimization = "speed"
debug_info = false
assertions = false

[toolchains.native]
provider = "cmake"
generator = "Ninja"

[toolchains.rust]
provider = "cargo"
toolchain = "stable"

[targets.core]
type = "staticlib"
language = "cpp"
sources = ["src/core/**/*.cpp"]
public_include_dirs = ["include"]

[targets.core.compile]
standard = "c++20"

[targets.math]
type = "staticlib"
language = "mixed"
sources = ["src/math/*.c", "src/math/*.cpp"]
public_include_dirs = ["include"]

[targets.math.platforms.linux-x86_64]
sources = ["src/math/asm/x86_64/*.S"]

[targets.physics]
type = "staticlib"
language = "rust"
manifest = "physics/Cargo.toml"
cargo_target = "physics"
cargo_features = []

[targets.renderer]
type = "staticlib"
language = "mixed"
sources = ["src/renderer/common/*.cpp"]
public_include_dirs = ["include"]
dependencies = ["math"]

[targets.renderer.platforms.windows]
sources = ["src/renderer/d3d12/*.cpp"]

[targets.renderer.platforms.linux]
sources = ["src/renderer/vulkan/*.cpp"]

[targets.renderer.platforms.macos]
sources = ["src/renderer/metal/*.mm"]
dependencies = ["metal"]

[targets.engine]
type = "staticlib"
language = "cpp"
sources = ["src/engine/*.cpp"]
public_include_dirs = ["include"]
dependencies = ["core", "renderer", "physics"]

[targets.sandbox]
type = "executable"
language = "cpp"
sources = ["sandbox/main.cpp"]
dependencies = ["engine", "sdl"]

[targets.sandbox.run]
working_directory = "."
args = []

[dependencies.sdl]
provider = "cmake"
path = "vendor/SDL"
target = "SDL3::SDL3"

[dependencies.sdl.options]
SDL_SHARED = false
SDL_STATIC = true
SDL_TESTS = false

[dependencies.metal]
provider = "system"
method = "apple-framework"
frameworks = ["Metal", "QuartzCore"]

[targets.unit_tests]
type = "executable"
language = "cpp"
sources = ["tests/*.cpp"]
dependencies = ["engine"]

[tests.unit]
target = "unit_tests"
timeout_seconds = 60
```

**Implementation note:** An executable and libraries can be separately built targets while resulting in a single final executable. The `target` and provider relationships specify the build graph; directories do not determine dependencies. This is an illustrative manifest whose referenced files and SDKs must exist for a successful build.

## 14. Validation requirements

| Condition | Required behavior |
|---|---|
| Unsupported schema or unknown field | Error with location; suggest closest known field where practical |
| Malformed TOML or invalid type | Error with line/column where available |
| Invalid/duplicate identifier | Error |
| Missing project or missing required target declaration | Error |
| Unknown target/dependency/test reference | Error |
| Duplicate target/dependency namespace name | Error |
| Build graph or profile inheritance cycle | Error with cycle path |
| Missing explicit source or empty matched glob | Error for enabled selected targets |
| Unsupported source extension or language combination | Error |
| Missing Cargo/CMake manifest or selected native target | Error |
| Missing or incompatible compiler/SDK/assembler | Error before compilation |
| Missing applicable prebuilt binary | Error for referenced dependency |
| Unsupported provider or provider-specific field | Error |
| Missing interpolated environment variable | Error |
| Invalid platform selector or ambiguous overlay | Error |
| Missing default target with multiple runnable targets | Require explicit `--target` for run |
| Runtime file declared but not present | Error at deployment/run preparation |
| Unsupported cross-compilation target | Error, never silently build for host |

Unreferenced dependencies may remain declared without being resolved or built. Disabled targets are excluded from default builds; selecting a disabled target explicitly is an error unless support for an explicit force-enable operation is added later.

## 15. CLI interaction contract (manifest-facing)

- `kiln validate`: parse and statically validate; toolchain checks may be optional or reported separately.
- `kiln doctor`: discover/diagnose environment and required tools.
- `kiln targets`: list effective targets and supported configurations.
- `kiln graph`: inspect target/dependency relationships.
- `kiln plan`: show selected targets, dependencies, providers, tools, and outputs without building.
- `kiln configure`: resolve toolchains and create underlying build configurations.
- `kiln build`: configure as needed, build selected/default targets, and link outputs.
- `kiln run`: build if necessary and execute selected/default runnable target using `.run` settings.
- `kiln test`: build and run selected tests.
- `kiln clean`: remove Kiln-owned generated artifacts safely.

CLI flags (such as target, profile, jobs, and manifest path) override equivalent manifest defaults. Provider-specific settings are not made global CLI flags by default.

## 16. Frozen v1 decisions

1. **Single root manifest** (`kiln.toml`), no nested Kiln manifests in schema 1.
2. **Mixed native-language targets allowed** (`language = "mixed"`); Rust remains a Cargo-built target.
3. **Dependencies supported:** internal targets, external CMake source projects, prebuilt libraries, and discoverable system libraries.
4. **Provider-specific settings typed where possible**, with documented CMake options/arguments and compiler flags as narrowly scoped pass-through mechanisms.
5. **No package management:** no downloading, updating, publishing, package-version solver, or Kiln-managed registry.
6. **No generic command provider in v1.** Unsupported vendor build systems require external preparation or future extension.
7. **One coherent configuration and target graph**, executed by existing tools.
8. **Stable contract:** changes to required semantics need a new manifest schema or clearly documented backward-compatible extensions.

---

**Specification status:** Frozen design baseline for implementing parser types, semantic validation, build planning, provider adapters, and fixture-based integration tests. These are design commitments, not claims that Kiln v1 has been implemented or that every example compiles on every host without the required source files/toolchains.
