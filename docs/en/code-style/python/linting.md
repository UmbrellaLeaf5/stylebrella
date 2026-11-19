# Linting Python

[Русская версия](../../../ru/code-style/python/linting.md)

Scope: `.py`; notebooks are out of scope. Only checks expressly defined by this contract are enabled as Stylebrella rules. One remaining finding fails lint; the formatter checks import order. For MARK, names starting with exactly one underscore are private; double-underscore special methods such as `__init__` are not private helpers.

[Shared rules](../linting.md)

Examples isolate the rule being illustrated.

## `naming`

<a id="stylebrella-linter-python-naming-1"></a>

<a id="sb-l-p-n-1"></a>

### SB-L-P-N-1 — Naming conventions for Python declarations

`Stylebrella-linter-python-naming-1`

Require snake_case for functions, methods, variables, parameters and fields; PascalCase for classes, dataclasses and exception types; and SCREAMING_SNAKE_CASE for constants. Use `T` or a PascalCase name starting with `T` for a resolved type variable. Exempt Python special names.

**Allowed**:

```python
def load_config() -> Config:
  return read_config()
```

**Violation**:

```python
def loadConfig() -> Config:
  return read_config()
```

<a id="stylebrella-linter-python-naming-2"></a>

<a id="sb-l-p-n-2"></a>

### SB-L-P-N-2 — Leading underscore for internal symbols

`Stylebrella-linter-python-naming-2`

Require one leading underscore for symbols declared internal by project metadata or an explicit visibility convention. Lack of use or omission from an unrelated export list is not evidence.

**Allowed** (internal type):

```python
class _InternalType:
  pass
```

**Violation** (internal type):

```python
class InternalType:
  pass
```

## `json`

<a id="stylebrella-linter-python-json-1"></a>

<a id="sb-l-p-j-1"></a>

### SB-L-P-J-1 — One naming convention for JSON keys

`Stylebrella-linter-python-json-1`

Use one configured convention, snake_case or camelCase, for statically resolved JSON keys across the project. Do not infer a convention from an individual key.

**Allowed** (snake_case project):

```python
"user_id"
```

**Violation** (snake_case project):

```python
"User-ID"
```

## `types`

<a id="stylebrella-linter-python-types-1"></a>

<a id="sb-l-p-t-1"></a>

### SB-L-P-T-1 — Parameter and return annotations

`Stylebrella-linter-python-types-1`

Require annotations on every function parameter and result; exempt only the actual first instance/class receiver `self` or `cls`. A no-value result is annotated `-> None`. Report missing annotations on the parameter or function name.

**Allowed**:

```python
def clear(self, cache: Cache) -> None:
  cache.clear()
```

**Violation**:

```python
def clear(self, cache) -> None:
  cache.clear()
```

<a id="stylebrella-linter-python-types-2"></a>

<a id="sb-l-p-t-2"></a>

### SB-L-P-T-2 — Union with None instead of Optional

`Stylebrella-linter-python-types-2`

For Python targets at least 3.10, report annotations resolved to `typing.Optional[T]`; use `T | None`.

**Allowed**:

```python
def find() -> User | None:
  return lookup_user()
```

**Violation**:

```python
def find() -> Optional[User]:
  return lookup_user()
```

<a id="stylebrella-linter-python-types-3"></a>

<a id="sb-l-p-t-3"></a>

### SB-L-P-T-3 — No explicit typing.Any

`Stylebrella-linter-python-types-3`

Report explicit annotation references resolved to `typing.Any`. An unrelated symbol named `Any` does not match.

**Allowed**:

```python
def load(data: object) -> Config:
  return parse_config(data)
```

**Violation**:

```python
def load(data: Any) -> Config:
  return parse_config(data)
```

<a id="stylebrella-linter-python-types-4"></a>

<a id="sb-l-p-t-4"></a>

### SB-L-P-T-4 — Path annotations for filesystem paths

`Stylebrella-linter-python-types-4`

For an argument or result identified as a filesystem path by explicit metadata or resolved path APIs, report an annotation of `str`; the expected annotation is `pathlib.Path`. Do not infer a path from its name alone.

**Allowed**:

```python
def load(path: Path) -> Config:
  return parse_config(path)
```

**Violation** (confirmed path):

```python
def load(path: str) -> Config:
  return parse_config(path)
```

<a id="stylebrella-linter-python-types-5"></a>

<a id="sb-l-p-t-5"></a>

### SB-L-P-T-5 — Blocking type errors

`Stylebrella-linter-python-types-5`

Treat any type error produced by basic-mode analysis of in-scope Python as a blocking diagnostic. The reference in an unresolved context is checked only as defined by basic-mode type analysis.

**Allowed**:

```python
value: int = len(items)
```

**Violation**:

```python
value: int = "wrong"
```

## `idioms`

<a id="stylebrella-linter-python-idioms-1"></a>

<a id="sb-l-p-id-1"></a>

### SB-L-P-ID-1 — String interpolation and deferred logging

`Stylebrella-linter-python-idioms-1`

Require f-strings for resolved interpolation using string `%` formatting or `str.format`; arithmetic modulo is excluded. In a resolved logger call with interpolation, require the deferred `%` template and separate arguments instead of an f-string or a precomputed formatted string.

**Allowed**:

```python
logger.debug("Loaded %r", config)
```

**Violation**:

```python
logger.debug(f"Loaded {config!r}")
```

<a id="stylebrella-linter-python-idioms-2"></a>

<a id="sb-l-p-id-2"></a>

### SB-L-P-ID-2 — Context managers for local resources

`Stylebrella-linter-python-idioms-2`

Report a resolved `open` or `Path.open` acquisition when the acquired file is used locally, is not used as a `with` manager, and does not escape the scope or enter another resolved resource manager. Apply the same condition to recognized connection, lock and user resource APIs with context-manager support.

**Allowed**:

```python
with path.open() as stream: consume(stream)
```

**Violation**:

```python
stream = path.open(); stream.read()
```

<a id="stylebrella-linter-python-idioms-3"></a>

<a id="sb-l-p-id-3"></a>

### SB-L-P-ID-3 — Maximum comprehension nesting

`Stylebrella-linter-python-idioms-3`

Allow at most two nested comprehension generator levels. Sum the enclosing comprehension generator clauses on a path and the clauses of the innermost comprehension; report at the clause that crosses the limit.

**Allowed**:

```python
[x for row in rows for x in row]
```

**Violation**:

```python
[x for table in tables for row in table for x in row]
```

<a id="stylebrella-linter-python-idioms-4"></a>

<a id="sb-l-p-id-4"></a>

### SB-L-P-ID-4 — Pathlib instead of os.path

`Stylebrella-linter-python-idioms-4`

Report calls resolved to `os.path` functions for filesystem path operations; use `pathlib.Path` instead.

**Allowed**:

```python
path.exists()
```

**Violation**:

```python
os.path.exists(path)
```

<a id="stylebrella-linter-python-idioms-5"></a>

<a id="sb-l-p-id-5"></a>

### SB-L-P-ID-5 — Executable script headers and entry guards

`Stylebrella-linter-python-idioms-5`

A file identified by project metadata as an executable script in a `.venv` or `uv` project starts with `#!/usr/bin/env python3`. If identified as both importable and executable, its invocation must be inside `if __name__ == "__main__":`.

**Allowed**:

```python
#!/usr/bin/env python3
if __name__ == "__main__":
  main()
```

**Violation** (executable and importable):

```python
main()
```

<a id="stylebrella-linter-python-idioms-6"></a>

<a id="sb-l-p-id-6"></a>

### SB-L-P-ID-6 — Logging and ASCII console messages

`Stylebrella-linter-python-idioms-6`

Report calls resolved to built-in `print` in files explicitly classified as production code. Require ASCII in resolved static console-message text; the actual language of the prose is reviewed separately.

**Allowed**:

```python
logger.info("Loaded")
```

**Violation** (production):

```python
print("Loaded")
```

## `imports`

<a id="stylebrella-linter-python-imports-1"></a>

<a id="sb-l-p-i-1"></a>

### SB-L-P-I-1 — Wildcard imports and explicit package exports

`Stylebrella-linter-python-imports-1`

Reject wildcard imports except explicit package reexports in `__init__.py` whose source exports are statically resolved and whose public exposure is bounded by an explicit `__all__`. Declare package exports with `__all__`, not redundant self-aliases or suppression directives added to mark reexports.

**Allowed**:

```python
from .service import UserService
__all__ = ["UserService"]
```

**Violation** (outside __init__.py):

```python
from .service import *
```

<a id="stylebrella-linter-python-imports-2"></a>

<a id="sb-l-p-i-2"></a>

### SB-L-P-I-2 — Cyclic imports

`Stylebrella-linter-python-imports-2`

Report statically resolved cycles in the project’s unconditional import graph. Dynamic/unresolved imports do not form graph edges. Do not automatically introduce a late import to break a cycle.

**Allowed**:

```python
# a.py
from b import y

# b.py
y = load()
```

**Violation**:

```python
# a.py
from b import y

# b.py
from a import x
```

## `source`

<a id="stylebrella-linter-python-source-1"></a>

<a id="sb-l-p-s-1"></a>

### SB-L-P-S-1 — Unauthorized diagnostic suppressions

`Stylebrella-linter-python-source-1`

Report `# noqa` and rule/file exclusions not explicitly authorized in project configuration. A preapproved exception applies only to the exact registered rule and scope.

**Allowed**:

```python
value = read()
```

**Violation**:

```python
value = read()  # noqa
```

## `documentation`

<a id="stylebrella-linter-python-documentation-1"></a>

<a id="sb-l-p-d-1"></a>

### SB-L-P-D-1 — When a function needs a docstring

`Stylebrella-linter-python-documentation-1`

Require a multiline docstring as the first statement in every public nontrivial, important, or descriptively MARKed function and in any function with more than 15 executable body lines or more than 3 declared parameters, excluding `self` and `cls`. Count `*args` and `**kwargs` once each. Use [SB-L-G-M-2](../linting.md#stylebrella-linter-general-mark-2) to count lines and importance. Exactly 15 lines or 3 parameters does not by itself trigger the rule.

**Allowed**:

```python
def f(a: int, b: int, c: int, d: int) -> None:
  """
  Передаёт входные значения обработчику.

  Args:
    a: Первое входное значение.
    b: Второе входное значение.
    c: Третье входное значение.
    d: Четвёртое входное значение.
  """

  consume(a, b, c, d)
```

**Violation**:

```python
def f(a: int, b: int, c: int, d: int) -> None:
  consume(a, b, c, d)
```

<a id="stylebrella-linter-python-documentation-2"></a>

<a id="sb-l-p-d-2"></a>

### SB-L-P-D-2 — Google-style docstring structure

`Stylebrella-linter-python-documentation-2`

Place opening and closing triple double quotes on their own lines. Begin with a nonempty summary. Separate paragraphs and sections with one blank line; use Google `Args:`, `Returns:`, `Raises:`, and `Attributes:` sections where applicable, with entries indented by two spaces. Require exact parameter or attribute names in declaration order, no receiver or repeated type in the label, and nonempty descriptions for applicable entries. `Returns:` applies to a resolved non-None result; `Raises:` applies to known escaping exceptions. Omit inapplicable sections.

**Allowed**:

```python
"""
  Returns the count.

  Returns:
    Number of rows.
  """
```

**Violation**:

```python
"""Returns the count."""
```

## `comments`

<a id="stylebrella-linter-python-comments-1"></a>

<a id="sb-l-p-cm-1"></a>

### SB-L-P-CM-1 — Comments on dataclass fields

`Stylebrella-linter-python-comments-1`

Apply [SB-L-G-CM-1](../linting.md#stylebrella-linter-general-comments-1) to dataclass instance fields except exact names `id`, `user_id` and `item_id`.

**Allowed**:

```python
df_hz: float = 0.0  # Смещение, Гц
```

**Violation**:

```python
df_hz: float = 0.0
```

<a id="stylebrella-linter-python-comments-2"></a>

<a id="sb-l-p-cm-2"></a>

### SB-L-P-CM-2 — Constructor parameter categories

`Stylebrella-linter-python-comments-2`

Group resolved constructor parameter types with suffix `Repository`, `Mapper` or `Service` under matching `# repositories:`, `# mappers:` or `# services:` markers, or under an explicit configured category. Report a known parameter outside its category; unknown types do not trigger a finding.

**Allowed**:

```python
def __init__(
  self,
  # services:
  service: OrderService,
) -> None:
  self._service = service
```

**Violation**:

```python
def __init__(
  self,
  # mappers:
  service: OrderService,
) -> None:
  self._service = service
```

## `constants`

<a id="stylebrella-linter-python-constants-1"></a>

<a id="sb-l-p-c-1"></a>

### SB-L-P-C-1 — Shared constants and metadata exceptions

`Stylebrella-linter-python-constants-1`

Shared constants live in `constants.py` as `Constants.<Category>.<FIELD>` (e.g. Math, Entity, Validation, Pattern or a domain category). Report a named value referenced from two or more modules outside this container or a noncanonical reference to a resolved shared constant. Exempt `__main__` in the exact entry guard, names declared in `__all__`, and declarative pytest parameter/fixture names from [SB-L-G-C-1](../linting.md#stylebrella-linter-general-constants-1).

**Allowed**:

```python
Constants.Math.DEG_TO_RAD
```

**Violation** (module-level constant used by two modules):

```python
DEG_TO_RAD
```

## `testing`

<a id="stylebrella-linter-python-testing-1"></a>

<a id="sb-l-p-ts-1"></a>

### SB-L-P-TS-1 — Test file names, pytest, and shared fixtures

`Stylebrella-linter-python-testing-1`

Files classified as module tests must use `test_<module>.py`; tests in them use pytest, not `unittest.TestCase`. Place fixtures referenced by tests in multiple files in the nearest common `conftest.py`.

**Allowed** (`test_parser.py`):

```python
def test_parse() -> None:
  assert parse(DEFAULT_INPUT) == EXPECTED_RESULT
```

**Violation** (`parser_test.py`):

```python
class ParserTest(unittest.TestCase): ...
```

<a id="stylebrella-linter-python-testing-2"></a>

<a id="sb-l-p-ts-2"></a>

### SB-L-P-TS-2 — Environment-dependent tests and test data

`Stylebrella-linter-python-testing-2`

Mark tests explicitly classified as environment-dependent with pytest `skip` or `skipif` resolved on that test or its enclosing class/module. Apply [SB-L-G-C-1](../linting.md#stylebrella-linter-general-constants-1) to literal test inputs and parameter data.

**Allowed**:

```python
@pytest.mark.skipif(not has_service(), reason="Service unavailable")
def test_service() -> None:
  assert service.is_available()
```

**Violation** (environment-dependent):

```python
def test_service() -> None:
  assert service.is_available()
```

## Recommendations (no diagnostic)

Use `repr()` or `!r` when a diagnostic needs a value’s exact representation. Use a generator for lazy traversal of large data. Use dataclasses for records, frozen dataclasses for immutable values, validated models for configuration, enums for closed sets, and Protocol for structural interfaces. Design fixture scope and parameterize repeated cases; prefer `pytest.MonkeyPatch` for module-level substitutions and instance mocks for instance-level ones. Choose a logging level by intent. Prefer Russian docstring prose and explain exceptions that can reach callers. Resolve cycles through structure, rather than moving an import into a function.
