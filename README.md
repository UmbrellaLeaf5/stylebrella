# Stylebrella

Stylebrella is a unified linter and formatter for Python, Kotlin, and C++.
It delegates standard formatting to established language-specific engines and
adds checks for project-specific conventions. Its target requirements come
from `python/CODE-STYLE.md`, `python/notebooks/CODE-STYLE.md`,
`kotlin/CODE-STYLE.md`, and `cpp/CODE-STYLE.md` in
`template_opencode_agented`. The first Windows preview packages the style
engines and a CLI scaffold; the style commands themselves are still stubs.

The goal is to replace long instructions for _mechanically enforceable_ style
with reproducible checks. Architectural conventions, development procedures,
and requirements that depend on the meaning of code remain human-facing
instructions unless they can be checked reliably.

## Windows x64 preview

Unpack the entire ZIP and run `stylebrella.exe --version --engines` to verify
Ruff, clang-format, clang-tidy, and ktlint. `--version` alone prints only the
CLI version. The style command stubs return exit code `2` and do not process
files.

To build the ZIP from source on Windows x64 with the
`x86_64-pc-windows-gnu` Rust toolchain and a working linker:

```text
cargo xtask package
```

The Rust packaging tool verifies pinned downloads using
`packaging/engines.windows-x64.json` and installs the bundled engines before it
builds Stylebrella. It writes `dist/stylebrella-<version>-windows-x64.zip` after
testing the extracted archive. It extracts the native LLVM binaries from wheels,
so neither packaging nor the release needs Python. The Kotlin component uses
its bundled Java runtime. Normal `cargo test --workspace --locked` runs without
installed engines.

Rust callers use `BundledEngines::command(engine)` from `stylebrella-core` to
get a `std::process::Command` and configure its arguments and I/O. All engine
paths are resolved from the installation, without a system `PATH` fallback.

## Principles

- **One command, three languages.** Each language has one selected formatting
  engine. Stylebrella does not reimplement the underlying formatters.
- **No external installations for users.** Official release archives include
  every required executable and, if needed, a Java runtime for Kotlin plugins.
  Users do not need to install Ruff, ktlint, clang-format, or a JVM separately.
- **No unexpected edits.** `check` and `lint` are read-only; `format` changes
  formatting only; `fix` applies only explicitly designated safe fixes.
- **Unified diagnostics.** Native and custom findings share a path, location,
  source, stable rule ID, severity, and message.
- **Convergent output.** A second formatting pass changes nothing. Fixes and
  formatters must not undo one another.
- **Honest automation.** Terms such as “logical group,” “important method,”
  and “domain-significant literal” do not become reliable machine checks just
  because they occur in a style guide.

## Planned CLI

```text
stylebrella check [PATH ...] [--output text|json|sarif] [--config FILE]
stylebrella lint [PATH ...] [--output text|json|sarif] [--config FILE]
stylebrella format [PATH ...] [--check] [--diff] [--config FILE]
stylebrella fix [PATH ...] [--diff] [--config FILE]
stylebrella explain RULE_ID
stylebrella --version
```

`PATH` can be a file or directory; the current directory is used when no path
is supplied. `check` verifies both formatting and lint rules. `lint` checks
rules, including those from bundled linters. `format --check` checks only
formatting. `format --diff` and `fix --diff` print proposed changes without
writing files. `fix` applies only approved safe edits and still reports
violations that cannot be fixed automatically. `explain` displays a rule's
rationale and examples. Common flags will include rule filters and profile
selection; their exact spelling will be frozen alongside CLI tests.

Editor integration will support processing a single file from stdin:
`--stdin-filename PATH` supplies the language and configuration context. For
`format`, formatted source goes to stdout and errors go to stderr. For
`check` and `lint`, machine-readable reports go to stdout. Formatting stdin
must never put progress messages on stdout. Notebook editing through stdin
is outside the initial contract: `.ipynb` processing uses files to preserve
the JSON structure and metadata.

Exit codes: `0` for success without remaining violations, `1` for findings or
unformatted input, and `2` for invalid configuration, a failing bundled
engine, file I/O failure, or internal error. `format` and `fix` return `0`
after successful changes when no violations remain. CLI tests will specify
how unfixable findings affect the exit status after `fix`.

Examples of the **intended**, not yet available, interface:

```bash
stylebrella check .
stylebrella check src tests --output sarif
stylebrella lint src --output json
stylebrella format src --diff
stylebrella format src
stylebrella explain CPP-HEADER-PRAGMA-ONCE
```

## Language engines and reuse boundaries

| Language                                         | Bundled engine                                                     | Stylebrella's responsibility                                                              |
| ------------------------------------------------ | ------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| Python (`.py`)                                   | Ruff: `check`, permitted `check --fix` rules, and `format`         | Additional syntax/file rules and normalized diagnostics                                   |
| Python (`.ipynb`)                                | Ruff notebook linting/formatting                                   | Cell structure, imports, definitions, documentation, and preservation of metadata/outputs |
| Kotlin (`.kt`, `.kts`)                           | Preferred candidate: ktlint, including its formatter               | Kotlin AST rules in a separate JVM module, diagnostics, and safe fixes                    |
| C++ (`.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, `.hh`) | clang-format for layout; clang-tidy for applicable semantic checks | File structure, `MARK`, Doxygen, additional checks, and diagnostic context                |

### Python

Ruff is the source of truth for standard formatting. The existing `ruff.toml`
defines two-space indentation, a width of 100, double quotes, import sorting,
and the chosen lint rules. The Stylebrella preset includes the original
template settings; a project's local `ruff.toml` takes precedence. Any
postprocessing of `ruff format` output requires a convergence test: running
Ruff again must not undo Stylebrella's edits.

Ruff does not expose a third-party lint plugin API. Additional rules therefore
run in a separate analyzer. Syntax-aware Python checks need a parser with
source locations and access to comments or whitespace when those matter.
Pyright is a separate type checker: whether it belongs in the mandatory
`check` pipeline will be decided after a prototype and explicitly recorded in
configuration, rather than hidden behind Ruff. If included, its runtime
requirements must also ship in the release.

Ruff handles `.ipynb`, but it does not enforce the template's requirements
such as no `def`/`class` in cells, cell ordering, a maximum of 15 lines for
section-level code cells, or meaningful markdown cells. A separate notebook
JSON analyzer is needed. Executing notebooks with `nbconvert` checks
reproducibility; it is **not** formatting. It must be an explicit project
check because it executes user code and can require data or services.

### Kotlin

The source template uses `.editorconfig` and IntelliJ formatting. Its config
sets `indent_size = 2`, `max_line_length = 100`, wrapping, blank lines, and
`else` placement, and disables selected ktlint rules. Neither ktlint nor
ktfmt can be assumed to reproduce this without testing. First, collect
golden code examples from the style guide and compare ktlint, ktfmt, and
IntelliJ output. Then select **one** formatter. Running ktlint and ktfmt
sequentially is not an acceptable formatting strategy.

The initial candidate is ktlint: it supports `.editorconfig`, automatic
corrections, machine-readable reports, and custom Kotlin rulesets. Custom
rulesets require a JVM/JAR: native ktlint executables cannot load extra
ruleset JARs. Accordingly, the official Kotlin component would bundle a
compatible Java runtime, the ktlint JAR, and a custom ruleset JAR. The engine
choice can change after compatibility tests; ktfmt is not a silent second
formatter or a postprocessing step.

Candidate custom rules include banning `!!`, enforcing named null-validation
wrappers, limiting top-level types per file, file naming, constructor
parameter categories, `MARK` formatting, and expression-body conventions.
Concepts such as “important function” or “internal DTO” require project
context: enforce them only where a precise rule can be validated against
positive and negative examples.

### C++

The initial formatting preset comes from `cpp/.clang-format`: Google style,
a width of 100, two-space continuation indentation, and its other settings.
A project's own `.clang-format` takes precedence. clang-format runs only on
supported C/C++ sources and headers: **never on `CMakeLists.txt` or `*.cmake`**.
For stdin, pass the real path via `--assume-filename` so clang-format finds
the appropriate configuration.

clang-format does not enforce `#pragma once`, the `include/` and `src/`
layout, a type's matching filename, Doxygen, `MARK`, exception choice, or
ownership. Reuse existing clang-tidy checks first (for example, C-style
casts, null pointers, and `override`); implement only missing custom rules.
Semantic clang-tidy checks need `compile_commands.json`. If it is absent,
report that semantic checks were skipped instead of silently claiming they
passed; independent file and syntax checks must still run.

## Rules and fixes

Stable rule IDs use the `PY-*`, `NB-*`, `KT-*`, and `CPP-*` prefixes. Initial
candidates include:

- `CPP-HEADER-PRAGMA-ONCE`: require `#pragma once` in project-owned headers;
  only auto-insert it when its location is unambiguous.
- `CPP-HEADER-DOXYGEN`: require multiline Doxygen on applicable declarations
  in `.hpp` files; never invent a description.
- `KT-NO-BANG-BANG`: forbid `!!`; no automatic replacement because the right
  validation and exception depend on intent.
- `KT-ONE-TYPE-PER-FILE`: limit top-level types per file with documented
  exceptions; do not move classes automatically.
- `PY-TYPED-FUNCTION`: require parameter and return annotations; only
  auto-fix when the correct type is demonstrable.
- `NB-NO-DEFINITIONS`: forbid `def` and `class` in code cells; report only.
- `*-MARK-SEPARATOR`: validate `MARK` syntax and a separator of **exactly 50
  hyphens**; a uniquely identifiable separator length can be auto-fixed.

Diagnostics are sorted by path, line, column, and rule ID. A normalized
diagnostic carries `rule_id`, `source`, `severity`, `message`, `path`,
`range`, and an optional `fix` composed of non-overlapping text edits.
JSON and SARIF outputs include a schema version and bundled engine versions
for reproducibility. Stylebrella must not silently generate Russian prose,
decide whether a literal has domain meaning, or insert blank lines between
conceptual groups. Such guidance remains human-facing or becomes a clearly
documented optional heuristic.

## Configuration contract

Stylebrella finds `stylebrella.toml` by searching from the source file toward
the project root. The `template` preset captures mechanically enforceable
rules from the original guides. Closer `ruff.toml`, `.editorconfig`, and
`.clang-format` files override their **own engine's** preset settings;
their contents are not copied into `stylebrella.toml`. Exclusions are applied
before launching engines. Project boundaries and precedence must behave
consistently for individual files, stdin, and recursive walks.

Illustration of the intended configuration; final keys will be fixed by
schema and CLI tests:

```toml
preset = "template"

[include]
patterns = ["src/**", "tests/**", "notebooks/**"]

[exclude]
patterns = ["build/**", ".venv/**", "generated/**"]

[rules]
"CPP-HEADER-DOXYGEN" = "error"
"KT-NO-BANG-BANG" = "error"
"NB-NO-DEFINITIONS" = "error"
```

Bundled engine versions are pinned per release. Arbitrary programs found on
the user's `PATH` must not replace bundled tools by default. Rules can be
explicitly enabled, disabled, or given a supported severity; configuring a
file to alternate between two formatters is not supported.

## Architecture and distribution

Proposed source layout:

```text
crates/
  stylebrella-cli/        # arguments, human output, exit codes
  stylebrella-core/       # discovery, configs, pipeline, diagnostics, edits
  stylebrella-python/     # Python and notebook checks
  stylebrella-cpp/        # C++ checks and Clang adapters
kotlin-rules/             # Kotlin/JVM ktlint ruleset
presets/template/         # original configs and rule examples
tests/fixtures/           # golden examples and integration cases
```

Adapters launch **bundled, pinned** engines, supply the real filename for
configuration lookup, validate exit statuses, and normalize output. The core
controls timeouts, errors, output limits, and child-process termination.
Writes check for conflicting edits, preserve permissions and line endings,
and replace files only after a successful result has been prepared. Notebook
processing preserves untouched metadata and outputs; editing anything beyond
code cells requires an explicit notebook rule.

Official releases are **self-contained archives per supported OS and
architecture**: the CLI, Ruff, clang-format, clang-tidy, the Kotlin
runtime/JARs, preset configs, and third-party license notices. Release builds
download or build pinned versions, verify checksums, bundle necessary dynamic
libraries and resources, and test the archive in a clean environment without
Rust, Python, Java, or LLVM on `PATH`. The installed release works offline;
first launch never downloads dependencies. Supported targets and package
size will be determined by a packaging prototype, not assumed in advance.
Building from source with `cargo build` may require build dependencies; that
is distinct from installing an official release.

The full implementation plan and acceptance criteria are in [TODO.md](TODO.md).

## License

Stylebrella's own code is released under the [Unlicense](LICENSE). Bundled
third-party engines retain **their own** licenses; the required notices and
license texts must be included in every release archive.
