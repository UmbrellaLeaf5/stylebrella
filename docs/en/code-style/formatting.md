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

At file scope, put exactly two blank lines before and after the separator required by [SB-F-G-SP-3](#stylebrella-formatter-general-separators-3). If the separator belongs to a MARK, the two lines before it go before the MARK itself; [SB-F-G-BL-9](#stylebrella-formatter-general-blank_lines-9) governs the spacing after it where applicable.

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

### SB-F-G-BL-3 — Consecutive blank lines inside function and method bodies

`Stylebrella-formatter-general-blank_lines-3`

Inside function and method bodies, collapse each run of two or more blank lines between statements to one. This applies equally to standalone functions and class methods. It does not set spacing between method declarations; use [SB-F-G-SP-2](#stylebrella-formatter-general-separators-2) for that spacing and for the separator before the first method. Do not add blank lines between a documentation block, attached annotations or decorators, and its declaration. At a boundary with a specified count, use that count once, not the sum of matching rules.

**Python**

**Before**:

```python
class C:

  # --------------------------------------------------

  def run(self) -> None:
    send()


    log()

  # (no private methods)


def f() -> None:
  start()


  finish()
```

**After**:

```python
class C:

  # --------------------------------------------------

  def run(self) -> None:
    send()

    log()

  # (no private methods)


def f() -> None:
  start()

  finish()
```

**Kotlin**

**Before**:

```kotlin
class C {

  // --------------------------------------------------

  fun run() {
    send()


    log()
  }

  // (no private methods)
}


fun f() {
  start()


  finish()
}
```

**After**:

```kotlin
class C {

  // --------------------------------------------------

  fun run() {
    send()

    log()
  }

  // (no private methods)
}


fun f() {
  start()

  finish()
}
```

**C++**

**Before**:

```cpp
class C {
 public:

  // --------------------------------------------------

  void run() {
    send();


    log();
  }

  // (no private methods)
};


void f() {
  start();


  finish();
}
```

**After**:

```cpp
class C {
 public:

  // --------------------------------------------------

  void run() {
    send();

    log();
  }

  // (no private methods)
};


void f() {
  start();

  finish();
}
```

<a id="stylebrella-formatter-general-blank_lines-4"></a>

<a id="sb-f-g-bl-4"></a>

### SB-F-G-BL-4 — Spacing around local declaration and receiver-operation groups

`Stylebrella-formatter-general-blank_lines-4`

In a statement list, keep adjacent local declarations together when neither starts a receiver-operation group. Keep a local declaration with consecutive direct method calls, field assignments, reassignments that target that same local, and direct calls whose sole argument is that local (for example, `use(f)`). Separate each such group from unrelated adjacent statements by one blank line. A call with additional arguments, such as `use(f, g)`, or a call through another receiver, such as `g.func(f, a)`, starts an unrelated group. Preserve explicit comment boundaries.

**Python**

**Before**:

```python
  f: SomeClass
  f.init()
  f.x = x
  f = transform(f)
  use(f)
  close()
```

```python
  a = A()
  b = B()
  c = C()
  c.x = x
```

```python
  a = A()
  a.x = x
  b = B()
  b.x = x
```

```python
  a = A()
  b = B()
  b.init()
```

```python
  f: SomeClass
  f.init()
  f.x = x
  use(f, g)
```

```python
  f: SomeClass
  f.init()
  f.x = x
  g.func(f, a)
```

**After**:

```python
  f: SomeClass
  f.init()
  f.x = x
  f = transform(f)
  use(f)

  close()
```

```python
  a = A()
  b = B()

  c = C()
  c.x = x
```

```python
  a = A()
  a.x = x

  b = B()
  b.x = x
```

```python
  a = A()

  b = B()
  b.init()
```

```python
  f: SomeClass
  f.init()
  f.x = x

  use(f, g)
```

```python
  f: SomeClass
  f.init()
  f.x = x

  g.func(f, a)
```

**Kotlin**

**Before**:

```kotlin
  var f: SomeClass = SomeClass()
  f.init()
  f.x = x
  f = transform(f)
  use(f)
  close()
```

```kotlin
  val a = A()
  val b = B()
  val c = C()
  c.x = x
```

```kotlin
  val a = A()
  a.x = x
  val b = B()
  b.x = x
```

```kotlin
  val a = A()
  val b = B()
  b.init()
```

```kotlin
  var f: SomeClass = SomeClass()
  f.init()
  f.x = x
  use(f, g)
```

```kotlin
  var f: SomeClass = SomeClass()
  f.init()
  f.x = x
  g.func(f, a)
```

**After**:

```kotlin
  var f: SomeClass = SomeClass()
  f.init()
  f.x = x
  f = transform(f)
  use(f)

  close()
```

```kotlin
  val a = A()
  val b = B()

  val c = C()
  c.x = x
```

```kotlin
  val a = A()
  a.x = x

  val b = B()
  b.x = x
```

```kotlin
  val a = A()

  val b = B()
  b.init()
```

```kotlin
  var f: SomeClass = SomeClass()
  f.init()
  f.x = x

  use(f, g)
```

```kotlin
  var f: SomeClass = SomeClass()
  f.init()
  f.x = x

  g.func(f, a)
```

**C++**

**Before**:

```cpp
  SomeClass f;
  f.init();
  f.x = x;
  f = transform(f);
  use(f);
  close();
```

```cpp
  A a;
  B b;
  C c;
  c.x = x;
```

```cpp
  A a;
  a.x = x;
  B b;
  b.x = x;
```

```cpp
  A a;
  B b;
  b.init();
```

```cpp
  SomeClass f;
  f.init();
  f.x = x;
  use(f, g);
```

```cpp
  SomeClass f;
  f.init();
  f.x = x;
  g.func(f, a);
```

**After**:

```cpp
  SomeClass f;
  f.init();
  f.x = x;
  f = transform(f);
  use(f);

  close();
```

```cpp
  A a;
  B b;

  C c;
  c.x = x;
```

```cpp
  A a;
  a.x = x;

  B b;
  b.x = x;
```

```cpp
  A a;

  B b;
  b.init();
```

```cpp
  SomeClass f;
  f.init();
  f.x = x;

  use(f, g);
```

```cpp
  SomeClass f;
  f.init();
  f.x = x;

  g.func(f, a);
```

<a id="stylebrella-formatter-general-blank_lines-5"></a>

<a id="sb-f-g-bl-5"></a>

### SB-F-G-BL-5 — Spacing before comment-attached declarations

`Stylebrella-formatter-general-blank_lines-5`

Keep consecutive class-field and local-variable declarations together unless the next declaration has a leading comment or annotation; in that case put one blank line before the attached comment or annotation. For local variables, this boundary takes precedence over the grouping in [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4). A leading comment or annotation is treated as attached to the declaration.

**Python**

**Before**:

```python
class C:
  x: int

  y: int

  # legacy value
  z: int
```

**After**:

```python
class C:
  x: int
  y: int

  # legacy value
  z: int
```

**Before — local variables**:

```python
def build() -> None:
  a = A()
  # configured dependency
  b = B()
```

**After**:

```python
def build() -> None:
  a = A()

  # configured dependency
  b = B()
```

**Kotlin**

**Before**:

```kotlin
class C {
  var x: Int = 0

  var y: Int = 0

  @Deprecated("legacy")
  val z: Int = 3
}
```

**After**:

```kotlin
class C {
  var x: Int = 0
  var y: Int = 0

  @Deprecated("legacy")
  val z: Int = 3
}
```

**Before — local variables**:

```kotlin
fun build() {
  val a = A()
  // configured dependency
  val b = B()
}
```

**After**:

```kotlin
fun build() {
  val a = A()

  // configured dependency
  val b = B()
}
```

**C++**

**Before**:

```cpp
class C {
  int x;

  int y;

  // legacy value
  int z;
};
```

**After**:

```cpp
class C {
  int x;
  int y;

  // legacy value
  int z;
};
```

**Before — local variables**:

```cpp
void build() {
  A a;
  // configured dependency
  B b;
}
```

**After**:

```cpp
void build() {
  A a;

  // configured dependency
  B b;
}
```

<a id="stylebrella-formatter-general-blank_lines-6"></a>

<a id="sb-f-g-bl-6"></a>

### SB-F-G-BL-6 — Blank line after a multiline statement

`Stylebrella-formatter-general-blank_lines-6`

After a statement that remains multiline in formatted output, insert one blank line before the next sibling statement. Do not add this spacing just because the input split a statement that formatting otherwise collapses to one line. Calls that exceed the width limit must remain wrapped as required by [SB-F-G-LY-2](#stylebrella-formatter-general-layout-2). Do not insert a blank line before a closing delimiter. The uninterrupted declaration group of [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4) takes precedence within that group.

**Python**

**Before**:

```python
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )
  g()
```

**After**:

```python
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )

  g()
```

**Kotlin**

**Before**:

```kotlin
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )
  g()
```

**After**:

```kotlin
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )

  g()
```

**C++**

**Before**:

```cpp
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context);
  g();
```

**After**:

```cpp
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context);

  g();
```

<a id="stylebrella-formatter-general-blank_lines-7"></a>

<a id="sb-f-g-bl-7"></a>

### SB-F-G-BL-7 — Blank lines around control flow and between branches

`Stylebrella-formatter-general-blank_lines-7`

Separate an `if` or `for` from adjacent sibling statements by one blank line, including after an unbraced single-statement body. Separate successive conditional and exception-handler branches that begin on distinct lines by one blank line. When a language-specific rule moves a branch clause off the preceding branch's line, apply this spacing at the new boundary; same-line branch syntax is governed by that language-specific newline rule. Do not put a blank line just inside a closing delimiter or between a control header and its single-line body.

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

<a id="stylebrella-formatter-general-blank_lines-11"></a>

<a id="sb-f-g-bl-11"></a>

### SB-F-G-BL-11 — Blank line before a leading line-comment block

`Stylebrella-formatter-general-blank_lines-11`

In a statement list, before a contiguous block of one or more standalone line-comment lines immediately preceding a statement or declaration, insert one blank line when a sibling statement precedes the block. Keep comment lines within the block adjacent to each other and keep the block attached to the following statement or declaration. At the start of a file or statement list, do not add a preceding blank line. This rule does not apply to comments inside headers, parameter lists, or other nested syntax. More specific rules for the boundary take precedence; use their prescribed blank-line count instead of adding this rule's count.

**Python**

**Before**:

```python
  send()
  # explain the next operation
  receive()
```

**After**:

```python
  send()

  # explain the next operation
  receive()
```

**Kotlin**

**Before**:

```kotlin
  send()
  // explain the next operation
  receive()
```

**After**:

```kotlin
  send()

  // explain the next operation
  receive()
```

**C++**

**Before**:

```cpp
  send();
  // explain the next operation
  receive();
```

**After**:

```cpp
  send();

  // explain the next operation
  receive();
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
    log()
}
```

**After**:

```kotlin
if (ready) {
  send()
  log()
}
```

**C++**

**Before**:

```cpp
if (ready) {
    send();
    log();
}
```

**After**:

```cpp
if (ready) {
  send();
  log();
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

<a id="stylebrella-formatter-general-layout-3"></a>

<a id="sb-f-g-ly-3"></a>

### SB-F-G-LY-3 — Collapse eligible single-statement control blocks

`Stylebrella-formatter-general-layout-3`

In Kotlin and C++, remove braces from an `if` or `for` block whose body consists of exactly one statement when the language permits an unbraced body and the resulting complete statement fits within 100 columns. Apply this recursively to nested `for` blocks. Preserve braces when required for syntax or scope, when there are comments or multiple statements in the body, or when removing them could change control-flow binding. Python is not applicable: its suites are defined by indentation, not braces.

**Kotlin**

**Before**:

```kotlin
if (ready) {
  for (item in items) {
    send(item)
  }
}
```

**After**:

```kotlin
if (ready) for (item in items) send(item)
```

**C++**

**Before**:

```cpp
if (ready) {
  for (const auto& item : items) {
    send(item);
  }
}
```

**After**:

```cpp
if (ready) for (const auto& item : items) send(item);
```

**Python:** Not applicable; Python does not use braces to delimit control-flow suites.

## `separators`

Rules in this section use the `separators` token in canonical IDs and `SP` in their short aliases.

<a id="stylebrella-formatter-general-separators-1"></a>

<a id="sb-f-g-sp-1"></a>

### SB-F-G-SP-1 — Separator syntax and length

`Stylebrella-formatter-general-separators-1`

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

<a id="stylebrella-formatter-general-separators-2"></a>

<a id="sb-f-g-sp-2"></a>

### SB-F-G-SP-2 — Separators between functions and before the first method

`Stylebrella-formatter-general-separators-2`

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

<a id="stylebrella-formatter-general-separators-3"></a>

<a id="sb-f-g-sp-3"></a>

### SB-F-G-SP-3 — Separator after imports and includes

`Stylebrella-formatter-general-separators-3`

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
