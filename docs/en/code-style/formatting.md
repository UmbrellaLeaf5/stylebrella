# Shared formatting

[Русская версия](../../ru/code-style/formatting.md)

Normative preview contract for `format`, `format --check` (read-only), and the formatting part of `check`. These rules describe intended behavior, not implementation status. Each language uses one canonical formatter, without chaining alternative formatters. After Stylebrella formatting, a second Stylebrella pass and a pass of the language formatter with the same Stylebrella preset must each leave the bytes unchanged. An incompatibility blocks support for the affected rule until its contract or the formatter selection is revised.

[Python](python/formatting.md) · [Kotlin](kotlin/formatting.md) · [C++](cpp/formatting.md)

[ID registry and linting](linting.md#id-registry)

Examples isolate the rule being illustrated.

## `blank_lines`

<a id="stylebrella-formatter-general-blank_lines-1"></a>

<a id="sb-f-g-bl-1"></a>

### SB-F-G-BL-1 — Blank lines around the import separator

`Stylebrella-formatter-general-blank_lines-1`

At file scope, put exactly two blank lines before and after the separator required by [SB-F-G-SP-1](#stylebrella-formatter-general-separators-1). If the separator belongs to a MARK, the two lines before it go before the MARK itself; [SB-F-G-BL-9](#stylebrella-formatter-general-blank_lines-9) governs the spacing after it where applicable.

**Python**

**Before**:

```python
import time
# --------------------------------------------------
def run() -> float:
  return time.monotonic()
```

**After**:

```python
import time


# --------------------------------------------------


def run() -> float:
  return time.monotonic()
```

**Kotlin**

**Before**:

```kotlin
import java.time.Instant
// --------------------------------------------------
fun run() = Instant.now()
```

**After**:

```kotlin
import java.time.Instant


// --------------------------------------------------


fun run() = Instant.now()
```

**C++**

**Before**:

```cpp
#include "Clock.hpp"
// --------------------------------------------------
void run();
```

**After**:

```cpp
#include "Clock.hpp"


// --------------------------------------------------


void run();
```

<a id="stylebrella-formatter-general-blank_lines-2"></a>

<a id="sb-f-g-bl-2"></a>

### SB-F-G-BL-2 — Blank lines between top-level declarations

`Stylebrella-formatter-general-blank_lines-2`

At file scope, put two blank lines between adjacent declaration blocks and on each side of their separator, except at a file boundary. Documentation, decorators, and annotations travel with the declaration. The MARK-to-documentation exception is specified in [SB-F-G-BL-9](#stylebrella-formatter-general-blank_lines-9).

**Python**

**Before**:

```python
def a() -> None:
  flush()
# --------------------------------------------------
def b() -> None:
  close()
```

**After**:

```python
def a() -> None:
  flush()


# --------------------------------------------------


def b() -> None:
  close()
```

**Kotlin**

**Before**:

```kotlin
fun a() = Unit
// --------------------------------------------------
fun b() = Unit
```

**After**:

```kotlin
fun a() = Unit


// --------------------------------------------------


fun b() = Unit
```

**C++**

**Before**:

```cpp
void a();
// --------------------------------------------------
void b();
```

**After**:

```cpp
void a();


// --------------------------------------------------


void b();
```

<a id="stylebrella-formatter-general-blank_lines-3"></a>

<a id="sb-f-g-bl-3"></a>

### SB-F-G-BL-3 — Consecutive blank lines inside classes and functions

`Stylebrella-formatter-general-blank_lines-3`

Inside a class or function, collapse runs of two or more blank lines to one. Do not add blank lines between a documentation block, attached annotations or decorators, and its declaration. At a boundary with a specified count, use that count once, not the sum of matching rules.

**Python**

**Before**:

```python
# MARK: Run tasks
# --------------------------------------------------


def f() -> None:
  """
  Запускает задачи.
  """

  a()


  b()
```

**After**:

```python
# MARK: Run tasks
# --------------------------------------------------


def f() -> None:
  """
  Запускает задачи.
  """

  a()

  b()
```

**Kotlin**

**Before**:

```kotlin
// MARK: Run tasks
// --------------------------------------------------

/**
 * Запускает задачи.
 */
fun f() {
  a()


  b()
}
```

**After**:

```kotlin
// MARK: Run tasks
// --------------------------------------------------

/**
 * Запускает задачи.
 */
fun f() {
  a()

  b()
}
```

**C++**

**Before**:

```cpp
// MARK: Run tasks
// --------------------------------------------------

/**
 * @brief Запускает задачи.
 */
void f() {
  a();


  b();
}
```

**After**:

```cpp
// MARK: Run tasks
// --------------------------------------------------

/**
 * @brief Запускает задачи.
 */
void f() {
  a();

  b();
}
```

<a id="stylebrella-formatter-general-blank_lines-4"></a>

<a id="sb-f-g-bl-4"></a>

### SB-F-G-BL-4 — Spacing around local declaration groups

`Stylebrella-formatter-general-blank_lines-4`

In a statement list, a run of adjacent local declarations is one group. Direct method calls or field assignments whose receiver resolves to the last declared local extend its group. Separate that group by one blank line from adjacent statements outside it; preserve explicit comment boundaries. Python field-assignment runs use [SB-F-P-BL-3](python/formatting.md#stylebrella-formatter-python-blank_lines-3) instead.

**Python**

**Before**:

```python
  log()
  f = Filter()

  f.init()
```

**After**:

```python
  log()

  f = Filter()
  f.init()
```

**Kotlin**

**Before**:

```kotlin
  log()
  val f = Filter()

  f.init()
```

**After**:

```kotlin
  log()

  val f = Filter()
  f.init()
```

**C++**

**Before**:

```cpp
  log();
  Filter f;

  f.init();
```

**After**:

```cpp
  log();

  Filter f;
  f.init();
```

<a id="stylebrella-formatter-general-blank_lines-5"></a>

<a id="sb-f-g-bl-5"></a>

### SB-F-G-BL-5 — Spacing between field declarations

`Stylebrella-formatter-general-blank_lines-5`

Keep consecutive field declarations together unless the next field has a leading comment or annotation; in that case use one blank line before the attached group.

**Python**

**Before**:

```python
  x: int

  y: int
```

**After**:

```python
  x: int
  y: int
```

**Kotlin**

**Before**:

```kotlin
  val x: Int

  val y: Int
```

**After**:

```kotlin
  val x: Int
  val y: Int
```

**C++**

**Before**:

```cpp
  int x;

  int y;
```

**After**:

```cpp
  int x;
  int y;
```

<a id="stylebrella-formatter-general-blank_lines-6"></a>

<a id="sb-f-g-bl-6"></a>

### SB-F-G-BL-6 — Blank line after a multiline statement

`Stylebrella-formatter-general-blank_lines-6`

After a statement spanning multiple code lines, insert one blank line before the next sibling statement. Do not insert one before a closing delimiter. The uninterrupted declaration group of [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4) takes precedence within that group.

**Python**

**Before**:

```python
  f(
    x,
  )
  g()
```

**After**:

```python
  f(
    x,
  )

  g()
```

**Kotlin**

**Before**:

```kotlin
  f(
    x,
  )
  g()
```

**After**:

```kotlin
  f(
    x,
  )

  g()
```

**C++**

**Before**:

```cpp
  f(
    x);
  g();
```

**After**:

```cpp
  f(
    x);

  g();
```

<a id="stylebrella-formatter-general-blank_lines-7"></a>

<a id="sb-f-g-bl-7"></a>

### SB-F-G-BL-7 — Blank lines around control flow and between branches

`Stylebrella-formatter-general-blank_lines-7`

Separate an `if` or `for` from adjacent sibling statements by one blank line, including after an unbraced single-statement body. Separate successive conditional and exception-handler branches by one blank line. Do not put a blank line just inside a closing delimiter or between a control header and its single-line body.

**Python**

**Before**:

```python
  if x: f()
  g()
```

**After**:

```python
  if x: f()

  g()
```

**Kotlin**

**Before**:

```kotlin
  if (x) f()
  g()
```

**After**:

```kotlin
  if (x) f()

  g()
```

**C++**

**Before**:

```cpp
  if (x) f();
  g();
```

**After**:

```cpp
  if (x) f();

  g();
```

<a id="stylebrella-formatter-general-blank_lines-8"></a>

<a id="sb-f-g-bl-8"></a>

### SB-F-G-BL-8 — Blank line before return

`Stylebrella-formatter-general-blank_lines-8`

Insert one blank line before `return` following another sibling statement. A sole statement in a block or an inline control body has no such preceding blank line.

**Python**

**Before**:

```python
  send()
  return result
```

**After**:

```python
  send()

  return result
```

**Kotlin**

**Before**:

```kotlin
  send()
  return result
```

**After**:

```kotlin
  send()

  return result
```

**C++**

**Before**:

```cpp
  send();
  return result;
```

**After**:

```cpp
  send();

  return result;
```

<a id="stylebrella-formatter-general-blank_lines-9"></a>

<a id="sb-f-g-bl-9"></a>

### SB-F-G-BL-9 — Blank lines after a MARK separator

`Stylebrella-formatter-general-blank_lines-9`

Between a MARK separator and attached KDoc or Doxygen use exactly one blank line, even at file scope. In Python place one blank line before the attached declaration or decorator if stable under the active preset; use two only where that preset changes one to two. A Python docstring stays inside the declaration body.

**Python**

**Before**:

```python
class C:

  # MARK: Read input
  # --------------------------------------------------
  def read(self) -> None:
    """
    Читает входные данные.
    """

    load()

  # (no private methods)
```

**After**:

```python
class C:

  # MARK: Read input
  # --------------------------------------------------

  def read(self) -> None:
    """
    Читает входные данные.
    """

    load()

  # (no private methods)
```

**Kotlin**

**Before**:

```kotlin
// MARK: Read input
// --------------------------------------------------
/**
 * Читает входные данные.
 *
 * @return входные данные
 */
fun read() = input()
```

**After**:

```kotlin
// MARK: Read input
// --------------------------------------------------

/**
 * Читает входные данные.
 *
 * @return входные данные
 */
fun read() = input()
```

**C++**

**Before**:

```cpp
// MARK: Read input
// --------------------------------------------------
/**
 * @brief Читает входные данные.
 * @return входные данные
 */
int read();
```

**After**:

```cpp
// MARK: Read input
// --------------------------------------------------

/**
 * @brief Читает входные данные.
 * @return входные данные
 */
int read();
```

<a id="stylebrella-formatter-general-blank_lines-10"></a>

<a id="sb-f-g-bl-10"></a>

### SB-F-G-BL-10 — No blank lines between MARK and its separator

`Stylebrella-formatter-general-blank_lines-10`

When a MARK and its separator have only blank lines between them, remove all those lines. This rule does not insert a missing separator; see [SB-F-G-M-3](#sb-f-g-m-3).

**Python**

**Before**:

```python
# MARK: Build response

# --------------------------------------------------
```

**After**:

```python
# MARK: Build response
# --------------------------------------------------
```

**Kotlin**

**Before**:

```kotlin
// MARK: Build response

// --------------------------------------------------
```

**After**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**C++**

**Before**:

```cpp
// MARK: Build response

// --------------------------------------------------
```

**After**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

## `layout`

<a id="stylebrella-formatter-general-layout-1"></a>

<a id="sb-f-g-ly-1"></a>

### SB-F-G-LY-1 — Two-space block indentation

`Stylebrella-formatter-general-layout-1`

Indent each syntactic block level by exactly two ASCII spaces. Do not modify whitespace inside literals.

**Python**

**Before**:

```python
if ready:
    send()
```

**After**:

```python
if ready:
  send()
```

**Kotlin**

**Before**:

```kotlin
if (ready) {
    send()
}
```

**After**:

```kotlin
if (ready) {
  send()
}
```

**C++**

**Before**:

```cpp
if (ready) {
    send();
}
```

**After**:

```cpp
if (ready) {
  send();
}
```

<a id="stylebrella-formatter-general-layout-2"></a>

<a id="sb-f-g-ly-2"></a>

### SB-F-G-LY-2 — Wrapping long calls and signatures

`Stylebrella-formatter-general-layout-2`

Use a 100-column wrapping limit for calls and signatures. If their single-line form would exceed it, use hanging indentation with one item per continuation line at two extra spaces, subject to the language closing-delimiter rules. Do not split an indivisible token even if the line stays over the limit.

**Python**

**Before**:

```python
run_request_processing_pipeline(project_configuration, request_processing_settings, transaction_logging_context)
```

**After**:

```python
run_request_processing_pipeline(
  project_configuration,
  request_processing_settings,
  transaction_logging_context,
)
```

**Kotlin**

**Before**:

```kotlin
run_request_processing_pipeline(project_configuration, request_processing_settings, transaction_logging_context)
```

**After**:

```kotlin
run_request_processing_pipeline(
  project_configuration,
  request_processing_settings,
  transaction_logging_context,
)
```

**C++**

**Before**:

```cpp
run_request_processing_pipeline(project_configuration, request_processing_settings, transaction_logging_context);
```

**After**:

```cpp
run_request_processing_pipeline(
  project_configuration,
  request_processing_settings,
  transaction_logging_context);
```

## `separators`

The existing `SB-F-G-M-1` and `SB-F-G-M-2` identifiers are retained after moving the two rules into this section.

<a id="stylebrella-formatter-general-mark-1"></a>

<a id="sb-f-g-m-1"></a>

### SB-F-G-M-1 — Separator syntax and length

`Stylebrella-formatter-general-mark-1`

A separator is a standalone comment consisting of the language prefix (`#` or `//`), one space, and exactly 50 ASCII hyphens. Normalize a standalone dash-only comment of at least two hyphens at a separator position; preserve other comments.

**Python**

**Before**:

```python
# ----
```

**After**:

```python
# --------------------------------------------------
```

**Kotlin**

**Before**:

```kotlin
// ----
```

**After**:

```kotlin
// --------------------------------------------------
```

**C++**

**Before**:

```cpp
// ----
```

**After**:

```cpp
// --------------------------------------------------
```

<a id="stylebrella-formatter-general-mark-2"></a>

<a id="sb-f-g-m-2"></a>

### SB-F-G-M-2 — Separators between functions and before the first method

`Stylebrella-formatter-general-mark-2`

Insert a separator between adjacent same-scope functions or methods and before the first method of a class. Place it before attached documentation or annotations and, in C++, after any access label opening that method's section. An already present MARK-block separator satisfies the requirement. Inside the class, put one blank line on each side that has a neighboring line of content, except between a MARK and its separator.

**Python**

**Before**:

```python
class C:
  def f(self) -> None:
    run()

  # (no private methods)
```

**After**:

```python
class C:

  # --------------------------------------------------

  def f(self) -> None:
    run()

  # (no private methods)
```

**Kotlin**

**Before**:

```kotlin
class C {
  fun f() = Unit

  // (no private methods)
}
```

**After**:

```kotlin
class C {

  // --------------------------------------------------

  fun f() = Unit

  // (no private methods)
}
```

**C++**

**Before**:

```cpp
class C {
 public:
  void f();

  // (no private methods)
};
```

**After**:

```cpp
class C {
 public:

  // --------------------------------------------------

  void f();

  // (no private methods)
};
```

<a id="stylebrella-formatter-general-separators-1"></a>

<a id="sb-f-g-sp-1"></a>

### SB-F-G-SP-1 — Separator after imports and includes

`Stylebrella-formatter-general-separators-1`

When a file has leading imports or includes followed by a file-scope declaration, require exactly one separator at that boundary. Insert it before the first declaration and its attached documentation, decorators, or annotations. If a MARK already belongs to the declaration, use its separator instead; do not insert a second one. Apply the spacing in [SB-F-G-BL-1](#stylebrella-formatter-general-blank_lines-1).

**Python**

**Before**:

```python
import time


def run() -> float:
  return time.monotonic()
```

**After**:

```python
import time


# --------------------------------------------------


def run() -> float:
  return time.monotonic()
```

**Kotlin**

**Before**:

```kotlin
import java.time.Instant


fun run() = Instant.now()
```

**After**:

```kotlin
import java.time.Instant


// --------------------------------------------------


fun run() = Instant.now()
```

**C++** (`Clock.cpp`)

**Before**:

```cpp
#include "Clock.hpp"


void run();
```

**After**:

```cpp
#include "Clock.hpp"


// --------------------------------------------------


void run();
```

## `mark`

<a id="stylebrella-formatter-general-mark-3"></a>

<a id="sb-f-g-m-3"></a>

### SB-F-G-M-3 — Required separator after MARK

`Stylebrella-formatter-general-mark-3`

Every standalone MARK must be followed by a separator at the same indentation. If the next nonblank line is not its separator, insert one on the next line after the MARK. If the separator already exists, keep it and apply [SB-F-G-BL-10](#sb-f-g-bl-10) to any intervening blank lines. Preserve the title text and group count.

**Python**

**Before**:

```python
# MARK: Build response
```

**After**:

```python
# MARK: Build response
# --------------------------------------------------
```

**Kotlin**

**Before**:

```kotlin
// MARK: Build response
```

**After**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**C++**

**Before**:

```cpp
// MARK: Build response
```

**After**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

<a id="stylebrella-formatter-general-mark-5"></a>

<a id="sb-f-g-m-5"></a>

### SB-F-G-M-5 — Private Helpers section

`Stylebrella-formatter-general-mark-5`

For a concrete class with at least one public and one private method, put a single `MARK: Private Helpers` and separator before its first private method. Remove duplicate structural blocks and obsolete `(no private methods)` comments. Preserve access labels, member order, and all descriptive MARKs, even when lint reports adjacent MARKs. Do not apply to interfaces or types positively identified as DTO-only.

**Python**

**Before**:

```python
class Reader:
  # MARK: Read input
  # --------------------------------------------------

  def read(self, value: str) -> str:
    return self._copy(value)

  def write(self, value: str) -> int:
    return len(value)

  def _copy(self, value: str) -> str:
    return value
```

**After**:

```python
class Reader:
  # MARK: Read input
  # --------------------------------------------------

  def read(self, value: str) -> str:
    return self._copy(value)

  def write(self, value: str) -> int:
    return len(value)

  # MARK: Private Helpers
  # --------------------------------------------------

  def _copy(self, value: str) -> str:
    return value
```

**Kotlin**

**Before**:

```kotlin
class Reader {
  // MARK: Read input
  // --------------------------------------------------

  fun read(value: String): String = _copy(value)

  fun write(value: String): Int = value.length

  private fun _copy(value: String): String = value
}
```

**After**:

```kotlin
class Reader {
  // MARK: Read input
  // --------------------------------------------------

  fun read(value: String): String = _copy(value)

  fun write(value: String): Int = value.length

  // MARK: Private Helpers
  // --------------------------------------------------

  private fun _copy(value: String): String = value
}
```

**C++**

**Before**:

```cpp
class Reader {
 public:
  // MARK: Read input
  // --------------------------------------------------

  int read(int value) const { return _copy(value); }

  int write(int value) const { return value; }

 private:
  int _copy(int value) const { return value; }
};
```

**After**:

```cpp
class Reader {
 public:
  // MARK: Read input
  // --------------------------------------------------

  int read(int value) const { return _copy(value); }

  int write(int value) const { return value; }

 private:
  // MARK: Private Helpers
  // --------------------------------------------------

  int _copy(int value) const { return value; }
};
```

<a id="stylebrella-formatter-general-mark-6"></a>

<a id="sb-f-g-m-6"></a>

### SB-F-G-M-6 — Placeholder for classes without private methods

`Stylebrella-formatter-general-mark-6`

In the same eligible class with public but no private methods, retain exactly one `(no private methods)` standalone comment after the last public method, separated by one blank line. Replace stale structural blocks and remove duplicate placeholders.

**Python**

**Before**:

```python
  def read(self) -> None:
    load()
```

**After**:

```python
  def read(self) -> None:
    load()

  # (no private methods)
```

**Kotlin**

**Before**:

```kotlin
  fun read() = Unit
```

**After**:

```kotlin
  fun read() = Unit

  // (no private methods)
```

**C++**

**Before**:

```cpp
  void read();
```

**After**:

```cpp
  void read();

  // (no private methods)
```

<a id="stylebrella-formatter-general-mark-7"></a>

<a id="sb-f-g-m-7"></a>

### SB-F-G-M-7 — One space after `MARK:`

`Stylebrella-formatter-general-mark-7`

When a MARK has a title, use exactly one ASCII space between `MARK:` and the first non-whitespace character of the title. Replace missing spaces, extra spaces, or tabs at that boundary. Do not invent a title for an empty MARK.

**Python**

**Before**:

```python
# MARK:Build response
# --------------------------------------------------
```

**After**:

```python
# MARK: Build response
# --------------------------------------------------
```

**Kotlin**

**Before**:

```kotlin
// MARK:  Build response
// --------------------------------------------------
```

**After**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**C++**

**Before**:

```cpp
// MARK:Build response
// --------------------------------------------------
```

**After**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

## `comments`

<a id="stylebrella-formatter-general-comments-1"></a>

<a id="sb-f-g-cm-1"></a>

### SB-F-G-CM-1 — Space after the line-comment prefix

`Stylebrella-formatter-general-comments-1`

The first character after `#` in a Python line comment or `//` in a Kotlin or C++ line comment must be an ASCII space if the comment has content. This applies to standalone and trailing comments, including MARKs and separators. If that character is missing or is not a space, insert a space before the text or replace the first whitespace character; preserve the remaining comment text. A prefix-only comment stays bare. Preserve special prefixes with their own syntax, such as a Python shebang (`#!`) or C++ line documentation (`///`, `//!`). This rule applies only to comment tokens, not to strings or C++ preprocessor directives.

**Python**

**Before**:

```python
#Load the saved settings
```

**After**:

```python
# Load the saved settings
```

**Kotlin**

**Before**:

```kotlin
//Load the saved settings
```

**After**:

```kotlin
// Load the saved settings
```

**C++**

**Before**:

```cpp
//Load the saved settings
```

**After**:

```cpp
// Load the saved settings
```

## Recommendations (no diagnostic)

Use explicit breaks or group comments when the intended layout cannot be inferred. Prefer short argument names only when their meaning remains clear. Never break indivisible tokens or change literal values to meet the width.
