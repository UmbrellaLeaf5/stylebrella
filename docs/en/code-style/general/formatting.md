# Shared formatting

[Русская версия](../../../ru/code-style/general/formatting.md)

Normative preview contract for `format`, `check`, and `format --check` (both checks are read-only and cover formatting only). These rules describe intended behavior, not implementation status. Each language uses one canonical formatter, without chaining alternative formatters. After Stylebrella formatting, a second Stylebrella pass and a pass of the language formatter with the same Stylebrella preset must each leave the bytes unchanged. An incompatibility blocks support for the affected rule until its contract or the formatter selection is revised.

[Python](../python/formatting.md) · [Kotlin](../kotlin/formatting.md) · [C++](../cpp/formatting.md)

[Shared linting](linting.md) · [ID registry](../README.md#id-registry)

Examples isolate the rule being illustrated.

## `source`

<a id="stylebrella-formatter-general-source-1"></a>

<a id="sb-f-g-s-1"></a>

### SB-F-G-S-1 — UTF-8 source files with LF line endings

`Stylebrella-formatter-general-source-1`

Write source files as UTF-8 without a BOM and normalize line endings to LF. Remove trailing spaces and tabs outside literals, and end every nonempty file with exactly one LF. Do not change literal values while normalizing. A language-specific formatter-off range is exempt where its language rule explicitly says so. In these examples, `<BOM>`, `··`, `→`, `<CRLF>`, `<LF>`, and `<EOF>` represent a byte-order mark, two trailing spaces, a trailing tab, line endings, and immediate end of file.

**Python**

**Before**:

```text
<BOM>value = "данные"··→<CRLF><EOF>
```

**After**:

```text
value = "данные"<LF><EOF>
```

**Kotlin**

**Before**:

```text
<BOM>val value = "данные"··→<CRLF><EOF>
```

**After**:

```text
val value = "данные"<LF><EOF>
```

**C++**

**Before**:

```text
<BOM>const char* value = "данные";··→<CRLF><EOF>
```

**After**:

```text
const char* value = "данные";<LF><EOF>
```

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

In a statement list, group adjacent local declarations that do not start receiver-operation groups. Group consecutive operations on the same variable even without an adjacent declaration, for example when the variable is a parameter. A receiver-operation group consists of at least two operations, or a local declaration immediately followed by one or more operations on that variable. Operations include direct method calls on the variable, assignments to its fields, reassignments to the variable itself, and direct calls whose sole argument is that variable (for example, `use(f)`). A bare call with additional arguments, such as `use(f, g)`, or a call through another receiver, such as `g.func(f, a)`, does not join the group for `f`. Separate a group from unrelated adjacent statements by one blank line, including when the next statement operates on a different variable. Within groups, omit blank lines except where [SB-F-G-BL-6](#stylebrella-formatter-general-blank_lines-6) requires one after a multiline operation. Do not reorder or group across intervening statements or explicit comment boundaries. Apply [SB-F-G-BL-5](#stylebrella-formatter-general-blank_lines-5) to comments on declarations.

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

```python
  target.reset()

  target.activate()
  use(target)
  cancel(other)
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

```python
  target.reset()
  target.activate()
  use(target)

  cancel(other)
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

```kotlin
  target.reset()

  target.activate()
  use(target)
  cancel(other)
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

```kotlin
  target.reset()
  target.activate()
  use(target)

  cancel(other)
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

```cpp
  target.reset();

  target.activate();
  use(target);
  cancel(other);
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

```cpp
  target.reset();
  target.activate();
  use(target);

  cancel(other);
```

<a id="stylebrella-formatter-general-blank_lines-5"></a>

<a id="sb-f-g-bl-5"></a>

### SB-F-G-BL-5 — Spacing before comment-attached declarations

`Stylebrella-formatter-general-blank_lines-5`

Keep consecutive class-field and local-variable declarations together unless the next declaration has a leading comment or annotation; in that case put one blank line before the attached comment or annotation. A trailing comment on the same line as its declaration does not separate the group. For local variables, the leading-comment or annotation boundary takes precedence over the grouping in [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4).

**Python**

**Before**:

```python
class C:
  _x: int

  _y: int

  # legacy value
  _z: int
```

```python
class Position:
  _y: int  # vertical coordinate

  _x: int  # horizontal coordinate
```

```python
def build() -> None:
  a = A()  # first dependency

  b = B()  # second dependency
  # configured dependency
  c = C()
```

**After**:

```python
class C:
  _x: int
  _y: int

  # legacy value
  _z: int
```

```python
class Position:
  _y: int  # vertical coordinate
  _x: int  # horizontal coordinate
```

```python
def build() -> None:
  a = A()  # first dependency
  b = B()  # second dependency

  # configured dependency
  c = C()
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

```kotlin
class Position {
  var y: Int = 0 // vertical coordinate

  var x: Int = 0 // horizontal coordinate
}
```

```kotlin
fun build() {
  val a = A() // first dependency

  val b = B() // second dependency
  // configured dependency
  val c = C()
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

```kotlin
class Position {
  var y: Int = 0 // vertical coordinate
  var x: Int = 0 // horizontal coordinate
}
```

```kotlin
fun build() {
  val a = A() // first dependency
  val b = B() // second dependency

  // configured dependency
  val c = C()
}
```

**C++**

**Before**:

```cpp
class C {
  int _x;

  int _y;

  // legacy value
  int _z;
};
```

```cpp
class Position {
  int _y;  // vertical coordinate

  int _x;  // horizontal coordinate
};
```

```cpp
void build() {
  A a;  // first dependency

  B b;  // second dependency
  // configured dependency
  C c;
}
```

**After**:

```cpp
class C {
  int _x;
  int _y;

  // legacy value
  int _z;
};
```

```cpp
class Position {
  int _y;  // vertical coordinate
  int _x;  // horizontal coordinate
};
```

```cpp
void build() {
  A a;  // first dependency
  B b;  // second dependency

  // configured dependency
  C c;
}
```

<a id="stylebrella-formatter-general-blank_lines-6"></a>

<a id="sb-f-g-bl-6"></a>

### SB-F-G-BL-6 — Blank line after a multiline statement

`Stylebrella-formatter-general-blank_lines-6`

After a statement that remains multiline in formatted output, insert one blank line before the next sibling statement. Do not add this spacing just because the input split a statement that formatting otherwise collapses to one line. Calls that exceed the width limit must remain wrapped as required by [SB-F-G-LY-2](#stylebrella-formatter-general-layout-2). Do not insert a blank line before a closing delimiter. Use [SB-F-G-BL-7](#stylebrella-formatter-general-blank_lines-7) for the control-flow boundaries it covers. The uninterrupted declaration group of [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4) takes precedence within that group.

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
    transaction_logging_context
  );
  g();
```

**After**:

```cpp
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context
  );

  g();
```

<a id="stylebrella-formatter-general-blank_lines-7"></a>

<a id="sb-f-g-bl-7"></a>

### SB-F-G-BL-7 — Blank lines around control flow and between branches

`Stylebrella-formatter-general-blank_lines-7`

Separate `if`, `for`, `try`, and selection statements (Python `match`, Kotlin `when`, C++ `switch`) from adjacent sibling statements by one blank line, including after an unbraced single-statement body. Use one blank line between successive `if` clauses and between successive `try` clauses (`except`/`catch`, `else` and `finally` where supported) that start on their own lines after language-specific layout; do not split a same-line clause solely to add a blank line. Start each selection branch (`case`, `when` entry, or `default`) on its own line and separate successive branches by one blank line. Keep consecutive C++ labels that share a body together without blank lines. Do not add a blank line before the first or after the last item of a body, or between a control header and its body.

**Python**

**Before**:

```python
  if x: f()
  g()
```

```python
  log()
  try:
    load()
  except OSError:
    recover()
  else:
    validate()
  finally:
    release()
  match state:
    case State.READY:
      start()
    case _:
      wait()
  flush()
```

**After**:

```python
  if x: f()

  g()
```

```python
  log()

  try:
    load()

  except OSError:
    recover()

  else:
    validate()

  finally:
    release()

  match state:
    case State.READY:
      start()

    case _:
      wait()

  flush()
```

**Kotlin**

**Before**:

```kotlin
  if (x) f()
  g()
```

```kotlin
  log()
  try {
    load()
  }
  catch (error: IOException) {
    recover()
  }
  finally {
    release()
  }
  when (state) {
    State.READY -> start()
    else -> waitForWork()
  }
  flush()
```

**After**:

```kotlin
  if (x) f()

  g()
```

```kotlin
  log()

  try {
    load()
  }

  catch (error: IOException) {
    recover()
  }

  finally {
    release()
  }

  when (state) {
    State.READY -> start()

    else -> waitForWork()
  }

  flush()
```

**C++**

**Before**:

```cpp
  if (x) f();
  g();
```

```cpp
  log();
  try {
    load();
  }
  catch (const LoadError& error) {
    recover();
  }
  switch (state) {
    case State::READY:
    case State::RETRY:
      start();
      break;
    default:
      wait();
      break;
  }
  flush();
```

**After**:

```cpp
  if (x) f();

  g();
```

```cpp
  log();

  try {
    load();
  }

  catch (const LoadError& error) {
    recover();
  }

  switch (state) {
    case State::READY:
    case State::RETRY:
      start();
      break;

    default:
      wait();
      break;
  }

  flush();
```

<a id="stylebrella-formatter-general-blank_lines-8"></a>

<a id="sb-f-g-bl-8"></a>

### SB-F-G-BL-8 — Blank line before return or yield

`Stylebrella-formatter-general-blank_lines-8`

Insert exactly one blank line before a `return` or a standalone yield following another sibling statement. This includes Python `yield` and `yield from`, and C++ `co_return` and standalone `co_yield`. In Kotlin, apply the same rule before the `return` keyword regardless of the returned expression, including `return try`, `return when`, and `return if`. Do not add a blank line before the first statement in a block, inside an inline control body, or before a yield nested inside another expression.

**Python**

**Before**:

```python
  send()
  return result
```

```python
  record(item)
  yield item
  yield from remaining_items
```

**After**:

```python
  send()

  return result
```

```python
  record(item)

  yield item

  yield from remaining_items
```

**Kotlin**

**Before**:

```kotlin
  send()
  return result
```

```kotlin
  recordAttempt()
  return try {
    load()
  }

  catch (error: IOException) {
    fallback()
  }

  finally {
    release()
  }
```

```kotlin
  recordState(state)
  return when (state) {
    State.READY -> current()

    else -> fallback()
  }
```

```kotlin
  recordAttempt()
  return if (ready) {
    recordSuccess()
    current()
  }

  else {
    recordFailure()
    fallback()
  }
```

**After**:

```kotlin
  send()

  return result
```

```kotlin
  recordAttempt()

  return try {
    load()
  }

  catch (error: IOException) {
    fallback()
  }

  finally {
    release()
  }
```

```kotlin
  recordState(state)

  return when (state) {
    State.READY -> current()

    else -> fallback()
  }
```

```kotlin
  recordAttempt()

  return if (ready) {
    recordSuccess()
    current()
  }

  else {
    recordFailure()
    fallback()
  }
```

**C++**

**Before**:

```cpp
  send();
  return result;
```

```cpp
  record(item);
  co_yield item;
  co_return;
```

**After**:

```cpp
  send();

  return result;
```

```cpp
  record(item);

  co_yield item;

  co_return;
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

When a MARK and its separator have only blank lines between them, remove all those lines. This rule does not insert a missing separator; see [SB-F-G-M-1](#sb-f-g-m-1).

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

### SB-F-G-BL-11 — Blank line before leading comments

`Stylebrella-formatter-general-blank_lines-11`

In a statement list, insert one blank line before a contiguous group of standalone comments that immediately precedes a statement or declaration, when a sibling statement precedes the group. This applies to individual line comments, consecutive line comments, and block comments such as multiline `/* ... */` in Kotlin and C++. Preserve spacing inside block comments and keep the group attached to the following statement or declaration. At the start of a file or statement list, do not add a preceding blank line. This rule does not apply to trailing comments or to comments inside headers, parameter lists, or other nested syntax. More specific rules for the boundary take precedence; use their prescribed blank-line count instead of adding this rule's count.

**Python**

**Before**:

```python
  send()
  # explain the next operation
  receive()
```

```python
  send()
  # read the response before
  # sending the next request
  receive()
```

**After**:

```python
  send()

  # explain the next operation
  receive()
```

```python
  send()

  # read the response before
  # sending the next request
  receive()
```

**Kotlin**

**Before**:

```kotlin
  send()
  // explain the next operation
  receive()
```

```kotlin
  send()
  /*
   * Read the response before
   * sending the next request.
   */
  receive()
```

**After**:

```kotlin
  send()

  // explain the next operation
  receive()
```

```kotlin
  send()

  /*
   * Read the response before
   * sending the next request.
   */
  receive()
```

**C++**

**Before**:

```cpp
  send();
  // explain the next operation
  receive();
```

```cpp
  send();
  /*
   * Read the response before
   * sending the next request.
   */
  receive();
```

**After**:

```cpp
  send();

  // explain the next operation
  receive();
```

```cpp
  send();

  /*
   * Read the response before
   * sending the next request.
   */
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

Use a 100-column wrapping limit for calls and signatures. Collapse a call with exactly one argument to one line whenever the complete formatted call fits within the limit, regardless of its input line breaks; remove a trailing comma if the language permits one. Otherwise, if the single-line form exceeds the limit, use hanging indentation with one item per continuation line at two extra spaces, subject to the language closing-delimiter rules. In C++, put the closing `);` of a wrapped call on its own line, aligned with the start of the call; do not add a trailing comma. Do not split an indivisible token even if the line stays over the limit.

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

**Before — one argument**:

```python
f(
  x,
)
```

**After**:

```python
f(x)
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

**Before — one argument**:

```kotlin
f(
  x,
)
```

**After**:

```kotlin
f(x)
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
  transaction_logging_context
);
```

**Before — one argument**:

```cpp
f(
  x
);
```

**After**:
```cpp
f(x);
```

<a id="stylebrella-formatter-general-layout-3"></a>

<a id="sb-f-g-ly-3"></a>

### SB-F-G-LY-3 — Collapse eligible single-statement control blocks

`Stylebrella-formatter-general-layout-3`

In Kotlin and C++, remove braces from an `if` or `for` block whose body consists of exactly one statement if the language permits an unbraced body and the resulting layout fits within 100 columns per line. Keep a single non-control body on the header line if it fits and is not part of an unbraced chain of nested controls. In such a chain, put each header on its own line and indent its body two spaces on the next line, including the innermost body. Preserve braces when required for syntax or scope, when there are comments or multiple statements in the body, or when removing them could change control-flow binding. Python is not applicable: its suites are defined by indentation, not braces.

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
if (ready)
  for (item in items)
    send(item)
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
if (ready)
  for (const auto& item : items)
    send(item);
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

<a id="stylebrella-formatter-general-mark-1"></a>

<a id="sb-f-g-m-1"></a>

### SB-F-G-M-1 — Required separator after MARK

`Stylebrella-formatter-general-mark-1`

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

<a id="stylebrella-formatter-general-mark-2"></a>

<a id="sb-f-g-m-2"></a>

### SB-F-G-M-2 — Private Helpers section

`Stylebrella-formatter-general-mark-2`

For a concrete class with at least one private method, including a class with no public methods, require exactly one `MARK: Private Helpers` block before its first private method. `format` inserts or moves the block and its separator to that position and removes duplicate structural blocks and obsolete `(no private methods)` comments. `check` reports the same needed edits without writing. Preserve access labels, member order, and all descriptive MARKs, even when lint reports adjacent MARKs. Do not apply to interfaces or confirmed DTO-only types.

If a public method follows a private one, skip all Private Helpers and placeholder edits in that class and continue formatting. [SB-L-G-M-7](linting.md#sb-l-g-m-7) reports the order violation in `lint`. Determine visibility before editing, using the language's declaration rules; in Python, preserve the classification of methods identified by an existing private section. For a class with public methods and no private methods, apply [SB-F-G-CM-4](#sb-f-g-cm-4) instead.

In each example below, the same marker edit applies if the class contains only the private `_copy` method: add the header before the existing separator. If `_copy` instead precedes `read`, leave the class's structural markers as they are. If there are no private methods, use the placeholder examples in [SB-F-G-CM-4](#sb-f-g-cm-4).

**Python**

**Before**:

```python
class Reader:
  # MARK: Read input
  # --------------------------------------------------

  def read(self, value: str) -> str:
    return self._copy(value)

  # --------------------------------------------------

  def write(self, value: str) -> int:
    return len(value)

  # --------------------------------------------------

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

  # --------------------------------------------------

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

  // --------------------------------------------------

  fun write(value: String): Int = value.length

  // --------------------------------------------------

  private fun _copy(value: String): String = value
}
```

**After**:

```kotlin
class Reader {
  // MARK: Read input
  // --------------------------------------------------

  fun read(value: String): String = _copy(value)

  // --------------------------------------------------

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

  // --------------------------------------------------

  int write(int value) const { return value; }

 private:

  // --------------------------------------------------

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

  // --------------------------------------------------

  int write(int value) const { return value; }

 private:
  // MARK: Private Helpers
  // --------------------------------------------------

  int _copy(int value) const { return value; }
};
```

<a id="stylebrella-formatter-general-mark-3"></a>

<a id="sb-f-g-m-3"></a>

### SB-F-G-M-3 — One space after `MARK:`

`Stylebrella-formatter-general-mark-3`

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

<a id="stylebrella-formatter-general-mark-4"></a>

<a id="sb-f-g-m-4"></a>

### SB-F-G-M-4 — Remove the singleton group suffix

`Stylebrella-formatter-general-mark-4`

When a MARK title ends with the exact suffix ` [group:1]`, remove the suffix and its preceding space. Leave all other title text and MARK structure unchanged. Group counts of two or more are preserved.

**Python**

**Before**:

```python
# MARK: Load adapters [group:1]
# --------------------------------------------------
```

**After**:

```python
# MARK: Load adapters
# --------------------------------------------------
```

**Kotlin**

**Before**:

```kotlin
// MARK: Load adapters [group:1]
// --------------------------------------------------
```

**After**:

```kotlin
// MARK: Load adapters
// --------------------------------------------------
```

**C++**

**Before**:

```cpp
// MARK: Load adapters [group:1]
// --------------------------------------------------
```

**After**:

```cpp
// MARK: Load adapters
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
#load the saved settings
```

**After**:

```python
# load the saved settings
```

**Kotlin**

**Before**:

```kotlin
//load the saved settings
```

**After**:

```kotlin
// load the saved settings
```

**C++**

**Before**:

```cpp
//load the saved settings
```

**After**:

```cpp
// load the saved settings
```

<a id="stylebrella-formatter-general-comments-2"></a>

<a id="sb-f-g-cm-2"></a>

### SB-F-G-CM-2 — Lowercase opening of single-line comments

`Stylebrella-formatter-general-comments-2`

If the first non-whitespace character of an ordinary standalone or trailing single-line comment is an uppercase letter, lowercase it (including non-ASCII letters). This applies to `#`, `//`, and ordinary `/* ... */` comments confined to one line. Preserve the rest of the text; do not skip punctuation, digits, or code delimiters to find a letter. Preserve structured comments such as MARKs, separators, shebangs, documentation, and tool directives.

**Python**

**Before**:

```python
# Load the saved settings
```

**After**:

```python
# load the saved settings
```

**Kotlin**

**Before**:

```kotlin
  val retries: Int, // Максимум повторов
```

**After**:

```kotlin
  val retries: Int, // максимум повторов
```

**C++**

**Before**:

```cpp
  /* Skip the cached value. */
  refresh();
```

**After**:

```cpp
  /* skip the cached value. */
  refresh();
```

<a id="stylebrella-formatter-general-comments-3"></a>

<a id="sb-f-g-cm-3"></a>

### SB-F-G-CM-3 — Wrapping trailing comments on variable declarations

`Stylebrella-formatter-general-comments-3`

For an ordinary prose trailing `#` or `//` comment on a variable or field declaration, wrap comment text at word boundaries to keep each line within 100 columns where possible. Preserve every word and punctuation mark; normalize inter-word whitespace to one ASCII space. If the comment continues, start each continuation marker at the same column as the original marker and put one space after it. Do not split an indivisible word. Leave standalone comments, comments attached to other statements, documentation, MARKs, separators, tool directives, and comments containing aligned/code-formatted text unchanged. Apply the language-specific prefix and capitalization rules as well.

**Python**

**Before**:

```python
value = 10  # key feature of this configuration and its required behavior for every request, including retries and recovery
```

**After**:

```python
value = 10  # key feature of this configuration and its required behavior for every request,
            # including retries and recovery
```

**Kotlin**

**Before**:

```kotlin
val value = 10  // key feature of this configuration and its required behavior for every request, including retries and recovery
```

**After**:

```kotlin
val value = 10  // key feature of this configuration and its required behavior for every request,
                // including retries and recovery
```

**C++**

**Before**:

```cpp
int value; // key feature of this configuration and its required behavior for every request, including retries and recovery
```

**After**:

```cpp
int value; // key feature of this configuration and its required behavior for every request,
           // including retries and recovery
```

<a id="stylebrella-formatter-general-comments-4"></a>

<a id="sb-f-g-cm-4"></a>

### SB-F-G-CM-4 — Placeholder for classes without private methods

`Stylebrella-formatter-general-comments-4`

For a concrete class with public but no private methods, require exactly one `(no private methods)` standalone comment after the last public method, separated by one blank line. `format` replaces stale `Private Helpers` blocks, inserts or moves the placeholder, and removes duplicates; `check` reports the same needed edits without writing. A class with neither public nor private methods does not need a placeholder. If private methods exist, follow [SB-F-G-M-2](#sb-f-g-m-2), including its order guard. Interfaces and confirmed DTO-only types are exempt.

**Python**

**Before**:

```python
class Counter:

  # --------------------------------------------------

  def __init__(self) -> None:
    """
    Создаёт счётчик без зарегистрированных событий.
    """

    self._count = 0

  # --------------------------------------------------

  def increment(self) -> None:
    """
    Учитывает одно новое событие.
    """

    self._count += 1

  # --------------------------------------------------

  def value(self) -> int:
    """
    Возвращает число зарегистрированных событий.

    Returns:
      Число событий с момента создания счётчика.
    """

    return self._count
```

**After**:

```python
class Counter:

  # --------------------------------------------------

  def __init__(self) -> None:
    """
    Создаёт счётчик без зарегистрированных событий.
    """

    self._count = 0

  # --------------------------------------------------

  def increment(self) -> None:
    """
    Учитывает одно новое событие.
    """

    self._count += 1

  # --------------------------------------------------

  def value(self) -> int:
    """
    Возвращает число зарегистрированных событий.

    Returns:
      Число событий с момента создания счётчика.
    """

    return self._count

  # (no private methods)
```

**Kotlin**

**Before**:

```kotlin
class Counter {
  private var _count = 0

  // --------------------------------------------------

  /**
   * Учитывает одно новое событие.
   */
  fun increment(): Unit {
    _count += 1
  }

  // --------------------------------------------------

  /**
   * Возвращает число зарегистрированных событий.
   *
   * @return число событий с момента создания счётчика
   */
  fun value(): Int = _count

  // MARK: Private Helpers
  // --------------------------------------------------
}
```

**After**:

```kotlin
class Counter {
  private var _count = 0

  // --------------------------------------------------

  /**
   * Учитывает одно новое событие.
   */
  fun increment(): Unit {
    _count += 1
  }

  // --------------------------------------------------

  /**
   * Возвращает число зарегистрированных событий.
   *
   * @return число событий с момента создания счётчика
   */
  fun value(): Int = _count

  // (no private methods)
}
```

**C++**

**Before**:

```cpp
/**
 * @brief Считает зарегистрированные события.
 */
class Counter {
  int _count = 0;  // число зарегистрированных событий

 public:

  // --------------------------------------------------

  /**
   * @brief Учитывает одно новое событие.
   */
  void increment() { ++_count; }

  // --------------------------------------------------

  /**
   * @brief Возвращает число зарегистрированных событий.
   * @return Число событий с момента создания счётчика.
   */
  int value() const { return _count; }

  // (no private methods)
  // (no private methods)
};
```

**After**:

```cpp
/**
 * @brief Считает зарегистрированные события.
 */
class Counter {
  int _count = 0;  // число зарегистрированных событий

 public:

  // --------------------------------------------------

  /**
   * @brief Учитывает одно новое событие.
   */
  void increment() { ++_count; }

  // --------------------------------------------------

  /**
   * @brief Возвращает число зарегистрированных событий.
   * @return Число событий с момента создания счётчика.
   */
  int value() const { return _count; }

  // (no private methods)
};
```

## Recommendations (no diagnostic)

Use explicit breaks or group comments when the intended layout cannot be inferred. Prefer short argument names only when their meaning remains clear. Never break indivisible tokens or change literal values to meet the width.
