# Shared formatting

[Русская версия](../../ru/code-style/formatting.md)

Target contract for the `template` preset. `format` applies these edits; `format --check` reports whether they are needed without writing, and `check` combines that result with [linting](linting.md). Each language adds its own rules: [Python](python/formatting.md), [Kotlin](kotlin/formatting.md), [C++](cpp/formatting.md). Examples are excerpts: `⏎` denotes a newline and `⏎⏎` denotes one blank line. Run the whole formatter twice in acceptance tests: the second pass must produce no edits.

## Indentation and wrapping

Use two-space indentation and a 100-column wrapping target; wrap long signatures and calls with hanging indentation. Do not split an indivisible token or change the value of a literal. Language-specific delimiters and continuation rules still apply.

| Language | Before                       | After                      |
| -------- | ---------------------------- | -------------------------- |
| Python   | `if ready:⏎    send()`       | `if ready:⏎  send()`       |
| Kotlin   | `if (ready) {⏎    send()⏎}`  | `if (ready) {⏎  send()⏎}`  |
| C++      | `if (ready) {⏎    send();⏎}` | `if (ready) {⏎  send();⏎}` |

When these calls exceed the wrapping target, use the corresponding shape (names are shortened here):

| Language | Before                           | After                                    |
| -------- | -------------------------------- | ---------------------------------------- |
| Python   | `run(config, settings, logger)`  | `run(⏎  config,⏎  settings,⏎  logger,⏎)` |
| Kotlin   | `run(config, settings, logger)`  | `run(⏎  config,⏎  settings,⏎  logger,⏎)` |
| C++      | `run(config, settings, logger);` | `run(⏎  config,⏎  settings,⏎  logger);`  |

## Top-level and member spacing

Adopt Ruff-style spacing in all three languages: two blank lines after the final import/include block and between top-level code blocks, including each side of an intervening peer separator. Within a class or function use at most one blank line; use one before the first method and between peer members that need separation. Multiple rules at one boundary do not add up. Keep annotations, decorators and documentation attached to their declarations.

| Language | Before                                     | After                                        |
| -------- | ------------------------------------------ | -------------------------------------------- |
| Python   | `import time⏎def run(): ...`               | `import time⏎⏎⏎def run(): ...`               |
| Kotlin   | `import app.Clock⏎fun run() = clock.now()` | `import app.Clock⏎⏎⏎fun run() = clock.now()` |
| C++      | `#include "Clock.hpp"⏎void run();`         | `#include "Clock.hpp"⏎⏎⏎void run();`         |

## Statement groups

Use a blank line around local declaration groups when adjacent statements belong to other groups. Adjacent declarations may stay together. A declaration and contiguous operations on the same receiver may stay together; stop at an intervening statement or explicit boundary. Keep consecutive fields together unless a comment or annotation opens another group. Preserve explicit boundaries instead of inferring business relationships. After a wrapped statement use a blank line before the next peer statement. See language-specific exceptions.

| Language | Before                                                  | After                                                   |
| -------- | ------------------------------------------------------- | ------------------------------------------------------- |
| Python   | `announce()⏎filter_ = Filter()⏎⏎filter_.init(config)`   | `announce()⏎⏎filter_ = Filter()⏎filter_.init(config)`   |
| Kotlin   | `announce()⏎val filter = Filter()⏎⏎filter.init(config)` | `announce()⏎⏎val filter = Filter()⏎filter.init(config)` |
| C++      | `announce();⏎Filter filter;⏎⏎filter.init(config);`      | `announce();⏎⏎Filter filter;⏎filter.init(config);`      |

## Control-flow spacing

Separate peer `if` and `for` constructs from surrounding statements, including after an unbraced one-line body. Separate conditional branches and exception handlers with a blank line. Put a blank line before `return` unless it is the only statement in its block or belongs to a one-line control-flow body. Do not insert an empty line before the closing delimiter of a block just because the previous statement was control flow. The language pages specify other control-flow boundaries and syntax.

| Language | Before                       | After                         |
| -------- | ---------------------------- | ----------------------------- |
| Python   | `  send()⏎  return result`   | `  send()⏎⏎  return result`   |
| Kotlin   | `  send()⏎  return result`   | `  send()⏎⏎  return result`   |
| C++      | `  send();⏎  return result;` | `  send();⏎⏎  return result;` |

## MARK layout and separators

Use a line of exactly 50 dashes after the comment prefix and one space. Insert a separator at an unambiguous peer-function boundary when missing. A MARK line is directly above its separator. Use the normal top-level or member spacing around the whole block. Fix the length only of recognizable separators; preserve ordinary comments. The formatter does not write descriptive MARK text or change `[group:N]` counts.

| Language | Before                         | After                                                                       |
| -------- | ------------------------------ | --------------------------------------------------------------------------- |
| Python   | `# MARK: Read input⏎⏎# ----`   | `# MARK: Read input⏎# --------------------------------------------------`   |
| Kotlin   | `// MARK: Read input⏎⏎// ----` | `// MARK: Read input⏎// --------------------------------------------------` |
| C++      | `// MARK: Read input⏎⏎// ----` | `// MARK: Read input⏎// --------------------------------------------------` |

## Private Helpers section

In a concrete class with public methods, insert a single `MARK: Private Helpers` block before the private methods if they exist. Remove duplicate structural headers and stale placeholders. When there are no private methods, remove the section header and its separator and put an ordinary `(no private methods)` comment in its place, or insert the comment after the public methods if no section exists. Exempt interfaces and DTO-only classes. Anchor the header to the private declarations; in C++ preserve the effective access specifier. Never reorder methods; [lint](linting.md#private-method-order) reports violations.

| Language | Before (class with no private methods)                                           | After                     |
| -------- | -------------------------------------------------------------------------------- | ------------------------- |
| Python   | `# MARK: Private Helpers⏎# --------------------------------------------------`   | `# (no private methods)`  |
| Kotlin   | `// MARK: Private Helpers⏎// --------------------------------------------------` | `// (no private methods)` |
| C++      | `// MARK: Private Helpers⏎// --------------------------------------------------` | `// (no private methods)` |
