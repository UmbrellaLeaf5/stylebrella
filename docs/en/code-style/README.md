# Code style

[Русская версия](../../ru/code-style/README.md)

These specifications define the expected behavior of Stylebrella's formatter and linter. Start with the shared rules in `general/`, then consult the language-specific rules for Python, Kotlin, or C++.

| Scope        | Formatting                          | Linting                       |
| ------------ | ----------------------------------- | ----------------------------- |
| Shared rules | [Formatting](general/formatting.md) | [Linting](general/linting.md) |
| Python       | [Formatting](python/formatting.md)  | [Linting](python/linting.md)  |
| Kotlin       | [Formatting](kotlin/formatting.md)  | [Linting](kotlin/linting.md)  |
| C++          | [Formatting](cpp/formatting.md)     | [Linting](cpp/linting.md)     |

<a id="id-registry"></a>

## ID registry

Canonical IDs use `Stylebrella-<kind>-<language>-<subsection>-<number>`. Equivalent short aliases use `SB-<kind>-<language>-<subsection>-<number>` with the tokens below.

For each kind and language, number rules within a subsection consecutively from 1 in document order. When renumbering, update both IDs, their anchors, and all references in both languages.

| Canonical token        | Alias token |
| ---------------------- | ----------- |
| `formatter` / `linter` | `F` / `L`   |
| `general`              | `G`         |
| `python`               | `P`         |
| `kotlin`               | `K`         |
| `cpp`                  | `C`         |
| `blank_lines`          | `BL`        |
| `layout`               | `LY`        |
| `mark`                 | `M`         |
| `separators`           | `SP`        |
| `imports`              | `I`         |
| `source`               | `S`         |
| `expressions`          | `E`         |
| `naming`               | `N`         |
| `json`                 | `J`         |
| `comments`             | `CM`        |
| `constants`            | `C`         |
| `loops`                | `LP`        |
| `types`                | `T`         |
| `idioms`               | `ID`        |
| `testing`              | `TS`        |
| `documentation`        | `D`         |
| `nulls`                | `NU`        |
| `files`                | `FI`        |
| `exceptions`           | `EX`        |
