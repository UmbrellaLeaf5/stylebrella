# Python linting

[Русская версия](../../../ru/code-style/python/linting.md)

Target contract for `.py`, alongside [shared linting](../linting.md) and [Python formatting](formatting.md). Notebooks are deferred. Enable Ruff rules `E`, `F`, `B`, `UP`, `RUF`, and `PL`, excluding `RUF001`, `RUF002`, `RUF003`, `F405`, `F403`, `PLR0912`, `PLR0915`, `PLR0913`, `PLR0911`, `B008`, `PLW2901`, `RUF012`, `PLC2401`, and `PLR2004`. Require zero outstanding warnings or errors. Import ordering (`I`) belongs to the formatting check, not `lint`. Do not add `# noqa`, per-file ignores, or further exclusions without explicit authorization. Pyright `basic` is a planned engine: its type errors must block the Python check once integrated.

## Names and annotations

Require `snake_case` for functions, methods, variables, parameters and fields; `PascalCase` for classes, data classes and exceptions; `SCREAMING_SNAKE_CASE` for constants; and `T` or `T`-prefixed PascalCase for generic type variables. Internal symbols take a leading underscore (`_cache`, `_build_response`, `_InternalType`) when known to be internal; do not infer module visibility from lack of references. For JSON keys, require the selected project convention (`snake_case` or `camelCase`), consistently within that project.

Require parameter and return annotations on functions and methods, including `-> None` where applicable; implicit `self`/`cls` receivers are exempt. Prefer `T | None` over `Optional[T]` on Python 3.10+. Flag explicit `Any` when resolved to `typing.Any`; use `object`, a real type, `Protocol`, or an ABC as appropriate. Recognized file-path parameters and results use `pathlib.Path`, not `str`; do not flag unrelated strings by their names alone.

| Pass                                                    | Report                                                                                              |
| ------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `def load_config(path: Path) -> Config: ...`            | `def loadConfig(path): ...` (name and missing types)                                                |
| `def find_user(user_id: str) -> User \| None: ...`      | `def find_user(user_id: str) -> Optional[User]: ...`                                                |
| `_cache`, `_InternalType`, `TItem`, `DEFAULT_TIMEOUT_S` | `Cache` as an internal variable, `ItemT` as a generic type variable, `defaultTimeout` as a constant |

## Language usage

- Use f-strings for interpolation. Diagnose known string `%` formatting or `.format()` calls, not arithmetic modulo. Prefer `repr()` or `!r` when exact values matter (review). Resolved logging calls are an exception: use lazy `%` placeholders with separate arguments, e.g. `logger.debug("Loaded %r", config)` rather than `logger.debug(f"Loaded {config!r}")`.
- Use `with` for resource lifetimes: file handles, connections, locks and custom context managers. Prefer `with path.open() as stream:` to a recognized unmanaged `path.open()` lifetime. Implement `__enter__`/`__exit__` or use `contextlib.contextmanager` for custom resources; if ownership or lifetime cannot be resolved, review it.
- Use comprehensions for simple transformations, with at most two nested levels (count nested comprehension nodes and nested generator clauses). Use generator expressions for lazy processing of large datasets; whether a dataset is large enough is review. Use [the shared loop rule](../linting.md#loop-intent).
- Prefer `pathlib.Path` to `os.path`. Use `dataclasses.dataclass` for structured containers, Pydantic `BaseModel` for validated configuration, `Enum`/`IntEnum` instead of sets of string constants representing enumerations, frozen dataclasses for immutable data, and `typing.Protocol` for structural interfaces. Diagnose recognized `os.path` calls and other explicit violations; classification of arbitrary data structures and the need for immutability remain review.
- Executable scripts in `.venv`/`uv` projects use `#!/usr/bin/env python3`. Scripts meant to be importable and executable guard execution with `if __name__ == "__main__":`. Identify executable/entrypoint status through project metadata or explicit conventions; do not infer it from arbitrary files.
- Use `logging`, not `print()` in identified production code. Use DEBUG, INFO, WARNING, ERROR and CRITICAL at the appropriate levels (review). Statically identifiable console message text must be English and ASCII; diagnose non-ASCII, review language and dynamic content.

```python
def load_config(path: Path) -> Config:
  with path.open() as stream:
    config = parse_config(stream)

  logger.debug("Loaded %r", config)

  return config
```

## Imports and exports

Reject production wildcard imports except `__init__.py` re-exports constrained by explicit `__all__`. Packages declare their public API in `__all__`; prefer this to redundant aliasing or suppression. Report a circular import only if a cycle is established from resolved project imports; review dynamic imports. Restructure cycles rather than introducing lazy imports unless necessary.

```python
# __init__.py — explicit exports
from .service import UserService


__all__ = ["UserService"]
```

## MARKs, method order, and documentation

Apply [shared MARK and method rules](../linting.md#mark-text-and-importance). `# MARK:` is the marker. A single leading underscore denotes a private method; exclude dunder methods such as `__init__` from the helper classification. Private helpers use the shared section and order, both at class level and for grouped private routines where appropriate. Do not assume an unresolvable dynamic reference makes a routine important.

Require a multiline Google-style docstring as the first statement of every non-trivial public function, every long or important routine, and any routine selected by a dedicated descriptive MARK. An important first private helper under `Private Helpers` needs the docstring even though it does not have its own MARK. Count long bodies as more than 15 nonblank, non-comment executable source lines in canonical layout, excluding the existing docstring. Trivial self-documenting routines and ordinary private helpers can omit the docstring when no requirement applies.

Put the opening and closing `"""` each on its own line. Start with a summary, followed by optional detail paragraphs and applicable English sections `Args:`, `Returns:`, `Raises:` (and `Attributes:` for documented classes). Put one blank line between paragraphs and sections. Indent section content two spaces relative to the label. Use `name: Description` for parameters and attributes without repeating signature types; use prose for `Returns:` and `ExceptionType: Description` for known escaping exceptions (backticks around symbols are allowed). Match names and order to the declaration, excluding implicit receivers; omit inapplicable sections. Report malformed or missing sections where evidence establishes applicability. Russian prose is preferred; quality is reviewed. Reject a single-line docstring when this block is required.

Dataclass properties require a same-line explanation of meaning and units, except `id`, `user_id`, and `item_id`. Group `__init__` parameters under categories such as `# repositories:`, `# mappers:`, `# services:` when resolved type suffixes identify the categories (`Repository`, `Mapper`, `Service`); use configured extensions and review unknown categories.

```python
# MARK: Load configuration
# --------------------------------------------------


def load_config(path: Path) -> Config:
  """
  Загружает конфигурацию из файла.

  Args:
    path: Путь к файлу конфигурации.

  Returns:
    Прочитанная конфигурация.
  """

  config = parse_config(path)
  validate_config(config)

  return config
```

## Constants and tests

Put shared values in `constants.py`, under `Constants.Math`, `Constants.Entity`, `Constants.Validation`, `Constants.Pattern`, or an appropriate extension for a different domain. Use `Constants.<Category>.<FIELD>` and uppercase names; e.g. `Constants.Math.DEG_TO_RAD` instead of `pi / 180.0` at a call site. Apply the [shared inline-literal policy](../linting.md#constants-and-inline-literals). `"__main__"` in an entry guard, resolved `__all__` names, and pytest's declarative parameter/fixture names are syntax metadata, not test data.

Use pytest and `test_<module>.py`; share focused fixtures in `conftest.py` and use `@pytest.mark.parametrize` for repeated test shapes. Use `pytest.MonkeyPatch` for module-level mocks and `unittest.mock` for instance-level mocks; mark environment-specific tests with `skip`/`skipif`. Check resolved framework calls and names; whether a fixture has one responsibility or tests should be parameterized is reviewed. Name test inputs rather than writing magic values directly in assertions or parameter lists.

```python
@pytest.mark.parametrize("name,valid", Constants.TestData.NAME_CASES)
def test_validate_name(name: str, valid: bool) -> None:
  assert User.is_valid_name(name) == valid
```
