# Shared linting

[Русская версия](../../../ru/code-style/general/linting.md)

Normative preview contract for `lint`, which checks lint rules only. `check` does not run lint rules. Emit a finding only when the stated predicate is established. Missing resolution or required context is not a violation. These rules describe future behavior.

[Python](../python/linting.md) · [Kotlin](../kotlin/linting.md) · [C++](../cpp/linting.md)

[Shared formatting](formatting.md) · [ID registry](../README.md#id-registry)

Examples isolate the rule being illustrated. Outside documentation rules, function examples are focused excerpts; any required documentation not shown is assumed present in omitted source context.

## `naming`

<a id="stylebrella-linter-general-naming-1"></a>

<a id="sb-l-g-n-1"></a>

### SB-L-G-N-1 — Naming by declaration kind

`Stylebrella-linter-general-naming-1`

Require PascalCase for class-like type declarations and SCREAMING_SNAKE_CASE for named constants in Python, Kotlin, and C++. Variables, properties, parameters, and functions follow the language-specific naming rules; this shared rule does not override them. Report at the declared identifier. Never infer a declaration's kind from its name or rename it as a lint action.

**Python**

**Allowed**:

```python
class NetworkConfig:
  pass
```

**Violation**:

```python
class networkConfig:
  pass
```

**Kotlin**

**Allowed**:

```kotlin
class NetworkConfig
```

**Violation**:

```kotlin
class networkConfig
```

**C++**

**Allowed**:

```cpp
class NetworkConfig {};
```

**Violation**:

```cpp
class networkConfig {};
```

**Python**

**Allowed** (constant):

```python
MAX_RETRIES = 3
```

**Violation** (constant):

```python
max_retries = 3
```

**Kotlin**

**Allowed** (constant):

```kotlin
const val MAX_RETRIES = 3
```

**Violation** (constant):

```kotlin
const val maxRetries = 3
```

**C++**

**Allowed** (constant):

```cpp
constexpr int MAX_RETRIES = 3;
```

**Violation** (constant):

```cpp
constexpr int max_retries = 3;
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

### SB-L-G-J-1 — snake_case for statically resolved JSON keys

`Stylebrella-linter-general-json-1`

Require `snake_case` for every statically resolved JSON key in Python, Kotlin, and C++, whether it appears at a serialization field or a JSON accessor. Report at the key, not an arbitrary string; unrelated strings are outside the rule. JSON keys are schema metadata and exempt from the inline-literal rule.

**Python**

**Allowed** (resolved JSON accessor):

```python
record["user_id"]
```

**Violation** (resolved JSON accessor):

```python
record["userId"]
```

**Kotlin**

**Allowed** (resolved serialization field):

```kotlin
@JsonProperty("user_id") val userId: String
```

**Violation** (resolved serialization field):

```kotlin
@JsonProperty("userId") val userId: String
```

**C++**

**Allowed** (resolved JSON accessor):

```cpp
record["user_id"];
```

**Violation** (resolved JSON accessor):

```cpp
record["userId"];
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
# --------------------------------------------------
```

**Violation**:

```python
# MARK: buildResponse
# --------------------------------------------------
```

**Kotlin**

**Allowed**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**Violation**:

```kotlin
// MARK: buildResponse
// --------------------------------------------------
```

**C++**

**Allowed**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

**Violation**:

```cpp
// MARK: BuildResponse
// --------------------------------------------------
```

<a id="stylebrella-linter-general-mark-2"></a>

<a id="sb-l-g-m-2"></a>

### SB-L-G-M-2 — MARK for nontrivial functions that are public or important

`Stylebrella-linter-general-mark-2`

A function is nontrivial if it declares more than three parameters or its canonical body spans more than 15 executable lines. Exclude type parameters and the receiver, including Python's `self` or `cls` when it serves as the receiver. Count each variadic or defaulted parameter once. Count body lines in the canonical format, excluding blank, comment-only, documentation-only and delimiter-only lines.

A function is important if at least two distinct project references resolve to it other than self-recursion; multiple references in one caller count separately. This rule requires a descriptive MARK only for a nontrivial function that is public or important, unless covered by a [valid group](#stylebrella-linter-general-mark-3) or [SB-L-G-M-5](#stylebrella-linter-general-mark-5). A trivial function does not need its own descriptive MARK or group under this rule, even if important. Keep `MARK: Private Helpers` where [SB-L-G-M-6](#stylebrella-linter-general-mark-6) requires it; do not insert a second MARK between that header and the first private method ([SB-L-G-M-4](#stylebrella-linter-general-mark-4)).

**Python**

**Allowed** (not important, exactly three parameters and 15 executable lines):

```python
# --------------------------------------------------

def run_backup(source: Source, target: Target, journal: Journal) -> None:
  """
  Сохраняет проверенную копию и фиксирует завершение в журнале.

  Args:
    source: Источник данных.
    target: Хранилище резервной копии.
    journal: Журнал операций копирования.
  """

  journal.begin()

  source.open()
  source.verify_manifest()
  source.read_metadata()
  source.read_index()

  target.prepare()
  target.check_capacity()
  target.write_metadata()
  target.write_index()

  source.copy_to(target)
  source.close()

  target.verify_checksum()
  target.flush()
  target.commit()

  journal.finish()
```

**Violation** (16 executable lines, no descriptive MARK):

```python
# --------------------------------------------------

def run_backup(source: Source, target: Target, journal: Journal) -> None:
  """
  Сохраняет проверенную копию и фиксирует завершение в журнале.

  Args:
    source: Источник данных.
    target: Хранилище резервной копии.
    journal: Журнал операций копирования.
  """

  journal.begin()

  source.open()
  source.verify_manifest()
  source.read_metadata()
  source.read_index()

  target.prepare()
  target.check_capacity()
  target.write_metadata()
  target.write_index()

  source.copy_to(target)
  source.close()

  target.verify_checksum()
  target.flush()
  target.commit()
  target.sync_directory()

  journal.finish()
```

**Kotlin**

**Allowed** (four parameters, one executable line, with a descriptive MARK):

```kotlin
// MARK: Update viewport bounds
// --------------------------------------------------

/**
 * Передаёт новые границы видимой области.
 *
 * @param left левая граница
 * @param top верхняя граница
 * @param right правая граница
 * @param bottom нижняя граница
 */
fun setBounds(left: Int, top: Int, right: Int, bottom: Int): Unit =
  view.set(left, top, right, bottom)
```

**Violation** (four parameters, one executable line, no descriptive MARK):

```kotlin
// --------------------------------------------------

/**
 * Передаёт новые границы видимой области.
 *
 * @param left левая граница
 * @param top верхняя граница
 * @param right правая граница
 * @param bottom нижняя граница
 */
fun setBounds(left: Int, top: Int, right: Int, bottom: Int): Unit =
  view.set(left, top, right, bottom)
```

**Allowed** (the first private method has four parameters; the structural MARK and KDoc suffice):

```kotlin
class BoundsUpdater {
  // MARK: Restore default viewport
  // --------------------------------------------------

  /**
   * Восстанавливает границы видимой области по умолчанию.
   */
  fun resetBounds(): Unit = _setBounds(defaults.left, defaults.top, defaults.right, defaults.bottom)


  // MARK: Private Helpers
  // --------------------------------------------------

  /**
   * Передаёт новые границы видимой области.
   *
   * @param left левая граница
   * @param top верхняя граница
   * @param right правая граница
   * @param bottom нижняя граница
   */
  private fun _setBounds(left: Int, top: Int, right: Int, bottom: Int): Unit =
    view.set(left, top, right, bottom)
}
```

**C++**

**Allowed** (both calls count, but `flush_log` is trivial and needs no descriptive MARK or group):

```cpp
// --------------------------------------------------

/**
 * @brief Сохраняет накопленные записи на диск.
 */
static void flush_log() {
  log.flush();
}


// --------------------------------------------------


/**
 * @brief Меняет файл журнала после сохранения записей.
 */
void rotate_log() {
  flush_log();
  log.rotate();
  flush_log();
}
```

<a id="stylebrella-linter-general-mark-3"></a>

<a id="sb-l-g-m-3"></a>

### SB-L-G-M-3 — Counted MARK groups

`Stylebrella-linter-general-mark-3`

A `MARK: <title> [group:N]` covers exactly the next `N` sibling function or method declarations in the current scope. `N` is a decimal integer at least 2. Ignore nested definitions and nonfunction declarations. The formatter removes the redundant `[group:1]` suffix under [SB-F-G-M-5](formatting.md#stylebrella-formatter-general-mark-5). Report other malformed counts, another MARK before the Nth declaration, or scope ending with fewer than N. The N+1th declaration is outside the group. Apply [SB-F-G-SP-2](formatting.md#stylebrella-formatter-general-separators-2) to require separators between group members.

**Python**

**Allowed**:

```python
# MARK: Load adapters [group:2]
# --------------------------------------------------

def a(): ...


# --------------------------------------------------


def b(): ...
```

**Violation** (fewer than two members at EOF):

```python
# MARK: Load adapters [group:2]
# --------------------------------------------------

def a(): ...
```

**Violation** (another MARK interrupts the three-member group):

```python
# MARK: Load adapters [group:3]
# --------------------------------------------------

def a(): ...


# --------------------------------------------------


# MARK: Parse adapters [group:2]
# --------------------------------------------------

def b(): ...


# --------------------------------------------------


def c(): ...
```

**Kotlin**

**Allowed**:

```kotlin
// MARK: Load adapters [group:2]
// --------------------------------------------------

fun a() = Unit


// --------------------------------------------------


fun b() = Unit
```

**Violation** (fewer than two members at EOF):

```kotlin
// MARK: Load adapters [group:2]
// --------------------------------------------------

fun a() = Unit
```

**Violation** (another MARK interrupts the three-member group):

```kotlin
// MARK: Load adapters [group:3]
// --------------------------------------------------

fun a() = Unit


// --------------------------------------------------


// MARK: Parse adapters [group:2]
// --------------------------------------------------

fun b() = Unit


// --------------------------------------------------


fun c() = Unit
```

**C++**

**Allowed**:

```cpp
// MARK: Load adapters [group:2]
// --------------------------------------------------

void a();


// --------------------------------------------------


void b();
```

**Violation** (fewer than two members at EOF):

```cpp
// MARK: Load adapters [group:2]
// --------------------------------------------------

void a();
```

**Violation** (another MARK interrupts the three-member group):

```cpp
// MARK: Load adapters [group:3]
// --------------------------------------------------

void a();


// --------------------------------------------------


// MARK: Parse adapters [group:2]
// --------------------------------------------------

void b();


// --------------------------------------------------


void c();
```

<a id="stylebrella-linter-general-mark-4"></a>

<a id="sb-l-g-m-4"></a>

### SB-L-G-M-4 — Adjacent MARKs

`Stylebrella-linter-general-mark-4`

Report the second of two sibling MARKs when there is no declaration or executable statement between them. Blank lines, separators and documentation do not break adjacency. After `Private Helpers`, recommend replacing the second MARK with the language's documentation block. `Private Helpers` is structural, has no count, and remains in effect across later descriptive MARKs.

**Python**

**Allowed**:

```python
# MARK: Read records
# --------------------------------------------------

def read(): ...


# MARK: Write results
# --------------------------------------------------

def write(): ...
```

**Violation**:

```python
# MARK: Private Helpers
# --------------------------------------------------

# MARK: Read records
# --------------------------------------------------

def read(): ...
```

**Kotlin**

**Allowed**:

```kotlin
// MARK: Read records
// --------------------------------------------------

fun read() = Unit


// MARK: Write results
// --------------------------------------------------

fun write() = Unit
```

**Violation**:

```kotlin
// MARK: Private Helpers
// --------------------------------------------------

// MARK: Read records
// --------------------------------------------------

fun read() = Unit
```

**C++**

**Allowed**:

```cpp
// MARK: Read records
// --------------------------------------------------

void read();


// MARK: Write results
// --------------------------------------------------

void write();
```

**Violation**:

```cpp
// MARK: Private Helpers
// --------------------------------------------------

// MARK: Read records
// --------------------------------------------------

void read();
```

<a id="stylebrella-linter-general-mark-5"></a>

<a id="sb-l-g-m-5"></a>

### SB-L-G-M-5 — Documentation for a nontrivial, important first private method

`Stylebrella-linter-general-mark-5`

The structural header covers the first private method only. If that method is both important and nontrivial under [SB-L-G-M-2](#stylebrella-linter-general-mark-2), its required language documentation takes the place of an additional MARK. In C++, document a method declared in a header there, not again at its source definition. Report missing documentation under this rule instead of the language presence rule at this boundary; report malformed documentation under the language format rule.

**Allowed** (Kotlin): `_applyBounds` has four parameters and two resolved call sites. Both entry points share the validation and update so an invalid region never reaches the viewport. KDoc satisfies the requirement at the private section boundary.

```kotlin
class ViewportController(private val viewport: Viewport) {
  // MARK: Restore viewport bounds [group:2]
  // --------------------------------------------------

  /**
   * Восстанавливает границы, сохранённые в профиле пользователя.
   *
   * @param profile профиль с сохранёнными границами
   * @throws IllegalArgumentException если сохранённые границы некорректны
   */
  fun restoreProfile(profile: ViewportProfile): Unit =
    _applyBounds(profile.left, profile.top, profile.right, profile.bottom)


  // --------------------------------------------------


  /**
   * Возвращает видимую область к границам рабочего пространства.
   *
   * @param limits границы рабочего пространства
   * @throws IllegalArgumentException если границы рабочего пространства некорректны
   */
  fun resetToWorkspace(limits: WorkspaceBounds): Unit =
    _applyBounds(limits.left, limits.top, limits.right, limits.bottom)


  // MARK: Private Helpers
  // --------------------------------------------------

  /**
   * Проверяет обе оси перед изменением видимой области.
   *
   * @param left левая граница
   * @param top верхняя граница
   * @param right правая граница
   * @param bottom нижняя граница
   * @throws IllegalArgumentException если одна из осей имеет неположительный размер
   */
  private fun _applyBounds(left: Int, top: Int, right: Int, bottom: Int) {
    require(left < right)
    require(top < bottom)

    viewport.setBounds(left, top, right, bottom)
  }
}
```

**Violation** (replace only the private section of `ViewportController` with this version):

```kotlin
  // MARK: Private Helpers
  // --------------------------------------------------

  private fun _applyBounds(left: Int, top: Int, right: Int, bottom: Int) {
    require(left < right)
    require(top < bottom)

    viewport.setBounds(left, top, right, bottom)
  }
```

**Allowed** (Python and C++ forms of the same private section): assume the same two public methods call the helper with all four bounds. In each case, removing only the shown documentation is a **violation**. The C++ example is the declaration in `viewport_controller.hpp`.

```python
  # MARK: Private Helpers
  # --------------------------------------------------

  def _apply_bounds(self, left: int, top: int, right: int, bottom: int) -> None:
    """
    Проверяет обе оси перед изменением видимой области.

    Args:
      left: Левая граница.
      top: Верхняя граница.
      right: Правая граница.
      bottom: Нижняя граница.

    Raises:
      ValueError: Одна из осей имеет неположительный размер.
    """

    if left >= right or top >= bottom:
      raise ValueError("Viewport bounds must have positive width and height")

    self._viewport.set_bounds(left, top, right, bottom)
```

```cpp
 private:
  // MARK: Private Helpers
  // --------------------------------------------------

  /**
   * @brief Проверяет обе оси перед изменением видимой области.
   * @param left Левая граница.
   * @param top Верхняя граница.
   * @param right Правая граница.
   * @param bottom Нижняя граница.
   * @throws std::invalid_argument Одна из осей имеет неположительный размер.
   */
  void _apply_bounds(int left, int top, int right, int bottom);
```

<a id="stylebrella-linter-general-mark-6"></a>

<a id="sb-l-g-m-6"></a>

### SB-L-G-M-6 — Required class MARKs

`Stylebrella-linter-general-mark-6`

A concrete class with public methods must have either a valid counted MARK group of public methods or a descriptive MARK attached to a public method the author selected as important. Do not second-guess that choice based on reference counts or body size. Apply [SB-L-G-M-2](#stylebrella-linter-general-mark-2) independently to methods not covered by the group or a descriptive MARK.

The class also needs a separate `MARK: Private Helpers` before its first private method, or the standalone `(no private methods)` comment after its last public method if it has no private methods. A public MARK or group does not replace either structural marker. Exempt interfaces and confirmed DTO-only types.

**Allowed** (Python, a single MARK chosen by the author and no private methods):

```python
class ExportQueue:
  # MARK: Dispatch next export
  # --------------------------------------------------

  def dispatch(self, pending: ExportStore, worker: ExportWorker) -> None:
    """
    Передаёт старейшее ожидающее задание свободному обработчику.

    Args:
      pending: Хранилище ожидающих заданий.
      worker: Свободный обработчик экспорта.
    """

    job = pending.take_oldest()

    worker.submit(job)


  # --------------------------------------------------


  def requeue(self, pending: ExportStore, job: ExportJob) -> None:
    """
    Возвращает прерванный экспорт в очередь.

    Args:
      pending: Хранилище ожидающих заданий.
      job: Прерванное задание экспорта.
    """

    pending.append(job)

  # (no private methods)
```

**Allowed** (Kotlin, a counted group): [`ViewportController` above](#stylebrella-linter-general-mark-5) groups `restoreProfile` and `resetToWorkspace` under one `[group:2]` MARK. The group's scope ends before the separate `MARK: Private Helpers`.

**Violation** (C++, the public MARK is present, but the class has no `(no private methods)` marker):

```cpp
/**
 * @brief Управляет сохранением результатов экспорта в журнале.
 */
class ExportJournal {
 public:
  // MARK: Persist export results
  // --------------------------------------------------

  /**
   * @brief Сохраняет результаты экспорта перед завершением работы.
   * @param journal Открытый журнал экспорта.
   */
  void flush(Journal& journal) const {
    journal.flush();
  }


  // --------------------------------------------------


  /**
   * @brief Закрывает журнал после передачи всех результатов.
   * @param journal Открытый журнал экспорта.
   */
  void close(Journal& journal) const {
    journal.close();
  }
};
```

The Python class would also violate this rule if its public MARK were removed, and the Kotlin class would violate it if its separate `Private Helpers` MARK were removed.

<a id="stylebrella-linter-general-mark-7"></a>

<a id="sb-l-g-m-7"></a>

### SB-L-G-M-7 — Private methods after public methods

`Stylebrella-linter-general-mark-7`

In a class with both public and private methods covered by [SB-L-G-M-6](#stylebrella-linter-general-mark-6), a private method must not precede any public method. Report the early private declaration; do not reorder declarations. Use declaration visibility, including the explicit Python private-section convention, rather than observed use.

**Allowed** (Python, public operations first and the private writer last):

```python
class AuditTrail:
  # MARK: Record export outcomes [group:2]
  # --------------------------------------------------

  def record_success(self, audit_log: AuditLog, job: ExportJob) -> None:
    """
    Фиксирует успешную передачу результата экспорта.

    Args:
      audit_log: Журнал аудита.
      job: Выполненное задание экспорта.
    """

    self._append(audit_log, job.completed_event())


  # --------------------------------------------------


  def record_failure(self, audit_log: AuditLog, job: ExportJob) -> None:
    """
    Фиксирует сбой, чтобы его можно было расследовать по журналу.

    Args:
      audit_log: Журнал аудита.
      job: Задание, завершившееся со сбоем.
    """

    self._append(audit_log, job.failed_event())


  # MARK: Private Helpers
  # --------------------------------------------------

  def _append(self, audit_log: AuditLog, event: AuditEvent) -> None:
    """
    Дописывает событие и сбрасывает буфер журнала.

    Args:
      audit_log: Журнал аудита.
      event: Событие для сохранения.
    """

    audit_log.append(event)
    audit_log.flush()
```

**Allowed** (Kotlin): both public methods of [`ViewportController`](#stylebrella-linter-general-mark-5) precede `_applyBounds`. In either class, moving the private method and its structural MARK before the public group would violate this rule.

**Violation** (C++, `_append` is private and appears before `record_success` and `record_failure`):

```cpp
/**
 * @brief Фиксирует результаты экспорта в журнале аудита.
 */
class AuditTrail {
 private:
  // MARK: Private Helpers
  // --------------------------------------------------

  /**
   * @brief Дописывает событие и сбрасывает буфер журнала.
   * @param audit_log Журнал аудита.
   * @param event Событие для сохранения.
   */
  void _append(AuditLog& audit_log, const AuditEvent& event) const {
    audit_log.append(event);
    audit_log.flush();
  }

 public:
  // MARK: Record export outcomes [group:2]
  // --------------------------------------------------

  /**
   * @brief Фиксирует успешную передачу результата экспорта.
   * @param audit_log Журнал аудита.
   * @param job Выполненное задание экспорта.
   */
  void record_success(AuditLog& audit_log, const ExportJob& job) const {
    _append(audit_log, job.completed_event());
  }


  // --------------------------------------------------


  /**
   * @brief Фиксирует сбой, чтобы его можно было расследовать по журналу.
   * @param audit_log Журнал аудита.
   * @param job Задание, завершившееся со сбоем.
   */
  void record_failure(AuditLog& audit_log, const ExportJob& job) const {
    _append(audit_log, job.failed_event());
  }
};
```

The C++ class satisfies the order rule when the `private:` section and its MARK follow both public methods.

## `documentation`

<a id="stylebrella-linter-general-documentation-1"></a>

<a id="sb-l-g-d-1"></a>

### SB-L-G-D-1 — Only formal documentation may introduce a function

`Stylebrella-linter-general-documentation-1`

Require documentation for every named function and method, including private, local, and trivial ones. Check both presence and format: [Python docstrings](../python/linting.md#stylebrella-linter-python-documentation-1), [KDoc](../kotlin/linting.md#stylebrella-linter-kotlin-documentation-1), and [Doxygen](../cpp/linting.md#stylebrella-linter-cpp-documentation-1). A summary is always required and must explain the purpose, effect, or guarantee, not merely restate the function name. Require parameter and result descriptions where the language rule calls for them; never require empty sections. Report missing documentation at the declaration and malformed documentation at its block, rather than reporting a present but malformed block as missing.

Report any ordinary comment attached above a function, even when valid documentation is also present. If an ordinary comment is the only documentation, report the invalid substitute here rather than also reporting missing documentation under the language presence rule. A comment is attached if it is in the contiguous leading group at the function's indentation, without a blank line before its declaration or attached annotations/decorators; annotations and decorators do not break the group. Allow structural MARKs, separators, and recognized tool directives in their own positions. Python has no external function documentation: the docstring belongs inside the body. Detached comments about surrounding code are outside this rule.

**C++ note:** Put the detailed Doxygen documentation on the declaration in the header. Repeating it at the corresponding `.cpp` definition is a violation of [SB-L-C-D-1](../cpp/linting.md#stylebrella-linter-cpp-documentation-1).

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

**Violation** (an ordinary one-line comment cannot replace formal documentation):

```python
# Передаёт планировщику только готовые задачи.
def start_tasks() -> None:
  scheduler.enqueue_ready()
```

```kotlin
// Передаёт планировщику только готовые задачи.
fun startTasks(): Unit = scheduler.enqueueReady()
```

```cpp
// Передаёт планировщику только готовые задачи.
void start_tasks();
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

Report a `while` at its header when its counter is initialized to zero immediately before it, tests `<` against the size of an unchanged collection, increments once by one after processing each element, and has no other counter use except indexing that collection or preserving the final counter. This includes a C++ loop with an empty body when its condition performs the same bound check, visit, and increment in that order and cannot terminate early. If the final counter is used after the loop, any update, early exit, or side effect may change equivalence, or resolution is missing, do not report. Do not rewrite the loop.

**Python**

**Allowed**:

```python
for item in items:
  visit(item)
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
for (item in items)
  visit(item)
```

**Violation**:

```kotlin
var i = 0
while (i < items.size) {
  visit(items[i])
  i++
}
```

**C++** (both examples traverse a resolved standard vector; `visit` returns `void` and does not change the vector)

**Allowed**:

```cpp
for (const auto& item : items)
  visit(item);
```

**Violation**:

```cpp
std::size_t i = 0;
while (i < items.size() && (visit(items[i]), ++i, true))
  ;
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
  """
  Завершает процесс с кодом успешного выполнения.
  """

  sys.exit()
```

**Violation**:

```python
def run() -> None:
  """
  Завершает процесс с кодом успешного выполнения.
  """

  import sys

  sys.exit()
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
