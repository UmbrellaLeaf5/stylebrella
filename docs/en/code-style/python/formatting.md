# Python formatting

[Русская версия](../../../ru/code-style/python/formatting.md)

This page adds Python-specific rules to [shared formatting](../formatting.md). It covers `.py`; notebooks and standalone config files are deferred. See [Python linting](linting.md). Examples are acceptance fixtures or excerpts.

## Layout and imports

Use LF and double quotes where changing quotes preserves the literal. Ruff is the Python base formatter and import sorter. Do not chain Black or isort. Order imports as standard library, third-party, then local, with a blank line between groups. Preserve comment attachment and imports whose order cannot safely be changed. Import-order diagnostics belong to the formatting check, even if Ruff implements sorting through an `I` rule.

Before:

```python
from app import run
import os
def main():
    run('example', os.environ)
```

After:

```python
import os

from app import run


def main():
  run("example", os.environ)
```

## Blank lines

- Use a blank line after a function/class docstring before the first statement. A docstring-only body needs no trailing blank line inside it.
- In addition to the shared control-flow rules, add a blank line at a peer boundary before `while`, `try`, `finally`, `with`, `raise`, `assert`, and `continue`. Separate `elif`, `else`, `except`, and `finally` headers from the preceding suite. Do not split an inline suite or put a blank line before its only statement. Deeply nested one-line control-flow idioms can stay compact.
- A series of standalone field assignments after an object declaration starts after a blank line; initialization followed by a receiver call still uses shared grouping.
- Keep constructor category comments in the parameter list without inserting blank lines there. Inline comments use `# `; keep them attached to their code.

Before:

```python
def configure(config: Config) -> None:
  announce()
  filter_ = Filter()
  filter_.init(config)
  if config.enabled:
    filter_.start()
  return
```

After:

```python
def configure(config: Config) -> None:
  announce()

  filter_ = Filter()
  filter_.init(config)

  if config.enabled:
    filter_.start()

  return
```

## Separators and private helpers

Use `#` for the shared separators and structural comments. [Python linting](linting.md#marks-method-order-and-documentation) defines which methods count as private.

Before:

```python
class Reader:
  def read(self) -> str:
    return self._read()
  def _read(self) -> str:
    return self.source.read()
```

After:

```python
class Reader:

  def read(self) -> str:
    return self._read()

  # MARK: Private Helpers
  # --------------------------------------------------

  def _read(self) -> str:
    return self.source.read()
```

Formatting must stabilize on a second pass. If Ruff disagrees with a layout fixture, record the gap and resolve it before claiming support; do not ship two passes that undo one another.
