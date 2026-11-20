# Formatting Python

[Русская версия](../../../ru/code-style/python/formatting.md)

Scope: `.py`. Notebooks and standalone configuration files are outside this contract. For class visibility see [Python linting](linting.md).

[Shared rules](../general/formatting.md)

Examples isolate the rule being illustrated.

## `source`

<a id="stylebrella-formatter-python-source-1"></a>

<a id="sb-f-p-s-1"></a>

### SB-F-P-S-1 — LF line endings

`Stylebrella-formatter-python-source-1`

Normalize file line endings to LF without changing literal contents.

**Before** (CRLF):

```python
x = 1
```

**After** (LF):

```python
x = 1
```

<a id="stylebrella-formatter-python-source-2"></a>

<a id="sb-f-p-s-2"></a>

### SB-F-P-S-2 — Double quotes for strings

`Stylebrella-formatter-python-source-2`

Use double quotes for string literals when doing so preserves the value and does not introduce additional escapes.

**Before**:

```python
x = 'hello'
```

**After**:

```python
x = "hello"
```

## `imports`

<a id="stylebrella-formatter-python-imports-1"></a>

<a id="sb-f-p-i-1"></a>

### SB-F-P-I-1 — Grouping and sorting imports

`Stylebrella-formatter-python-imports-1`

In the leading import block order groups as future imports, standard library, third-party, and local; use one blank line between present groups. Order imports within a group by module and imported name, and retain attached comments with their imports. Do not skip sorting because of import-time effects. Do not relocate imports into the block.

**Before**:

```python
import requests
import sys
```

**After**:

```python
import sys

import requests
```

## `blank_lines`

<a id="stylebrella-formatter-python-blank_lines-1"></a>

<a id="sb-f-p-bl-1"></a>

### SB-F-P-BL-1 — Blank line after a docstring

`Stylebrella-formatter-python-blank_lines-1`

Put one blank line between a function or class docstring and the next body statement; add none when the docstring is the only statement.

**Before**:

```python
def read_data() -> None:
  """
  Читает данные.
  """
  read()
```

**After**:

```python
def read_data() -> None:
  """
  Читает данные.
  """

  read()
```

<a id="stylebrella-formatter-python-blank_lines-2"></a>

<a id="sb-f-p-bl-2"></a>

### SB-F-P-BL-2 — Blank lines before statements and clause headers

`Stylebrella-formatter-python-blank_lines-2`

Put one blank line before sibling `while`, `with`, `raise`, `assert`, and `continue` statements, and before loop `else` clause headers following a suite. For spacing around `if`, `for`, `try`, and `match` and between their branches, apply [SB-F-G-BL-7](../general/formatting.md#stylebrella-formatter-general-blank_lines-7). Do not split a same-line suite to insert a blank line.

**Before**:

```python
  log()
  assert ready
```

**After**:

```python
  log()

  assert ready
```

<a id="stylebrella-formatter-python-blank_lines-3"></a>

<a id="sb-f-p-bl-3"></a>

### SB-F-P-BL-3 — Spacing around constructor category comments

`Stylebrella-formatter-python-blank_lines-3`

Keep constructor parameter category comments in their parameter list and remove blank lines directly before and after them.

**Before**:

```python
def __init__(
  self,

  # services:
  service: Service,
): ...
```

**After**:

```python
def __init__(
  self,
  # services:
  service: Service,
): ...
```

## `mark`

<a id="stylebrella-formatter-python-mark-1"></a>

<a id="sb-f-p-m-1"></a>

### SB-F-P-M-1 — Spacing in inline comments

`Stylebrella-formatter-python-mark-1`

Keep an inline comment on the same line as its code. Apply [SB-F-G-CM-1](../general/formatting.md#sb-f-g-cm-1) to spacing after `#`. Preserve interpreter directives, encoding declarations, and tool-marker text.

**Before**:

```python
x = load()  #note
```

**After**:

```python
x = load()  # note
```

## Recommendations (no diagnostic)

Keep import-time effects independent of import order. Use a separate utility module rather than working around import placement.
