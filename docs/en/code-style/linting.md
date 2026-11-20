# Shared linting

[Русская версия](../../ru/code-style/linting.md)

Normative preview contract for `lint` and the linting part of `check`. Emit a finding only when the stated predicate is established. Missing resolution or required context is not a violation. These rules describe future behavior. A full ID is canonical; its short ID is an equivalent alias. Numbering starts at 1 per subsection, and assigned numbers are not reused.

[Python](python/linting.md) · [Kotlin](kotlin/linting.md) · [C++](cpp/linting.md)

<a id="id-registry"></a>

## ID registry

Canonical IDs use `Stylebrella-<kind>-<language>-<subsection>-<number>`; aliases use `SB-<kind>-<language>-<subsection>-<number>` with the tokens below.

| Canonical token | Alias token |
| --- | --- |
| `formatter` / `linter` | `F` / `L` |
| `general` | `G` |
| `python` | `P` |
| `kotlin` | `K` |
| `cpp` | `C` |
| `blank_lines` | `BL` |
| `layout` | `LY` |
| `mark` | `M` |
| `separators` | `SP` |
| `imports` | `I` |
| `source` | `S` |
| `expressions` | `E` |
| `naming` | `N` |
| `json` | `J` |
| `comments` | `CM` |
| `constants` | `C` |
| `loops` | `LP` |
| `types` | `T` |
| `idioms` | `ID` |
| `testing` | `TS` |
| `documentation` | `D` |
| `nulls` | `NU` |
| `files` | `FI` |
| `exceptions` | `EX` |

Examples isolate the rule being illustrated. Outside documentation rules, function fragments may show only the signature or statements relevant to the rule, with required documentation outside the excerpt.

## `naming`

<a id="stylebrella-linter-general-naming-1"></a>

<a id="sb-l-g-n-1"></a>

### SB-L-G-N-1 — Naming by declaration kind

`Stylebrella-linter-general-naming-1`

Check declaration names by their syntactic kind and language-specific naming rules; report at the declared identifier. Never infer the kind from the name or rename it as a lint action.

**Python**

**Allowed**:

```python
total_count = count()
```

**Violation**:

```python
totalCount = count()
```

**Kotlin**

**Allowed**:

```kotlin
val totalCount = count()
```

**Violation**:

```kotlin
val total_count = count()
```

**C++**

**Allowed**:

```cpp
int total_count = count();
```

**Violation**:

```cpp
int totalCount = count();
```

<a id="stylebrella-linter-general-naming-2"></a>

<a id="sb-l-g-n-2"></a>

### SB-L-G-N-2 — Leading underscore on private methods

`Stylebrella-linter-general-naming-2`

Every private method name must begin with exactly one underscore followed by its language-specific name. In Kotlin and C++, private visibility is determined by the declaration. In Python, this rule applies when project metadata or an explicit `MARK: Private Helpers` section identifies the method as private; a name alone cannot establish private intent. Python special methods whose names begin and end with double underscores are excluded. Report at the method name; do not rename it automatically.

**Python** — the project marks `copy` as private.

**Allowed**:

```python
class Reader:
  # MARK: Read value
  # --------------------------------------------------

  def read(self, value: str) -> str:
    return self._copy(value)

  # MARK: Private Helpers
  # --------------------------------------------------

  def _copy(self, value: str) -> str:
    return value
```

**Violation**:

```python
class Reader:
  # MARK: Read value
  # --------------------------------------------------

  def read(self, value: str) -> str:
    return self.copy(value)

  # MARK: Private Helpers
  # --------------------------------------------------

  def copy(self, value: str) -> str:
    return value
```

**Kotlin**

**Allowed**:

```kotlin
class Reader {
  // MARK: Read value
  // --------------------------------------------------

  fun read(value: String): String = _copy(value)

  // MARK: Private Helpers
  // --------------------------------------------------

  private fun _copy(value: String): String = value
}
```

**Violation**:

```kotlin
class Reader {
  // MARK: Read value
  // --------------------------------------------------

  fun read(value: String): String = copy(value)

  // MARK: Private Helpers
  // --------------------------------------------------

  private fun copy(value: String): String = value
}
```

**C++**

**Allowed**:

```cpp
class Reader {
 public:
  // MARK: Read value
  // --------------------------------------------------

  int read(int value) const { return _copy(value); }

 private:
  // MARK: Private Helpers
  // --------------------------------------------------

  int _copy(int value) const { return value; }
};
```

**Violation**:

```cpp
class Reader {
 public:
  // MARK: Read value
  // --------------------------------------------------

  int read(int value) const { return copy(value); }

 private:
  // MARK: Private Helpers
  // --------------------------------------------------

  int copy(int value) const { return value; }
};
```

## `json`

<a id="stylebrella-linter-general-json-1"></a>

<a id="sb-l-g-j-1"></a>

### SB-L-G-J-1 — Case of statically resolved JSON keys

`Stylebrella-linter-general-json-1`

Check the case of a key only when its constant spelling is statically resolved at a serialization field or JSON accessor. Report at the key, not an arbitrary string. The language rules define the accepted case. The spelling is not rewritten and the constants rules still apply to uses outside schema metadata.

**Python**

**Allowed**:

```python
record["user_id"]
```

**Violation**:

```python
record["User-ID"]
```

**Kotlin**

**Allowed**:

```kotlin
@JsonProperty("user_id")
```

**Violation**:

```kotlin
@JsonProperty("userId")
```

**C++**

**Allowed**:

```cpp
record["user_id"]
```

**Violation**:

```cpp
record["userId"]
```

## `mark`

<a id="stylebrella-linter-general-mark-1"></a>

<a id="sb-l-g-m-1"></a>

### SB-L-G-M-1 — Descriptive MARK titles

`Stylebrella-linter-general-mark-1`

A descriptive MARK title must be nonempty, begin with an uppercase ASCII letter, and not consist solely of the targeted declaration's camelCase or PascalCase identifier. A resolved HTTP method plus route is a valid title. Strip the exact trailing group suffix before checking the title.

**Python**

**Allowed**:

```python
# MARK: Build response
```

**Violation**:

```python
# MARK: buildResponse
```

**Kotlin**

**Allowed**:

```kotlin
// MARK: Build response
```

**Violation**:

```kotlin
// MARK: buildResponse
```

**C++**

**Allowed**:

```cpp
// MARK: Build response
```

**Violation**:

```cpp
// MARK: BuildResponse
```

<a id="stylebrella-linter-general-mark-2"></a>

<a id="sb-l-g-m-2"></a>

### SB-L-G-M-2 — MARK for nontrivial public and important functions

`Stylebrella-linter-general-mark-2`

A function is nontrivial if its canonical body spans at least two executable lines, contains at least two executable statements, or contains nested control flow. It is important if at least two distinct project references resolve to it other than self-recursion; multiple references in one caller count. Require a descriptive MARK for a public nontrivial function or an important function, unless covered by a valid group or [SB-L-G-M-5](#stylebrella-linter-general-mark-5). Otherwise a separator suffices. Count body lines in the canonical format, excluding blank, comment-only, documentation-only and delimiter-only lines.

**Python**

**Allowed**:

```python
# MARK: Read data
# --------------------------------------------------
def read():
  load()
  validate()
```

**Violation**:

```python
# --------------------------------------------------
def read():
  load()
  validate()
```

**Kotlin**

**Allowed**:

```kotlin
// MARK: Read data
// --------------------------------------------------
fun read() { load(); validate() }
```

**Violation**:

```kotlin
// --------------------------------------------------
fun read() { load(); validate() }
```

**C++**

**Allowed**:

```cpp
// MARK: Read data
// --------------------------------------------------
void read() { load(); validate(); }
```

**Violation**:

```cpp
// --------------------------------------------------
void read() { load(); validate(); }
```

<a id="stylebrella-linter-general-mark-3"></a>

<a id="sb-l-g-m-3"></a>

### SB-L-G-M-3 — Counted MARK groups

`Stylebrella-linter-general-mark-3`

A `MARK: <title> [group:N]` covers exactly the next `N` sibling function or method declarations in the current scope. `N` is a decimal integer at least 2. Ignore nested definitions and nonfunction declarations. Report a malformed count, another MARK before the Nth declaration, or scope ending with fewer than N. The N+1th declaration is outside the group.

**Python**

**Allowed**:

```python
# MARK: Load adapters [group:2]
def a(): ...
def b(): ...
```

**Violation** (EOF):

```python
# MARK: Load adapters [group:2]
def a(): ...
```

**Kotlin**

**Allowed**:

```kotlin
// MARK: Load adapters [group:2]
fun a() = Unit
fun b() = Unit
```

**Violation**:

```kotlin
// MARK: Load adapters [group:1]
fun a() = Unit
```

**C++**

**Allowed**:

```cpp
// MARK: Load adapters [group:2]
void a();
void b();
```

**Violation**:

```cpp
// MARK: Load adapters [group:1]
void a();
```

<a id="stylebrella-linter-general-mark-4"></a>

<a id="sb-l-g-m-4"></a>

### SB-L-G-M-4 — Adjacent MARKs

`Stylebrella-linter-general-mark-4`

Report the second of two sibling MARKs when there is no declaration or executable statement between them. Blank lines, separators and documentation do not break adjacency. After `Private Helpers`, recommend replacing the second MARK with the language's documentation block. `Private Helpers` is structural, has no count, and remains in effect across later descriptive MARKs.

**Python**

**Allowed**:

```python
# MARK: Read
def read(): ...
# MARK: Write
```

**Violation**:

```python
# MARK: Private Helpers
# --------------------------------------------------
# MARK: Read
```

**Kotlin**

**Allowed**:

```kotlin
// MARK: Read
fun read() = Unit
// MARK: Write
```

**Violation**:

```kotlin
// MARK: Private Helpers
// --------------------------------------------------
// MARK: Read
```

**C++**

**Allowed**:

```cpp
// MARK: Read
void read();
// MARK: Write
```

**Violation**:

```cpp
// MARK: Private Helpers
// --------------------------------------------------
// MARK: Read
```

<a id="stylebrella-linter-general-mark-5"></a>

<a id="sb-l-g-m-5"></a>

### SB-L-G-M-5 — Documentation for the first important private method

`Stylebrella-linter-general-mark-5`

The structural header covers the first private method only. If that method is important, its required language documentation takes the place of an additional MARK. A documented C++ declaration satisfies the requirement for its definition. Report missing documentation under this rule instead of the language presence rule at this boundary; report malformed documentation under the language format rule.

**Python**

**Allowed**:

```python
  # MARK: Private Helpers
  def _read(self) -> None:
    """
    Заполняет кэш из последнего сохранённого снимка.
    """

    self._cache.reload_snapshot()
```

**Violation**:

```python
  # MARK: Private Helpers
  def _read(self) -> None:
    self._cache.reload_snapshot()
```

**Kotlin**

**Allowed**:

```kotlin
  // MARK: Private Helpers
  /**
   * Заполняет кэш из последнего сохранённого снимка.
   */
  private fun _read(): Unit = cache.reloadSnapshot()
```

**Violation**:

```kotlin
  // MARK: Private Helpers
  private fun _read(): Unit = cache.reloadSnapshot()
```

**C++**

**Allowed**:

```cpp
  // MARK: Private Helpers
  /**
   * @brief Заполняет кэш из последнего сохранённого снимка.
   */
  void _read();
```

**Violation**:

```cpp
  // MARK: Private Helpers
  void _read();
```

<a id="stylebrella-linter-general-mark-6"></a>

<a id="sb-l-g-m-6"></a>

### SB-L-G-M-6 — Required class MARKs

`Stylebrella-linter-general-mark-6`

A concrete class with both public and private methods needs a MARK attached to a public method and the separate structural MARK. A counted group cannot replace both. When no public method is important, any public method may carry the required public MARK. Exempt interfaces and confirmed DTO-only types; a class without private methods does not need the structural MARK.

**Python**

**Allowed**:

```python
  # MARK: Read
  def read(self): ...
  # MARK: Private Helpers
  def _read(self): ...
```

**Violation**:

```python
  def read(self): ...
  # MARK: Private Helpers
  def _read(self): ...
```

**Kotlin**

**Allowed**:

```kotlin
  // MARK: Read
  fun read() = Unit
  // MARK: Private Helpers
  private fun _helper() = Unit
```

**Violation**:

```kotlin
  fun read() = Unit
  // MARK: Private Helpers
  private fun _helper() = Unit
```

**C++**

**Allowed**:

```cpp
 public:
  // MARK: Read
  void read();
 private:
  // MARK: Private Helpers
  void _read();
```

**Violation**:

```cpp
 public:
  void read();
 private:
  // MARK: Private Helpers
  void _read();
```

<a id="stylebrella-linter-general-mark-7"></a>

<a id="sb-l-g-m-7"></a>

### SB-L-G-M-7 — Private methods after public methods

`Stylebrella-linter-general-mark-7`

In an eligible class under [SB-L-G-M-6](#stylebrella-linter-general-mark-6), a private method must not precede any public method. Report the early private declaration; do not reorder declarations. Visibility is defined by the language, not by observed use.

**Python**

**Allowed**:

```python
  def read(self): ...
  def _read(self): ...
```

**Violation**:

```python
  def _read(self): ...
  def read(self): ...
```

**Kotlin**

**Allowed**:

```kotlin
  fun read() = Unit
  private fun _helper() = Unit
```

**Violation**:

```kotlin
  private fun _helper() = Unit
  fun read() = Unit
```

**C++**

**Allowed**:

```cpp
public: void read();
private: void _read();
```

**Violation**:

```cpp
private: void _read();
public: void read();
```

## `documentation`

<a id="stylebrella-linter-general-documentation-1"></a>

<a id="sb-l-g-d-1"></a>

### SB-L-G-D-1 — Only formal documentation may introduce a function

`Stylebrella-linter-general-documentation-1`

Require documentation for every named function and method, including private, local, and trivial ones. Check both presence and format: [Python docstrings](python/linting.md#stylebrella-linter-python-documentation-1), [KDoc](kotlin/linting.md#stylebrella-linter-kotlin-documentation-1), and [Doxygen](cpp/linting.md#stylebrella-linter-cpp-documentation-1). A summary is always required and must explain the purpose, effect, or guarantee, not merely restate the function name. Require parameter and result descriptions only when applicable; never require empty sections. Report missing documentation at the declaration and malformed documentation at its block, rather than reporting a present but malformed block as missing.

Report any ordinary comment attached above a function, even when valid documentation is also present. A comment is attached if it is in the contiguous leading group at the function's indentation, without a blank line before its declaration or attached annotations/decorators; annotations and decorators do not break the group. Allow structural MARKs, separators, and recognized tool directives in their own positions. Python has no external function documentation: the docstring belongs inside the body. Detached comments about surrounding code are outside this rule.

**Python — violation** (the docstring does not make the leading comment valid):

```python
# start tasks
def start_tasks() -> None:
  """
  Передаёт планировщику только задачи, готовые к выполнению.
  """

  scheduler.enqueue_ready()
```

**Kotlin — violation** (ordinary comment before KDoc):

```kotlin
// start tasks
/**
 * Передаёт планировщику только задачи, готовые к выполнению.
 */
fun startTasks(): Unit = scheduler.enqueueReady()
```

**C++ — violation** (ordinary comment before Doxygen):

```cpp
// start tasks
/**
 * @brief Передаёт планировщику только задачи, готовые к выполнению.
 */
void start_tasks();
```

## `comments`

<a id="stylebrella-linter-general-comments-1"></a>

<a id="sb-l-g-cm-1"></a>

### SB-L-G-CM-1 — Inline comments on fields

`Stylebrella-linter-general-comments-1`

For fields selected by the language-specific rules, require an inline comment with non-whitespace text after the declaration on the same line, except for explicit name exemptions. Report at the field; do not infer quality or units.

**Python**

**Allowed**:

```python
df_hz: float = 0.0  # offset, Hz
```

**Violation**:

```python
df_hz: float = 0.0
```

**Kotlin**

**Allowed**:

```kotlin
val dfHz: Double = 0.0, // offset, Hz
```

**Violation**:

```kotlin
val dfHz: Double = 0.0,
```

**C++**

**Allowed**:

```cpp
double df_hz = 0.0; // offset, Hz
```

**Violation**:

```cpp
double df_hz = 0.0;
```

## `constants`

<a id="stylebrella-linter-general-constants-1"></a>

<a id="sb-l-g-c-1"></a>

### SB-L-G-C-1 — Named values instead of inline literals

`Stylebrella-linter-general-constants-1`

Report inline numeric and string literals in runtime defaults, call arguments, returned/assigned values, expressions or test data unless a documented exception applies. Allow booleans and null, and numeric zero/one or the empty string only as direct initializers, including typed equivalents. A literal in a named constant's definition is not an inline use. Report the literal; never extract or deduplicate automatically.

**Python**

**Allowed**:

```python
connect(Constants.Network.DEFAULT_PORT)
```

**Violation**:

```python
connect(8080)
```

**Kotlin**

**Allowed**:

```kotlin
connect(Constants.Network.DEFAULT_PORT)
```

**Violation**:

```kotlin
connect(8080)
```

**C++**

**Allowed**:

```cpp
connect(app::constants::network::DEFAULT_PORT);
```

**Violation**:

```cpp
connect(8080);
```

<a id="stylebrella-linter-general-constants-2"></a>

<a id="sb-l-g-c-2"></a>

### SB-L-G-C-2 — Literal exceptions for messages and metadata

`Stylebrella-linter-general-constants-2`

Exempt literal message text in statically resolved output, logging or exception message parameters, including the text parts of interpolated messages. Data passed as separate arguments or interpolated expressions is not exempt. Also exempt language-specific schema and syntax metadata explicitly listed by the applicable contract.

**Python**

**Allowed**:

```python
logger.info("Connected")
```

**Violation**:

```python
logger.info("Port: %d", 8080)
```

**Kotlin**

**Allowed**:

```kotlin
logger.info("Connected")
```

**Violation**:

```kotlin
logger.info("Port: {}", 8080)
```

**C++**

**Allowed**:

```cpp
throw std::runtime_error("[Network::connect] Disconnected: peer=" + peer);
```

**Violation**:

```cpp
throw std::runtime_error("[Network::connect] Disconnected: port=" + std::to_string(8080));
```

## `loops`

<a id="stylebrella-linter-general-loops-1"></a>

<a id="sb-l-g-lp-1"></a>

### SB-L-G-LP-1 — For loops instead of equivalent counter loops

`Stylebrella-linter-general-loops-1`

Report a `while` at its header when its counter is initialized to zero immediately before it, tests `<` against the size of an unchanged collection, increments once by one at the end of every iteration, and has no other counter use except indexing that collection or preserving the final counter. If the final counter is used after the loop, any update, early exit, or side effect may change equivalence, or resolution is missing, do not report. Do not rewrite the loop.

**Python**

**Allowed**:

```python
for item in items: visit(item)
```

**Violation**:

```python
i = 0
while i < len(items):
  visit(items[i])
  i += 1
```

**Kotlin**

**Allowed**:

```kotlin
for (item in items) visit(item)
```

**Violation**:

```kotlin
var i = 0
while (i < items.size) { visit(items[i]); i++ }
```

**C++**

**Allowed**:

```cpp
for (const auto& item : items) visit(item);
```

**Violation**:

```cpp
int i = 0;
while (i < items.size()) { visit(items[i]); ++i; }
```

## `imports`

<a id="stylebrella-linter-general-imports-1"></a>

<a id="sb-l-g-i-1"></a>

### SB-L-G-I-1 — Imports and includes in the file preamble

`Stylebrella-linter-general-imports-1`

Imports and includes must appear in the initial file preamble, never after the first executable statement or declaration. The preamble allows comments, file directives (including module docstrings, Kotlin file annotations/package, C++ `#pragma once`), and conditional branches that contain only preamble items. Imports in functions or other non-file scopes are late. Report at the directive; formatting does not move it.

**Python**

**Allowed**:

```python
import sys


# --------------------------------------------------


def run() -> None:
  sys.exit()
```

**Violation**:

```python
def run() -> None:
  sys.exit()


import sys
```

**Kotlin**

**Allowed**:

```kotlin
package app
import java.time.Clock


// --------------------------------------------------


val clock = Clock.systemUTC()
```

**Violation**:

```kotlin
package app
val clock = Clock.systemUTC()


import java.time.Clock
```

**C++**

**Allowed**:

```cpp
#pragma once
#include <vector>


// --------------------------------------------------


void run();
```

**Violation**:

```cpp
#pragma once
void run();


#include <vector>
```

## Recommendations (no diagnostic)

Choose meaningful names and include units where useful. Describe intent in comments and put units in field explanations. Prefer Russian for ordinary new or changed comments; English is also allowed. Use English descriptions in MARKs. Consolidate values that mean the same thing, not merely values with the same spelling. Use `for` for simple traversal; keep `while` when state or termination is not equivalent. Keep executable logic independent of import order.
