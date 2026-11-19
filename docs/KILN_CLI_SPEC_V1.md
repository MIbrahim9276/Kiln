# Kiln CLI Specification — v1.0

**Status:** Frozen CLI design baseline  
**Date:** 2026-11-17  
**Compatibility:** Kiln Manifest Specification v1.0, `kiln.toml` schema `1`  
**Scope:** Local multilingual native project configuration, build orchestration, execution, inspection, and diagnostics. This is a design contract, not a claim that the commands are implemented.

## 1. Design principles

1. **Project-oriented:** Kiln exposes projects, targets, profiles, dependency graphs, and artifacts—not compiler-specific commands as its main interface.
2. **Delegate execution:** Kiln orchestrates CMake/Ninja, Cargo, compiler toolchains, and native linkers. It is not a compiler, package manager, or universal binding generator.
3. **Safe defaults:** Build commands perform only operations required by the selected project; destructive actions are restricted to verified Kiln-owned outputs.
4. **Predictable behavior:** Common flags, exit codes, targeting rules, and diagnostic conventions are consistent.
5. **Inspectability:** `validate`, `graph`, `targets`, and `plan` permit inspection without building application code.
6. **Manifest authority:** CLI overrides apply to one invocation and never silently rewrite `kiln.toml`.

## 2. Frozen v1 command inventory

Exactly **15 primary commands** are included.

| Group | Command | Purpose |
|---|---|---|
| Project | `kiln new <name>` | Create a new project |
| Project | `kiln init` | Initialize Kiln in an existing directory |
| Project | `kiln info` | Display project metadata and defaults |
| Project | `kiln validate` | Check manifest and project configuration |
| Build | `kiln configure` | Resolve and prepare native/Cargo build integration |
| Build | `kiln build` | Compile and link selected targets |
| Build | `kiln run` | Build if necessary and execute one executable target |
| Build | `kiln test` | Build if necessary and execute defined tests |
| Build | `kiln clean` | Remove verified Kiln-owned generated outputs |
| Inspection | `kiln targets` | List project targets |
| Inspection | `kiln graph` | Show target and dependency relationships |
| Inspection | `kiln plan` | Preview the resolved build plan |
| Diagnostics | `kiln doctor` | Check toolchain/environment readiness |
| Diagnostics | `kiln toolchain` | Inspect discovered toolchains (`list`, `show`) |
| Artifacts | `kiln artifacts` | List recorded outputs and runtime requirements |

`toolchain list` and `toolchain show` are subcommands, not additional primary commands. `--help` and `--version` are standard options, not primary commands.

**Deferred:** `install`, `rebuild` as a standalone command, `deps add/remove/update/sync`, `toolchain install`, `publish`, `deploy`, `generate-bindings`, `watch`, `cache`, and project template registries.

## 3. CLI grammar and conventions

```text
kiln [GLOBAL OPTIONS] <COMMAND> [COMMAND OPTIONS] [ARGS]
```

- Commands, subcommands, options, and profile names are lowercase; long options use kebab-case.
- `kiln --help`, `kiln <command> --help`, and `kiln --version` are supported.
- Common flags may appear before or after subcommands, where unambiguous.
- Unknown commands and options are errors with a suggested correction when possible.
- Standard output is for normal human-readable results or requested structured output. Standard error is for diagnostics and failures.
- Output examples in this document are illustrative and not binding byte-for-byte.
- No implicit shell execution for manifest arguments, paths, or environment variables.
- Kiln never downloads toolchains or third-party packages in v1; Cargo/CMake dependencies retain their existing declared behavior.

### 3.1 Global options

| Flag | Type | Meaning |
|---|---|---|
| `--manifest-path <path>` | path | Explicit `kiln.toml` location; overrides project discovery |
| `-v`, `--verbose` | repeatable flag | More diagnostic detail (e.g. `-vv`); external build logs may be shown |
| `-q`, `--quiet` | flag | Hide routine progress; errors remain visible |
| `--help`, `-h` | flag | Show help |
| `--version`, `-V` | flag | Show Kiln CLI version |

`--quiet` and `--verbose` cannot be combined. On project-aware commands, absent `--manifest-path`, search the current directory and its parents for the nearest `kiln.toml`. `new` does not perform this lookup; `init` operates in the chosen current directory. `doctor` without a project uses host-only checks; with an explicit manifest path or a discovered project it performs project-aware checks.

### 3.2 Shared command options

| Flag | Commands | Behavior |
|---|---|---|
| `--target <name>` | `configure`, `build`, `run`, `plan`, `graph` | Select a project target; repeatable only for multi-target commands |
| `--profile <name>` | `configure`, `build`, `run`, `test`, `plan`, `artifacts` | Use a manifest-defined or built-in profile |
| `--jobs <N>` | `build`, `test` | Number of concurrent jobs; `0` means automatic |
| `--format <format>` | `info`, `targets`, `graph`, `plan`, `artifacts`, `toolchain list`, `toolchain show` | Command-specific presentation format |
| `--no-build` | `run`, `test` | Execute existing artifacts without triggering builds |
| `--fresh` | `build` | Discard only verified Kiln-owned build state for the selected build context, then configure and rebuild |

**Selection rules:**
- `build`, `configure`, `plan`, and `graph` accept repeated `--target`; their default is all enabled relevant targets. `graph` with a selected target includes its dependency closure.
- `run` accepts **exactly one** `--target`, or resolves `[project].default_target` when executable, or the sole enabled executable if unambiguous. Otherwise it errors and lists eligible targets.
- `test` selects all declared tests by default; v1 does not define `--target` as a test selector. Filtering individual tests is deferred.
- `--profile` defaults to `[build].default_profile`, ultimately `debug`. `--jobs` defaults to `[build].jobs`, ultimately `0`.
- `--target` means a **build target name**, never a CPU target triple. Target-triple selection is deliberately excluded from the v1 CLI; manifest/toolchain support governs supported cross-build cases.
- Repeated target selections are deduplicated in first-seen order. Unknown or disabled selected targets are errors rather than silently ignored.

### 3.3 Output formats

- Default for inspection commands is `text`.
- `info`, `targets`, `plan`, `artifacts`, and `toolchain` support `text` and `json`.
- `graph` supports `text`, `json`, and `dot` (Graphviz DOT); `--format dot` emits a dependency graph, **not** a rendered image.
- Structured output must be valid UTF-8 with stable documented field names; incidental diagnostics go to stderr, not into JSON/DOT stdout.
- `validate` and `doctor` provide human-readable results in v1; structured diagnostics can be added in a later version.

## 4. Project commands

### 4.1 `kiln new <name>`

Creates a new directory named `<name>` under the current working directory and writes a minimal manifest and runnable starter source suitable for the detected host toolchain.

**v1 behavior:** Use one built-in minimal native C++ starter template; no network template registry and no selection of other languages. Emit the created path and next-step commands. The generated manifest uses schema `1` and a valid executable target.

**Safety:** Fail if the destination already exists; do not merge with or overwrite it. Check name against manifest project-name rules. Failure during creation must leave no misleading valid-looking partial project where practical.

```console
kiln new sandbox
cd sandbox
kiln build
kiln run
```

### 4.2 `kiln init`

Writes a minimal schema-1 `kiln.toml` in the current directory; intended for an existing codebase. It does **not** automatically scan all source files or infer dependency relationships. If a recognized single `src/main.cpp` exists, it may propose or generate a matching starter target; otherwise create an explicitly documented minimal target scaffold. The generated manifest must validate after initialization.

**Safety:** Never overwrite an existing `kiln.toml`, source file, or other user-owned file. No implicit VCS initialization.

### 4.3 `kiln info`

Shows project name/version/description, manifest location, default target, default profile, build and output roots, number of targets and declared tests, and known dependency providers. This is descriptive inspection; it does not configure or build.

```console
kiln info
kiln info --format json
```

Errors if no project is found or the manifest cannot be parsed sufficiently to report reliable details.

### 4.4 `kiln validate`

Parses TOML schema 1, validates field types, references, profile inheritance, selected platform overlays, source paths/globs, provider-specific options, target/dependency collisions, and graph cycles. Performs **static validation**, not compilation or vendor configuration. It may inspect declared source and binary paths but must not modify project files.

**Distinction from doctor:** `validate` determines whether the project description is internally valid; `doctor` determines whether the required external tools/environment are available. A validated manifest can still fail to build.

Success prints a concise summary. On failure, provide a source location where known, the invalid field/value, and a correction when clear.

## 5. Build and execution commands

### 5.1 `kiln configure [--target <name>...] [--profile <name>]`

Resolves profile, platform overlays, target graph, selected toolchains, and provider capabilities; prepares or updates the generated native build configuration and Cargo integration in the Kiln-owned build directory. Validates necessary environment/toolchain readiness.

- Idempotent for unchanged inputs within a build context.
- No compilation of game sources is required, though provider-specific configuration checks may run.
- A failed configuration must not be recorded as successful.
- Does not alter `kiln.toml` or vendor source configuration in place without an explicitly supported provider workflow.

### 5.2 `kiln build [--target <name>...] [--profile <name>] [--jobs <N>] [--fresh]`

Resolves and validates the selected target closure, automatically configures when missing or stale, and delegates compilation/linking to provider backends.

- Default builds all enabled relevant build targets.
- Dependencies are built in the required order; unrelated work may run in parallel through underlying build systems.
- Incremental builds are delegated to Cargo/CMake/Ninja; Kiln does not implement a compiler cache.
- A successful build records artifacts and declared runtime dependencies for later `run`/`artifacts` operations.
- A failed compiler/linker process propagates a nonzero Kiln exit status and preserves relevant diagnostics.
- `--fresh` removes only positively identified Kiln-owned generated state for the selected build context, then reconfigures; it does not wipe a global Cargo cache or arbitrary vendor directories.

```console
kiln build
kiln build --target renderer --profile release
kiln build --fresh
```

### 5.3 `kiln run [--target <name>] [--profile <name>] [--no-build] [-- <app args...>]`

Resolves exactly one executable target, builds it and its dependency closure unless `--no-build` is set, then executes the resulting program with manifest `[targets.NAME.run]` settings.

- Arguments after `--` are appended after manifest-declared run arguments.
- Manifest run `working_directory` and `environment` apply; Kiln inherits and safely overlays the process environment.
- Runtime files explicitly declared in the manifest must be made available according to supported platform deployment behavior; missing required files produce an actionable error.
- Child stdout/stderr are streamed; an interrupted run should forward termination appropriately where supported.
- A nonzero child exit code is reflected as a failure, not presented as a successful run.
- `--no-build` errors if no usable artifact is recorded/found for that profile and target.

```console
kiln run --target sandbox -- --level test
kiln run --no-build
```

### 5.4 `kiln test [--profile <name>] [--jobs <N>] [--no-build]`

Builds the executable targets referenced by `[tests.*]` unless `--no-build` is specified; runs every declared test with its specified arguments, working directory, environment, and optional timeout. Summarizes passes, failures, and timeouts.

- All configured tests run by default; no test-name filter is part of CLI v1.
- Test processes may run concurrently, limited by `--jobs`; reporting remains deterministic by test name.
- Returns nonzero if any test fails, times out, or cannot launch.
- With `--no-build`, missing test executables are errors.
- Test process output is captured or forwarded in a way that permits debugging failures.

### 5.5 `kiln clean`

Removes generated artifacts and intermediate build state **only when Kiln can confirm ownership** inside configured build/output roots. It does not delete arbitrary files simply because they are under a user-configured directory. It does not delete external Cargo caches, downloaded packages, source files, or unrelated vendor outputs.

- Default scope: all Kiln-owned generated contexts for the current project.
- Before deleting, canonicalize and verify destination paths; reject unsafe configurations, path escapes, or untrusted symlink traversals.
- Reports removed locations and any paths deliberately skipped.
- Safe repeated use: nothing to clean is success.
- v1 has no `--all`, `--force`, or general-purpose filesystem-cleaning mode.

## 6. Inspection commands

### 6.1 `kiln targets [--format text|json]`

Lists target name, type, language, enabled state for the selected host/platform, and key dependency references. Does not compile.

### 6.2 `kiln graph [--target <name>...] [--format text|json|dot]`

Displays the target/dependency graph. By default shows the full resolved graph; target selection limits output to specified roots and their dependencies. Graph edges follow the convention **consumer → dependency**. External dependencies appear as distinct nodes.

- Detect and report cycles as validation errors.
- `text`: readable indented dependency tree, annotating repeated nodes rather than recursing forever.
- `json`: machine-readable nodes and edges.
- `dot`: valid Graphviz DOT source.
- No graph image rendering or Graphviz installation required.

### 6.3 `kiln plan [--target <name>...] [--profile <name>] [--format text|json]`

Produces a preview of selected targets, dependency closure, provider assignments, required toolchains, effective profile/platform, and planned build/configure/link stages. Does not invoke compiler/linker builds or mutate artifacts.

**Precision:** The plan is a *high-level resolved operation plan*, not a guaranteed per-source incremental rebuild list; Cargo/Ninja retain their own up-to-date decisions. A plan may mark provider-specific actions as indeterminate until configuration. If a toolchain cannot be resolved, report that explicitly rather than inventing a command.

## 7. Diagnostics commands

### 7.1 `kiln doctor`

Detects available versions and basic capability of tools relevant to the host or discovered project: CMake, Ninja, native C/C++ toolchain, Cargo/rustc when Rust is used, Obj-C/Obj-C++ where supported, assemblers, SDKs, and platform/architecture compatibility. Reports required errors separately from optional warnings.

- Without a project: host-wide readiness checks only.
- With a project: checks toolchains and platform capabilities required by its enabled targets and dependencies.
- Does not install, modify, or update tools.
- A missing optional tool is a warning; a missing required tool is an error.

### 7.2 `kiln toolchain list` and `kiln toolchain show <name>`

Read-only inspection of discovered toolchain information, including version, path, supported target as known, and relevant provider configuration. `show` accepts configured toolchain identifiers such as `native` and `rust`.

```console
kiln toolchain list
kiln toolchain show native
kiln toolchain show rust --format json
```

These commands do not change toolchain selections or install software. Unknown toolchain identifier is an error.

## 8. Artifact inspection

### `kiln artifacts [--profile <name>] [--format text|json]`

Reads Kiln-generated artifact metadata for the selected profile and reports target, artifact kind, output path, build context, and explicitly declared runtime dependencies when known.

- Read-only; does not build, install, copy, or deploy artifacts.
- If no artifacts have been produced, display an empty list with a clear message (successful command).
- If recorded artifact files no longer exist, mark them missing/stale rather than presenting them as usable.
- Build outputs should be stored under `[workspace].output_dir`; Kiln-owned provider metadata remains under `[workspace].build_dir`.

## 9. Target selection, defaults, and manifest consistency

1. Manifest schema 1 defines project metadata, target kinds, profiles, platform overlays, dependencies, tests, and toolchains. CLI v1 does not add new manifest tables.
2. `default_target` in `[project]` is used for a single-target run operation when executable. Otherwise a sole executable is selected if unambiguous.
3. `--profile` overrides `[build].default_profile` for one invocation. `--jobs` overrides `[build].jobs` for one invocation. Neither rewrites the manifest.
4. Project target `type` uses exactly: `executable`, `staticlib`, `sharedlib`, `interface`. The CLI reports these names as specified.
5. Rust native libraries require declared compatible Cargo outputs. Kiln doesn't infer a working C ABI from Rust source.
6. An external dependency declared as `cmake`, `prebuilt`, or `system` is not managed as a downloadable package.
7. Toolchain resolution takes platform overlays and enabled targets into account; unsupported host/target combinations fail explicitly.
8. Project commands do not require a running game editor or IDE.
9. `--format` is a CLI presentation option, not a manifest field. No `--json` shortcut is required for v1.
10. The v1 CLI does not expose `--platform` or `--arch` as promised cross-build selectors; those can be added only after target/toolchain semantics are specified and tested.

## 10. Exit codes and errors

Kiln must return process exit code `0` only for successful completion (including no-op builds, clean with nothing to delete, and valid empty artifact listings).

| Exit code | Category | Examples |
|---|---|---|
| `0` | Success | Build succeeded, validation passed |
| `2` | CLI usage | Unknown command, missing argument, conflicting flags |
| `3` | Manifest/configuration | Invalid TOML, unknown target/profile, graph cycle |
| `4` | Toolchain/provider readiness | Missing compiler/SDK, unsupported provider integration |
| `5` | Build/configure failure | Native build system, compiler, linker, or Cargo error |
| `6` | Execution/test failure | Child executable failed, test failed or timed out |
| `7` | File/artifact operation | Unsafe clean, missing expected artifact, filesystem failure |
| `1` | Other internal failure | Unexpected Kiln error |

Exit codes are **Kiln category codes**; v1 does not promise to preserve an arbitrary child process numeric exit code verbatim. Kiln must show child exit status and provider diagnostics where available. Ctrl+C handling should return an appropriate interrupted status on the host and terminate child processes safely where possible.

Diagnostics should include: a short error category, affected target/provider/manifest field when available, the meaningful underlying cause, and actionable remediation. Do not dump secrets or unredacted environment overrides to logs.

## 11. Example workflows

### New C++ starter

```sh
kiln new sandbox
cd sandbox
kiln validate
kiln build
kiln run
```

### Multilingual engine

```sh
kiln doctor
kiln targets
kiln graph --format dot
kiln plan --target engine
kiln build --target engine --profile debug
kiln test
```

### Inspect and debug a failing build

```sh
kiln validate
kiln doctor
kiln plan --target sandbox --format json
kiln build --target sandbox -vv
kiln artifacts
```

### Produce fresh release build

```sh
kiln build --profile release --fresh
kiln test --profile release
kiln artifacts --profile release
```

`kiln install` is **not** part of this workflow or CLI v1. Packaging/deployment remains outside scope.

## 12. Implementation order (informative, not part of interface compatibility)

1. **Project primitives:** `init`, `validate`, `info`, `targets`.
2. **Toolchain and planning:** `doctor`, `toolchain`, `graph`, `plan`, `configure`.
3. **Core execution:** `build`, `run`, `test`, `clean`.
4. **Polish:** `new`, `artifacts`, structured inspection formats, cross-platform acceptance tests.

The order does not change the frozen command inventory.

## 13. Acceptance criteria for CLI v1

- A minimal schema-1 C++ project can be created, validated, configured, built, and run through Kiln commands.
- A mixed C/C++/assembly native target and a Cargo-produced Rust native library can be built and linked on tested supported hosts.
- A compatible vendor CMake source dependency, prebuilt library, and system dependency can be resolved from an existing manifest without Kiln downloading them.
- Apple-specific Objective-C++ sources are selected only on supported Apple build contexts.
- `graph` and `plan` describe the resolved graph and provider assignments without building application source.
- `doctor` reports missing required toolchains separately from optional tools.
- `test` reports failures and timeouts with nonzero status.
- `clean` never removes user-owned source, vendor directories, global caches, or unverified paths.
- CLI flag behavior and exit codes are covered by automated tests on Windows, Linux, and macOS for the documented supported combinations.

## 14. Frozen decisions and change control

**Approved v1 command set:** `new`, `init`, `info`, `validate`, `configure`, `build`, `run`, `test`, `clean`, `targets`, `graph`, `plan`, `doctor`, `toolchain`, `artifacts`.

**Explicit v1 non-goals:** package installation/resolution, compiler installation, deployment, graphical editor, custom linking engine, universal binding generation, arbitrary custom command provider, remote builds, and universal cross-compilation.

This document freezes the **public CLI design baseline**. Implementation details may change as long as the described user-visible behavior, manifest compatibility, safety rules, and supported command contracts remain intact. Incompatible CLI changes require a documented specification revision rather than an undocumented drift from this baseline.
