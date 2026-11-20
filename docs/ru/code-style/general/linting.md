# Общий линтинг

[English version](../../../en/code-style/general/linting.md)

Нормативный контракт будущего `lint`, который проверяет только правила линтинга. `check` не запускает правила линтинга. Диагностика выдаётся только при подтверждённом условии правила. Недостающая информация или неразрешённая ссылка не считаются нарушением.

[Python](../python/linting.md) · [Kotlin](../kotlin/linting.md) · [C++](../cpp/linting.md)

[Общее форматирование](formatting.md) · [Реестр ID](../README.md#id-registry)

Каждый пример изолирует указанное правило. Вне правил документации примеры функций являются фрагментами; обязательная документация, не показанная в примере, считается находящейся в опущенном контексте исходного кода.

## `naming`

<a id="stylebrella-linter-general-naming-1"></a>

<a id="sb-l-g-n-1"></a>

### SB-L-G-N-1 — PascalCase для типов классов

`Stylebrella-linter-general-naming-1`

Для объявлений классовых типов в Python, Kotlin и C++ требуется PascalCase. Диагностика ставится на объявляемом идентификаторе. Не определяйте по имени, является ли объявление типом, и не переименовывайте его при линтинге.

**Python**

**Допустимо**:

```python
class NetworkConfig:
  pass
```

**Нарушение**:

```python
class networkConfig:
  pass
```

**Kotlin**

**Допустимо**:

```kotlin
class NetworkConfig
```

**Нарушение**:

```kotlin
class networkConfig
```

**C++**

**Допустимо**:

```cpp
class NetworkConfig {};
```

**Нарушение**:

```cpp
class networkConfig {};
```

<a id="stylebrella-linter-general-naming-2"></a>

<a id="sb-l-g-n-2"></a>

### SB-L-G-N-2 — SCREAMING_SNAKE_CASE для именованных констант

`Stylebrella-linter-general-naming-2`

Для именованных констант в Python, Kotlin и C++ требуется SCREAMING_SNAKE_CASE. Определяйте константы по правилам языка, а не по написанию имени. Диагностика ставится на объявляемом идентификаторе; автоматически его не переименовывайте. Неконстантные переменные, свойства, параметры и функции именуются по правилам конкретного языка.

**Python**

**Допустимо** (константа):

```python
MAX_RETRIES = 3
```

**Нарушение** (константа):

```python
max_retries = 3
```

**Kotlin**

**Допустимо** (константа):

```kotlin
const val MAX_RETRIES = 3
```

**Нарушение** (константа):

```kotlin
const val maxRetries = 3
```

**C++**

**Допустимо** (константа):

```cpp
constexpr int MAX_RETRIES = 3;
```

**Нарушение** (константа):

```cpp
constexpr int max_retries = 3;
```

<a id="stylebrella-linter-general-naming-3"></a>

<a id="sb-l-g-n-3"></a>

### SB-L-G-N-3 — Начальное подчёркивание у приватных методов и полей C++

`Stylebrella-linter-general-naming-3`

Имя каждого приватного метода должно начинаться ровно с одного подчёркивания, после которого идёт имя по правилам языка. В Kotlin и C++ приватность определяется объявлением. В Python правило применяется, если метаданные проекта или явная секция `MARK: Private Helpers` указывают на приватный метод; по одному имени намерение определить нельзя. Специальные методы Python, имя которых начинается и заканчивается двумя подчёркиваниями, исключены. Диагностика ставится на имени метода; автоматического переименования нет.

В C++ такой же префикс нужен у приватных полей, не являющихся константами, если получившееся имя не зарезервировано. Члены `protected` не получают приватный префикс. Диагностика ставится на имени поля; автоматического переименования нет.

**Python** — проект объявил `copy` приватным.

**Допустимо**:

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

**Нарушение**:

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

**Допустимо**:

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

**Нарушение**:

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

**Допустимо**:

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

**Нарушение**:

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

**C++ — допустимо** (приватное поле):

```cpp
 private:
  int _max_retries;
```

**C++ — нарушение** (у приватного поля нет префикса):

```cpp
 private:
  int max_retries;
```

<a id="stylebrella-linter-general-naming-4"></a>

<a id="sb-l-g-n-4"></a>

### SB-L-G-N-4 — Без начального подчёркивания у публичных методов и самостоятельных функций

`Stylebrella-linter-general-naming-4`

В Kotlin и C++ запрещайте начальное подчёркивание у публичных методов с учётом действующей видимости объявления, включая неявный `public`. В Python проверка публичных методов не применяется. Во всех трёх языках запрещайте начальное подчёркивание у именованных функций, не являющихся методами: на верхнем уровне, в пространстве имён и локально, где язык это поддерживает, независимо от видимости. Специальные имена Python остаются исключением. Диагностика ставится на имени объявления; автоматического переименования нет.

**Python — допустимо / нарушение** (функции уровня модуля и локальные функции; проверка публичных методов неприменима):

```python
def read_value(value: str) -> str:
  """
  Удаляет пробелы по краям входной строки.

  Args:
    value: Исходная строка.

  Returns:
    Строка без пробелов по краям.
  """

  def normalize(text: str) -> str:
    """
    Удаляет начальные и конечные пробельные символы.

    Args:
      text: Текст для нормализации.

    Returns:
      Нормализованный текст.
    """

    return text.strip()

  return normalize(value)
```

```python
def _read_value(value: str) -> str:
  """
  Удаляет пробелы по краям входной строки.

  Args:
    value: Исходная строка.

  Returns:
    Строка без пробелов по краям.
  """

  def _normalize(text: str) -> str:
    """
    Удаляет начальные и конечные пробельные символы.

    Args:
      text: Текст для нормализации.

    Returns:
      Нормализованный текст.
    """

    return text.strip()

  return _normalize(value)
```

**Kotlin — допустимо / нарушение** (неявно публичный метод, приватная функция верхнего уровня и локальная функция):

```kotlin
class Reader {
  // MARK: Read value
  // --------------------------------------------------

  /**
   * Возвращает значение без изменений.
   *
   * @param value исходная строка
   * @return исходная строка без изменений
   */
  fun read(value: String): String = value

  // (no private methods)
}

/**
 * Удаляет пробельные символы по краям строки.
 *
 * @param value исходная строка
 * @return строка без пробелов по краям
 */
private fun readValue(value: String): String {
  /**
   * Нормализует края текста.
   *
   * @param text текст для нормализации
   * @return нормализованный текст
   */
  fun normalize(text: String): String = text.trim()

  return normalize(value)
}
```

```kotlin
class Reader {
  // MARK: Read value
  // --------------------------------------------------

  /**
   * Возвращает значение без изменений.
   *
   * @param value исходная строка
   * @return исходная строка без изменений
   */
  fun _read(value: String): String = value

  // (no private methods)
}

/**
 * Удаляет пробельные символы по краям строки.
 *
 * @param value исходная строка
 * @return строка без пробелов по краям
 */
private fun _readValue(value: String): String {
  /**
   * Нормализует края текста.
   *
   * @param text текст для нормализации
   * @return нормализованный текст
   */
  fun _normalize(text: String): String = text.trim()

  return _normalize(value)
}
```

**C++ — допустимо / нарушение** (публичный метод и функция в пространстве имён):

```cpp
namespace input {

/**
 * @brief Возвращает входное значение без преобразования.
 */
class Reader {
 public:
  // MARK: Read value
  // --------------------------------------------------

  /**
   * @brief Возвращает переданное значение без изменений.
   * @param value Входное значение.
   * @return Исходное значение.
   */
  int read(int value) const { return value; }

  // (no private methods)
};

/**
 * @brief Передаёт исходное значение без преобразования.
 * @param value Входное значение.
 * @return Исходное значение.
 */
int read_value(int value) { return value; }

}  // namespace input
```

```cpp
namespace input {

/**
 * @brief Возвращает входное значение без преобразования.
 */
class Reader {
 public:
  // MARK: Read value
  // --------------------------------------------------

  /**
   * @brief Возвращает переданное значение без изменений.
   * @param value Входное значение.
   * @return Исходное значение.
   */
  int _read(int value) const { return value; }

  // (no private methods)
};

/**
 * @brief Передаёт исходное значение без преобразования.
 * @param value Входное значение.
 * @return Исходное значение.
 */
int _read_value(int value) { return value; }

}  // namespace input
```

## `files`

<a id="stylebrella-linter-general-files-1"></a>

<a id="sb-l-g-fi-1"></a>

### SB-L-G-FI-1 — Не более одного класса верхнего уровня на файл

`Stylebrella-linter-general-files-1`

В исходном файле допускается не более одного определения именованного класса верхнего уровня. Классы, вложенные в другой тип или локальные для функции, не учитываются. В C++ область пространства имён считается верхним уровнем, в том числе если классы находятся в разных пространствах имён одного файла; предварительные объявления не учитываются. Отдельных функций может быть сколько угодно, как рядом с классом, так и в файле без классов. Диагностика ставится на имя каждого дополнительного класса верхнего уровня независимо от его видимости. Языковые правила для файлов могут также ограничивать другие виды типов верхнего уровня.

**Python — допустимо**:

```python
class Reader:
  """Группирует типы, используемые при чтении."""

  class Cursor:
    """Отмечает позицию в потоке."""


def input_is_empty(text: str) -> bool:
  """
  Проверяет, есть ли данные для чтения.

  Args:
    text: Входной текст.

  Returns:
    Истина, если вход пуст.
  """

  return not text
```

**Kotlin — допустимо** (`Reader.kt`):

```kotlin
class Reader {
  class Cursor
}


/**
 * Проверяет, есть ли данные для чтения.
 *
 * @param text входной текст
 * @return истина, если вход пуст
 */
fun inputIsEmpty(text: String): Boolean = text.isEmpty()
```

**C++ — допустимо** (`include/Reader.hpp`):

```cpp
#pragma once

/**
 * @brief Группирует типы, используемые при чтении.
 */
class Reader {
 public:
  /**
   * @brief Отмечает позицию в потоке.
   */
  class Cursor {};
};

/**
 * @brief Проверяет, есть ли данные для чтения.
 * @param input Входной поток.
 * @return Истина, если поток пуст.
 */
bool input_is_empty(const Input& input);
```

**Нарушение:** добавьте следующий класс в конец соответствующего допустимого файла. Диагностика ставится на `Writer`; вложенный `Cursor` и отдельная функция не учитываются.

```python
class Writer:
  """Группирует типы, используемые при записи."""
```

```kotlin
class Writer
```

```cpp
/**
 * @brief Группирует типы, используемые при записи.
 */
class Writer {};
```

## `mark`

<a id="stylebrella-linter-general-mark-1"></a>

<a id="sb-l-g-m-1"></a>

### SB-L-G-M-1 — Заголовки описательных MARK

`Stylebrella-linter-general-mark-1`

Описательный MARK непуст, начинается с прописной буквы ASCII и не может состоять только из идентификатора отмеченного объявления в camelCase или PascalCase. Разрешён HTTP-метод с маршрутом для подтверждённого обработчика. Перед проверкой заголовка удаляется точный завершающий суффикс группы.

**Python**

**Допустимо**:

```python
# MARK: Build response
# --------------------------------------------------
```

**Нарушение**:

```python
# MARK: buildResponse
# --------------------------------------------------
```

**Kotlin**

**Допустимо**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**Нарушение**:

```kotlin
// MARK: buildResponse
// --------------------------------------------------
```

**C++**

**Допустимо**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

**Нарушение**:

```cpp
// MARK: BuildResponse
// --------------------------------------------------
```

<a id="stylebrella-linter-general-mark-2"></a>

<a id="sb-l-g-m-2"></a>

### SB-L-G-M-2 — MARK для нетривиальных функций, которые публичны или важны

`Stylebrella-linter-general-mark-2`

Функция нетривиальна, если в её сигнатуре объявлено более трёх параметров или её каноническое тело занимает более 15 исполняемых строк. Параметры типа и получатель не учитываются; в Python это относится к `self` и `cls`, когда они служат получателем. Каждый вариативный параметр или параметр со значением по умолчанию считается один раз. Строки тела считаются в каноническом формате без пустых строк, строк только с комментариями, документацией или разделителями синтаксиса.

Функция считается важной, если на неё указывают хотя бы две разные ссылки из проекта, не считая саморекурсии; ссылки из одной вызывающей функции считаются отдельно. По этому правилу описательный MARK нужен только нетривиальной функции, которая публична или важна, если её не покрывает [действительная группа](#stylebrella-linter-general-mark-3) или правило [SB-L-G-M-5](#stylebrella-linter-general-mark-5). Тривиальной функции не нужен собственный описательный MARK или группа по этому правилу, даже если она важна. За `MARK: Private Helpers` отвечает форматтер по [SB-F-G-M-2](formatting.md#sb-f-g-m-2); не вставляйте второй MARK между этим заголовком и первым приватным методом ([SB-L-G-M-4](#stylebrella-linter-general-mark-4)).

**Python**

**Допустимо** (функция не важная, ровно три параметра и 15 исполняемых строк):

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

**Нарушение** (16 исполняемых строк, нет описательного MARK):

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

**Допустимо** (четыре параметра, одна исполняемая строка, есть описательный MARK):

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

**Нарушение** (четыре параметра, одна исполняемая строка, нет описательного MARK):

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

**Допустимо** (у первого приватного метода четыре параметра; структурного MARK и KDoc достаточно):

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

**Допустимо** (оба вызова учитываются, но `flush_log` тривиальна: описательный MARK и группа не нужны):

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

### SB-L-G-M-3 — Группы MARK со счётчиком

`Stylebrella-linter-general-mark-3`

`MARK: <title> [group:N]` покрывает ровно следующие `N` объявлений функций или методов того же уровня текущей области. `N` — десятичное целое от 2. Вложенные определения и объявления иных видов не считаются. Форматтер удаляет избыточный суффикс `[group:1]` по правилу [SB-F-G-M-4](formatting.md#stylebrella-formatter-general-mark-4). Диагностируются другие неверные счётчики, следующий MARK до N-го объявления или конец области при числе меньше N. Объявление N+1 вне группы. Между участниками группы требуются разделители по [SB-F-G-SP-2](formatting.md#stylebrella-formatter-general-separators-2).

**Python**

**Допустимо**:

```python
# MARK: Load adapters [group:2]
# --------------------------------------------------

def a(): ...


# --------------------------------------------------


def b(): ...
```

**Нарушение** (до конца области не хватает участников):

```python
# MARK: Load adapters [group:2]
# --------------------------------------------------

def a(): ...
```

**Нарушение** (другой MARK прерывает группу из трёх участников):

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

**Допустимо**:

```kotlin
// MARK: Load adapters [group:2]
// --------------------------------------------------

fun a() = Unit


// --------------------------------------------------


fun b() = Unit
```

**Нарушение** (до конца области не хватает участников):

```kotlin
// MARK: Load adapters [group:2]
// --------------------------------------------------

fun a() = Unit
```

**Нарушение** (другой MARK прерывает группу из трёх участников):

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

**Допустимо**:

```cpp
// MARK: Load adapters [group:2]
// --------------------------------------------------

void a();


// --------------------------------------------------


void b();
```

**Нарушение** (до конца области не хватает участников):

```cpp
// MARK: Load adapters [group:2]
// --------------------------------------------------

void a();
```

**Нарушение** (другой MARK прерывает группу из трёх участников):

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

### SB-L-G-M-4 — Соседние MARK

`Stylebrella-linter-general-mark-4`

Диагностика ставится на втором из двух MARK одного уровня, если между ними нет объявления или исполняемой инструкции. Пустые строки, разделители и документация соседство не разрывают. После `Private Helpers` диагностика предлагает заменить второй MARK блоком документации языка. `Private Helpers` — структурный, без счётчика и не закрывается более поздним описательным MARK.

**Python**

**Допустимо**:

```python
# MARK: Read records
# --------------------------------------------------

def read(): ...


# MARK: Write results
# --------------------------------------------------

def write(): ...
```

**Нарушение**:

```python
# MARK: Private Helpers
# --------------------------------------------------

# MARK: Read records
# --------------------------------------------------

def read(): ...
```

**Kotlin**

**Допустимо**:

```kotlin
// MARK: Read records
// --------------------------------------------------

fun read() = Unit


// MARK: Write results
// --------------------------------------------------

fun write() = Unit
```

**Нарушение**:

```kotlin
// MARK: Private Helpers
// --------------------------------------------------

// MARK: Read records
// --------------------------------------------------

fun read() = Unit
```

**C++**

**Допустимо**:

```cpp
// MARK: Read records
// --------------------------------------------------

void read();


// MARK: Write results
// --------------------------------------------------

void write();
```

**Нарушение**:

```cpp
// MARK: Private Helpers
// --------------------------------------------------

// MARK: Read records
// --------------------------------------------------

void read();
```

<a id="stylebrella-linter-general-mark-5"></a>

<a id="sb-l-g-m-5"></a>

### SB-L-G-M-5 — Документация первого приватного метода, если он важный и нетривиальный

`Stylebrella-linter-general-mark-5`

Структурный заголовок покрывает только первый приватный метод. Если он важный и нетривиальный по [SB-L-G-M-2](#stylebrella-linter-general-mark-2), обязательная документация на языке метода заменяет дополнительный MARK. Определяйте первый приватный метод по видимости и порядку объявлений, даже если заголовка нет: за его добавление отвечает [SB-F-G-M-2](formatting.md#sb-f-g-m-2). В C++ метод, объявленный в заголовочном файле, документируется там, а не повторно у определения в файле реализации.

**Допустимо** (Kotlin): у `_applyBounds` четыре параметра и два разрешённых места вызова. Обе точки входа используют общую проверку и обновление, чтобы не передавать в видимую область недопустимые границы. KDoc выполняет требование на границе приватной секции.

```kotlin
class ViewportController(private val _viewport: Viewport) {
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

    _viewport.setBounds(left, top, right, bottom)
  }
}
```

**Нарушение** (замените только приватную секцию `ViewportController` этой версией):

```kotlin
  // MARK: Private Helpers
  // --------------------------------------------------

  private fun _applyBounds(left: Int, top: Int, right: Int, bottom: Int) {
    require(left < right)
    require(top < bottom)

    _viewport.setBounds(left, top, right, bottom)
  }
```

**Допустимо** (та же приватная секция на Python и C++): те же два публичных метода вызывают вспомогательный метод со всеми четырьмя границами. В обоих случаях удаление только показанной документации — **нарушение**. В примере C++ показано объявление в `viewport_controller.hpp`.

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

### SB-L-G-M-6 — Обязательный MARK публичного метода класса

`Stylebrella-linter-general-mark-6`

Конкретному классу с публичными методами нужна либо действительная MARK-группа публичных методов со счётчиком, либо описательный MARK у публичного метода, который автор выбрал как важный. Линтер не оспаривает этот выбор по числу ссылок или размеру тела. К методам, не покрытым группой или описательным MARK, отдельно применяется [SB-L-G-M-2](#stylebrella-linter-general-mark-2).

Интерфейсы и подтверждённые DTO освобождены от правила. Линтер по этому правилу проверяет только публичный MARK или группу, написанные автором. Команда форматтера `check` без записи проверяет наличие и положение [секции Private Helpers (SB-F-G-M-2)](formatting.md#sb-f-g-m-2) или [заглушки `(no private methods)` (SB-F-G-CM-4)](formatting.md#sb-f-g-cm-4). Команда `format` исправляет эти маркеры; публичный MARK или группа их не заменяют.

**Допустимо** (Python, один MARK по выбору автора, приватных методов нет):

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

**Допустимо** (Kotlin, группа со счётчиком): [в классе `ViewportController` выше](#stylebrella-linter-general-mark-5) один MARK с `[group:2]` объединяет `restoreProfile` и `resetToWorkspace`. Группа заканчивается перед отдельным `MARK: Private Helpers`.

**Нарушение** (C++, нет описательного MARK публичного метода или группы публичных методов со счётчиком):

```cpp
/**
 * @brief Управляет сохранением результатов экспорта в журнале.
 */
class ExportJournal {
 public:
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

  // (no private methods)
};
```

Класс на Python также нарушил бы это правило без публичного MARK. Если убрать только заголовок `Private Helpers` в примере Kotlin или заглушку в примере Python, об этом сообщит `check`, а не это правило линтера.

<a id="stylebrella-linter-general-mark-7"></a>

<a id="sb-l-g-m-7"></a>

### SB-L-G-M-7 — Приватные методы после публичных

`Stylebrella-linter-general-mark-7`

В конкретном классе с публичными и приватными методами приватный метод не может стоять раньше публичного. Интерфейсы и подтверждённые DTO освобождены от правила. Диагностика указывает на раннее приватное объявление; порядок не меняется. Учитывайте видимость объявления, включая явную приватную секцию в Python, а не использование метода. При таком нарушении [SB-F-G-M-2](formatting.md#sb-f-g-m-2) пропускает правки структурных маркеров в этом классе.

**Допустимо** (Python, сначала публичные операции, затем приватный метод записи):

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

**Допустимо** (Kotlin): оба публичных метода [`ViewportController`](#stylebrella-linter-general-mark-5) стоят перед `_applyBounds`. В обоих классах перенос приватного метода вместе с его структурным MARK перед публичной группой нарушил бы правило.

**Нарушение** (C++, приватный `_append` стоит перед `record_success` и `record_failure`):

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

Класс на C++ соблюдает правило порядка, если секция `private:` со своим MARK идёт после обоих публичных методов.

## `documentation`

<a id="stylebrella-linter-general-documentation-1"></a>

<a id="sb-l-g-d-1"></a>

### SB-L-G-D-1 — Перед функцией разрешена только формальная документация

`Stylebrella-linter-general-documentation-1`

Документация обязательна для каждой именованной функции и метода, включая приватные, локальные и тривиальные. Проверяйте наличие и формат: [docstring в Python](../python/linting.md#stylebrella-linter-python-documentation-1), [KDoc](../kotlin/linting.md#stylebrella-linter-kotlin-documentation-1) и [Doxygen](../cpp/linting.md#stylebrella-linter-cpp-documentation-1). Краткое описание обязательно: оно объясняет назначение, эффект или гарантию, а не просто пересказывает имя функции. Описания параметров и результата нужны там, где их требует правило языка; пустых секций быть не должно. Об отсутствии документации сообщайте у объявления, о неверном формате — у самого блока, не считая имеющийся, но неверно оформленный блок отсутствующим.

Диагностируйте любой обычный комментарий, прикреплённый сверху к функции, даже при наличии корректной документации. Если обычный комментарий — единственная документация, сообщайте о недопустимой замене здесь, без повторной диагностики отсутствия документации по правилу языка. Комментарий считается прикреплённым, если он входит в непрерывную группу на уровне отступа функции без пустой строки перед объявлением или его аннотациями/декораторами; аннотации и декораторы группу не разрывают. Структурные MARK, разделители и распознанные директивы инструментов допустимы на своих местах. В Python внешнего блока документации функции нет: docstring находится внутри тела. Отделённые комментарии об окружающем коде не подпадают под правило.

**Примечание для C++:** подробный Doxygen размещается у объявления в заголовочном файле. Повторение документации у соответствующего определения в `.cpp` нарушает [SB-L-C-D-1](../cpp/linting.md#stylebrella-linter-cpp-documentation-1).

**Python — нарушение** (docstring не делает комментарий над функцией допустимым):

```python
# start tasks
def start_tasks() -> None:
  """
  Передаёт планировщику только задачи, готовые к выполнению.
  """

  scheduler.enqueue_ready()
```

**Kotlin — нарушение** (обычный комментарий перед KDoc):

```kotlin
// start tasks
/**
 * Передаёт планировщику только задачи, готовые к выполнению.
 */
fun startTasks(): Unit = scheduler.enqueueReady()
```

**Нарушение** (обычный однострочный комментарий не заменяет формальную документацию):

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

**C++ — нарушение** (обычный комментарий перед Doxygen):

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

### SB-L-G-CM-1 — Концевые комментарии у полей

`Stylebrella-linter-general-comments-1`

Для полей, указанных в языковых правилах, требуется непустой концевой комментарий на той же строке после объявления, кроме явных исключений по имени. Диагностика на поле; качество и единицы автоматически не оцениваются.

**Python**

**Допустимо**:

```python
df_hz: float = 0.0  # offset, Hz
```

**Нарушение**:

```python
df_hz: float = 0.0
```

**Kotlin**

**Допустимо**:

```kotlin
val dfHz: Double = 0.0, // offset, Hz
```

**Нарушение**:

```kotlin
val dfHz: Double = 0.0,
```

**C++**

**Допустимо**:

```cpp
double df_hz = 0.0; // offset, Hz
```

**Нарушение**:

```cpp
double df_hz = 0.0;
```

## `constants`

<a id="stylebrella-linter-general-constants-1"></a>

<a id="sb-l-g-c-1"></a>

### SB-L-G-C-1 — Именованные значения вместо встроенных литералов

`Stylebrella-linter-general-constants-1`

Диагностика для числовых и строковых литералов в значениях по умолчанию, аргументах, возвращаемых или присваиваемых значениях, выражениях и тестовых данных, если нет указанного исключения. Логические значения и null разрешены, ноль, единица и пустая строка — только как непосредственные инициализаторы, включая типизированные эквиваленты. Определение именованной константы не считается встроенным использованием. Диагностика на литерале; автоматического выноса или объединения нет.

**Python**

**Допустимо**:

```python
connect(Constants.Network.DEFAULT_PORT)
```

**Нарушение**:

```python
connect(8080)
```

**Kotlin**

**Допустимо**:

```kotlin
connect(Constants.Network.DEFAULT_PORT)
```

**Нарушение**:

```kotlin
connect(8080)
```

**C++**

**Допустимо**:

```cpp
connect(app::constants::network::DEFAULT_PORT);
```

**Нарушение**:

```cpp
connect(8080);
```

<a id="stylebrella-linter-general-constants-2"></a>

<a id="sb-l-g-c-2"></a>

### SB-L-G-C-2 — Исключения для литералов сообщений и метаданных

`Stylebrella-linter-general-constants-2`

Литеральный текст на месте статически разрешённого параметра сообщения вывода, лога или исключения разрешён, в том числе текстовые части интерполяции. Отдельные аргументы-данные и выражения внутри интерполяции не освобождаются. Также разрешены явно перечисленные в языковом контракте метаданные схемы и синтаксиса.

**Python**

**Допустимо**:

```python
logger.info("Connected")
```

**Нарушение**:

```python
logger.info("Port: %d", 8080)
```

**Kotlin**

**Допустимо**:

```kotlin
logger.info("Connected")
```

**Нарушение**:

```kotlin
logger.info("Port: {}", 8080)
```

**C++**

**Допустимо**:

```cpp
throw std::runtime_error("[Network::connect] Disconnected: peer=" + peer);
```

**Нарушение**:

```cpp
throw std::runtime_error("[Network::connect] Disconnected: port=" + std::to_string(8080));
```

## `loops`

<a id="stylebrella-linter-general-loops-1"></a>

<a id="sb-l-g-lp-1"></a>

### SB-L-G-LP-1 — For вместо эквивалентного цикла со счётчиком

`Stylebrella-linter-general-loops-1`

Диагностика на заголовке `while`, если счётчик непосредственно перед циклом получает ноль, сравнивается через `<` с размером неизменяемой коллекции, увеличивается на один после обработки каждого элемента и иначе используется только для индексации этой коллекции или сохранения итогового значения. В C++ сюда входит цикл с пустым телом, если условие в том же порядке проверяет границу, обрабатывает элемент и увеличивает счётчик и не может завершить цикл досрочно. Если счётчик нужен после цикла, обновления, ранний выход или побочные эффекты могут нарушить эквивалентность либо не хватает разрешения, диагностики нет. Цикл не переписывается.

**Python**

**Допустимо**:

```python
for item in items:
  visit(item)
```

**Нарушение**:

```python
i = 0
while i < len(items):
  visit(items[i])
  i += 1
```

**Kotlin**

**Допустимо**:

```kotlin
for (item in items)
  visit(item)
```

**Нарушение**:

```kotlin
var i = 0
while (i < items.size) {
  visit(items[i])
  i++
}
```

**C++** (в обоих примерах обходится стандартный вектор; `visit` возвращает `void` и не меняет вектор)

**Допустимо**:

```cpp
for (const auto& item : items)
  visit(item);
```

**Нарушение**:

```cpp
std::size_t i = 0;
while (i < items.size() && (visit(items[i]), ++i, true))
  ;
```

## `imports`

<a id="stylebrella-linter-general-imports-1"></a>

<a id="sb-l-g-i-1"></a>

### SB-L-G-I-1 — Импорты и включения в начале файла

`Stylebrella-linter-general-imports-1`

Импорты и включения возможны только в начальной преамбуле, до первого исполняемого оператора или объявления. В преамбуле разрешены комментарии, файловые директивы (включая docstring модуля, аннотации файла и `package` Kotlin, `#pragma once` C++) и условные ветви, состоящие только из элементов преамбулы. Импорты в функциях и иных не-файловых областях считаются поздними. Диагностика на директиве; форматтер её не переносит.

**Python**

**Допустимо**:

```python
import sys


# --------------------------------------------------


def run() -> None:
  """
  Завершает процесс с кодом успешного выполнения.
  """

  sys.exit()
```

**Нарушение**:

```python
def run() -> None:
  """
  Завершает процесс с кодом успешного выполнения.
  """

  import sys

  sys.exit()
```

**Kotlin**

**Допустимо**:

```kotlin
package app
import java.time.Clock


// --------------------------------------------------


val clock = Clock.systemUTC()
```

**Нарушение**:

```kotlin
package app
val clock = Clock.systemUTC()


import java.time.Clock
```

**C++**

**Допустимо**:

```cpp
#pragma once
#include <vector>


// --------------------------------------------------


void run();
```

**Нарушение**:

```cpp
#pragma once
void run();


#include <vector>
```

## Рекомендации (без диагностики)

Выбирайте содержательные имена и указывайте единицы измерения, где это полезно. Поясняйте смысл и единицы в комментариях к полям. Для новых и изменённых обычных комментариев предпочтителен русский; английский допустим. Описания MARK пишите по-английски. Объединяйте значения, совпадающие по смыслу, а не только по написанию. Простой обход пишите через `for`, а `while` оставляйте для неэквивалентного состояния или завершения. Код не должен зависеть от порядка импортов.
