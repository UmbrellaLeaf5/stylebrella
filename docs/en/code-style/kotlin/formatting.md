# Formatting Kotlin

[Русская версия](../../../ru/code-style/kotlin/formatting.md)

Scope: `.kt` and `.kts`, including Gradle scripts. The preset requires no IDE-specific settings to obtain the specified output. For class visibility see [Kotlin linting](linting.md).

[Shared rules](../general/formatting.md)

Examples isolate the rule being illustrated.

## `source`

<a id="stylebrella-formatter-kotlin-source-1"></a>

<a id="sb-f-k-s-1"></a>

### SB-F-K-S-1 — Encoding, line endings, and trailing whitespace

`Stylebrella-formatter-kotlin-source-1`

Emit UTF-8 with LF, remove trailing spaces and tabs outside literals, and end a nonempty file with exactly one LF. In the example, `·` represents a trailing ASCII space, `<CRLF>` and `<LF>` the line endings, and `<EOF>` the immediate end of file.

**Before**:

```text
val label = "данные"··<CRLF><EOF>
```

**After**:

```text
val label = "данные"<LF><EOF>
```

<a id="stylebrella-formatter-kotlin-source-2"></a>

<a id="sb-f-k-s-2"></a>

### SB-F-K-S-2 — Formatter-disabled regions

`Stylebrella-formatter-kotlin-source-2`

Do not change bytes in a range from a standalone `// @formatter:off` to the next standalone `// @formatter:on`, inclusive, or to EOF if unmatched. This also exempts the range from file-level whitespace normalization.

**Before**:

```kotlin
// @formatter:off
val n=1
// @formatter:on
```

**After**:

```kotlin
// @formatter:off
val n=1
// @formatter:on
```

## `layout`

<a id="stylebrella-formatter-kotlin-layout-1"></a>

<a id="sb-f-k-ly-1"></a>

### SB-F-K-LY-1 — Wrapped arguments and parameters

`Stylebrella-formatter-kotlin-layout-1`

A wrapped call or signature has one argument or parameter per line and a closing parenthesis on a separate line aligned with the start of the construct. Use a trailing comma where Kotlin allows one.

**Before**:

```kotlin
call(
  x, y)
```

**After**:

```kotlin
call(
  x,
  y,
)
```

<a id="stylebrella-formatter-kotlin-layout-2"></a>

<a id="sb-f-k-ly-2"></a>

### SB-F-K-LY-2 — Spaces after commas and around operators

`Stylebrella-formatter-kotlin-layout-2`

Add one space after a comma when another token follows on the same line, and on both sides of binary arithmetic, equality, and comparison operators. Do not treat unary operators or generic delimiters as binary operators.

**Before**:

```kotlin
f(x,y+1==z)
```

**After**:

```kotlin
f(x, y + 1 == z)
```

## `imports`

<a id="stylebrella-formatter-kotlin-imports-1"></a>

<a id="sb-f-k-i-1"></a>

### SB-F-K-I-1 — Import group order

`Stylebrella-formatter-kotlin-imports-1`

Order leading imports lexicographically within groups in this order: ordinary, `java`, `javax`, `kotlin`, then aliases. Move attached comments with their imports. Do not relocate late imports.

**Before**:

```kotlin
import kotlin.math.PI
import app.run
```

**After**:

```kotlin
import app.run
import kotlin.math.PI
```

<a id="stylebrella-formatter-kotlin-imports-2"></a>

<a id="sb-f-k-i-2"></a>

### SB-F-K-I-2 — Star import thresholds

`Stylebrella-formatter-kotlin-imports-2`

Replace imports of at least five distinct nonaliased names from one package, or three distinct nonaliased members from one type, with one star import when that replacement preserves the identity of every resolved reference and creates no new ambiguity. Do not count aliases toward either threshold.

**Before**:

```kotlin
import app.Tools.a
import app.Tools.b
import app.Tools.c
```

**After**:

```kotlin
import app.Tools.*
```

## `expressions`

<a id="stylebrella-formatter-kotlin-expressions-1"></a>

<a id="sb-f-k-e-1"></a>

### SB-F-K-E-1 — Expression bodies

`Stylebrella-formatter-kotlin-expressions-1`

Rewrite a block consisting only of an unlabelled `return expression` into `= expression` if there is no contract and the rewrite preserves behavior. Omit the explicit return type only if the inferred type is the same and the public and override signatures and `Unit` semantics are unchanged. Leave early, labelled, and unresolved returns unchanged.

**Before**:

```kotlin
fun count(): Int { return items.size }
```

**After**:

```kotlin
fun count() = items.size
```

## `blank_lines`

<a id="stylebrella-formatter-kotlin-blank_lines-1"></a>

<a id="sb-f-k-bl-1"></a>

### SB-F-K-BL-1 — Blank lines between when branches

`Stylebrella-formatter-kotlin-blank_lines-1`

Apply [SB-F-G-BL-7](../general/formatting.md#stylebrella-formatter-general-blank_lines-7) between all adjacent `when` branches, even if both have expression bodies.

**Before**:

```kotlin
  A -> { f() }
  B -> g()
```

**After**:

```kotlin
  A -> { f() }

  B -> g()
```

<a id="stylebrella-formatter-kotlin-blank_lines-2"></a>

<a id="sb-f-k-bl-2"></a>

### SB-F-K-BL-2 — Spacing for constructor categories and annotated declarations

`Stylebrella-formatter-kotlin-blank_lines-2`

Use one blank line between constructor parameter categories, before the next category comment, and before a declaration with an attached leading comment or annotation when preceded by a sibling. The attached comment or annotation stays with its declaration.

**Before**:

```kotlin
val a = 1
@JvmField
val b = 2
```

**After**:

```kotlin
val a = 1

@JvmField
val b = 2
```

<a id="stylebrella-formatter-kotlin-blank_lines-3"></a>

<a id="sb-f-k-bl-3"></a>

### SB-F-K-BL-3 — Line breaks before else, catch, and finally

`Stylebrella-formatter-kotlin-blank_lines-3`

Start `else if`, `else`, `catch`, and `finally` on a new line, including inside `return if` and `return try`. Apply [SB-F-G-BL-7](../general/formatting.md#stylebrella-formatter-general-blank_lines-7) at the branch boundary and [SB-F-G-BL-8](../general/formatting.md#stylebrella-formatter-general-blank_lines-8) before the whole `return` statement. Do not introduce braces solely for formatting.

**Before**:

```kotlin
if (x) {
  f()
  g()
} else {
  h()
  i()
}
```

```kotlin
  return try {
    load()
  } catch (error: IOException) {
    fallback()
  } finally {
    release()
  }
```

```kotlin
  return if (ready) {
    recordSuccess()
    current()
  } else {
    recordFailure()
    fallback()
  }
```

**After**:

```kotlin
if (x) {
  f()
  g()
}

else {
  h()
  i()
}
```

```kotlin
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
  return if (ready) {
    recordSuccess()
    current()
  }

  else {
    recordFailure()
    fallback()
  }
```

## Recommendations (no diagnostic)

Use a block when an expression body would hide control flow. Keep import-time effects independent of import order.
