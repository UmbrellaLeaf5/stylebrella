# TODO: Complete Stylebrella Implementation

This roadmap implements the application described in [README.md](README.md).
All tasks below are planned; none is marked as completed before the code
exists. The Stylebrella [formatting](docs/en/code-style/general/formatting.md) and
[linting](docs/en/code-style/general/linting.md) contracts define the current target.
The original template and engine configurations are reference material.
Notebook ideas below are deferred until a notebook contract is defined.

## 0. Requirements and compatibility proof

- [ ] Map every documented rule ID to an implementation and acceptance
      fixtures. Keep this implementation mapping separate from the public
      contract. Do not implement review recommendations as diagnostics.
- [ ] Turn examples into golden fixtures for indentation, comments, blank
      lines, `else`/`catch`, signatures, and file structure.
- [ ] Preserve `cpp/.clang-format`, `kotlin/.editorconfig`, and
      `python/ruff.toml` as versioned preset inputs.
- [ ] Compare ktlint, ktfmt, and IntelliJ output on the golden Kotlin
      fixtures; document differences. Select **one** Kotlin formatter. If an
      essential rule conflicts with every candidate, resolve the product
      requirement explicitly rather than creating a postprocessing loop.
- [ ] Verify the C++ and Python engines against their guide examples;
      specifically investigate conflicting blank-line rules and convergence.

**Done when:** all in-scope rule IDs have implementation coverage, runnable golden
fixtures exist, and the Kotlin formatter choice is supported by evidence.

## 1. Rust workspace and core model

- [ ] Create workspace members for the CLI, core, Python/notebooks, and
      C++; keep the Kotlin ruleset in a separate JVM module. Separate argument
      parsing, process execution, and rule logic.
- [ ] Define `Diagnostic`, `SourceRange`, `TextEdit`, `RuleMetadata`,
      `EngineResult`, `ProjectConfig`, `Language`, and the machine-output
      schema version. Specify Unicode coordinate and offset conventions.
- [ ] Implement canonical IDs and short aliases from the
      [rule ID registry](docs/en/code-style/README.md#id-registry). Map
      engine findings to the documented Stylebrella rules they implement.
- [ ] Implement project discovery, extension filtering, symlink behavior,
      generated-directory exclusions, deterministic file ordering, and nested
      project boundaries; do not enter dependencies or submodules implicitly.
- [ ] Implement `stylebrella.toml` and engine-native config discovery,
      schema validation, and actionable errors for invalid keys or paths.
- [ ] Implement a common `EngineAdapter`, bundled executable lookup,
      timeouts, output limits, error handling, cancellation, and child-process
      termination.
- [ ] Implement read-only diff generation, atomic writes, preservation of
      line endings, encoding, and permissions, and detection of overlapping
      text edits.

**Done when:** core unit tests run without installed Ruff/Java/LLVM, and
missing or broken release components return error `2`, never false success.

## 2. CLI and machine-readable API

- [ ] Implement `check`, `lint`, `format`, `fix`, `explain`, and `--version`.
      Freeze `--config`, files/directories, `--output text|json|sarif`,
      `--check`, `--diff`, and invalid flag combinations in CLI tests.
- [ ] Implement single-file stdin processing for `.py`, `.kt`, `.kts`,
      and C++ with mandatory `--stdin-filename`. Keep formatter source output
      free of engine logs; direct errors to stderr.
- [ ] Freeze exit codes `0`/`1`/`2` and test all commands with findings,
      invalid configs, unsupported languages, and successful fixes.
- [ ] Implement deterministic diagnostic sorting and deduplication,
      human output, versioned JSON, and valid SARIF with correct locations.
- [ ] Document custom rules through `explain`; only promise an automatic
      `fix` for rules with a proven safe edit.

**Done when:** the CLI matches README, accepts multiple paths, read-only
commands leave files unchanged, and standard consumers parse JSON/SARIF.

## 3. Python and Jupyter

- [ ] Bundle a pinned Ruff release; implement `ruff check`,
      `ruff format`, read-only modes, and approved safe fix modes.
- [ ] Honor preset and local `ruff.toml` settings: width 100, indent 2,
      import rules, and the original config's rule selections/exclusions.
- [ ] Implement syntax-aware Python analysis with locations and access
      to comments/whitespace for custom checks; do not use regex for nested
      syntactic constructs.
- [ ] Implement missing enforceable checks: function annotations,
      well-formed `MARK` and 50-hyphen separators, applicable naming,
      docstrings, and constructor category markers where criteria are precise.
- [ ] Avoid duplicating Ruff rules and never generate `# noqa`, guessed
      types, or invented documentation.
- [ ] Run Ruff on `.ipynb`; separately parse notebook JSON to validate
      empty or oversized cells, no `def`/`class`, import-section placement,
      and other precisely specified notebook rules.
- [ ] Preserve notebook metadata and outputs; do not execute code,
      remove results, or reformat markdown without an explicit notebook rule.
- [ ] Evaluate optional Pyright integration: document runtime cost and
      behavior without a project config; if included in the release, bundle
      every required runtime component.

**Done when:** Python and notebook fixtures pass `check`, `format`, and
`fix`, and a second run does not alter files or unrelated notebook cells.

## 4. Kotlin

- [ ] Complete the formatter decision in stage 0. For ktlint, pin
      compatible ktlint, Kotlin compiler/plugin, and JVM versions.
- [ ] Run the Kotlin engine on `.kt` and `.kts` with correct
      `.editorconfig` resolution, existing disabled standard rules, and
      normalized machine-readable diagnostics.
- [ ] Build a JVM ruleset for `!!`, named null wrappers, top-level type
      counts, file/declaration names, expression bodies, `MARK`/separators,
      and other precisely enforceable requirements.
- [ ] Cover top-level-function-only files and application entry points
      separately; do not relocate classes as an automatic fix.
- [ ] Test blank lines, `else` placement, constructor categories, and
      argument wrapping against the original `.editorconfig`.
- [ ] Do not use the native ktlint executable as a host for a ruleset JAR;
      test the bundled JAR, JVM, and ruleset together.
- [ ] Keep intent-dependent concepts such as “important method” and
      “internal DTO” human-facing or optional until a reliable criterion exists.

**Done when:** golden Kotlin fixtures reach a fixed point, and custom
rules run from an offline package without a system JVM.

## 5. C++

- [ ] Bundle compatible pinned clang-format and clang-tidy versions;
      account for dynamic libraries and `.clang-format` lookup.
- [ ] Format only supported C/C++ files; exclude `CMakeLists.txt` and
      `*.cmake` even when scanning a directory. Use `--assume-filename` for
      stdin and the relevant project config.
- [ ] Reuse applicable clang-tidy checks for C-style casts,
      `NULL`/`nullptr`, `override`, and other native capabilities; document
      cases where native checks only partially match the original rule.
- [ ] Use `compile_commands.json` for checks requiring compiler flags.
      Explicitly report skipped semantic checks if it is missing; still run
      independent file and syntax checks.
- [ ] Implement enforceable gaps: `#pragma once`, type/file matching,
      header structure, required multiline Doxygen in `.hpp`,
      `MARK`/separators, and applicable naming rules.
- [ ] Leave exception-class choice, meaningful Doxygen text, and
      ownership decisions to humans where static analysis is insufficient.

**Done when:** C++ checks work with and without a compilation database,
report coverage honestly, and never alter CMake files.

## 6. Processing order and output quality

- [ ] Fix a pipeline order: config resolution → file selection → native
      formatter/linter → custom rules → diagnostic merge → edits. Specify
      native fix order and final formatting in `fix` mode.
- [ ] Reject overlapping edits and re-analyze after changes; continue
      reporting findings that remain after a fix.
- [ ] Add fixed-point tests: `format(format(x)) = format(x)`; `fix` +
      `format` + `check` must not produce a change cycle.
- [ ] Test CRLF input for Kotlin from `.editorconfig` and Python line endings
       from `ruff.toml`; verify that formatting normalizes both to LF,
      Unicode locations, BOM, parse errors, empty and large files, symlinks,
      and multiple nested configs.
- [ ] Turn examples from all four guides into integration fixtures.
      Each rule needs at least a positive, negative, and boundary case.
      Verify that automatic fixes preserve meaning where a test can prove it.
- [ ] Run end-to-end CLI tests on fixture directories and consume
      machine reports in a CI-compatible reader; verify stable repeated output.

**Done when:** no bundled formatter undoes a custom fix and results are
reproducible across supported platforms.

## 7. Self-contained release

- [ ] Define supported OS/architecture targets and archive layout:
      `stylebrella`, Ruff, clang-format, clang-tidy, Kotlin JAR and ruleset,
      compatible Java runtime, preset configs, license files, and a version
      manifest.
- [ ] Pin versions and verify hashes for third-party artifacts. Download
      or build dependencies **during release creation**, never on first run.
- [ ] Locate resources relative to the installed executable, rather
      than relying on tools in the user's `PATH`.
- [ ] Check dynamic dependencies, executable permissions, architecture,
      macOS/Windows packaging behavior, and final download size.
- [ ] Include third-party licenses and notices with every archive;
      Stylebrella's own code uses the Unlicense.
- [ ] Test each archive in a clean system/container without separately
      installed Python, Ruff, Java, ktlint, or LLVM; disable network access
      while running `check`, `lint`, `format`, and `fix`.
- [ ] Publish installation, update, and version-check instructions for
      every supported platform.

**Done when:** users install only the Stylebrella package and every command
works offline without installing separate language tools.

## 8. Template migration and 1.0 release

- [ ] Map each entry from the stage 0 inventory to an engine config,
      implemented rule, `explain` entry, or short human-facing instruction;
      retain notebook-specific requirements.
- [ ] When migrating the template, update references in `AGENTS.md` to
      removed `CODE-STYLE.md` files. Keep architectural/process rules where
      a linter cannot enforce them. Remove guides only after coverage exists.
- [ ] Write user documentation covering configuration, stdout/stderr,
      exit codes, fixed-point behavior, supported extensions, and limitations.
- [ ] Verify CLI, JSON, SARIF, and stable rule ID compatibility. Define
      a versioning policy for presets and bundled engine upgrades.
- [ ] Release 1.0 only after golden tests and clean-environment package
      checks pass for every advertised target.

**Full-implementation criterion:** three languages and notebooks have
compatible formatters; formalizable rules have checks and explanations;
rules that cannot be reliably automated have explicit documentation; and
the release requires no third-party installations by users.
