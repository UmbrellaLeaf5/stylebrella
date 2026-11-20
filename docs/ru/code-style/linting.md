# Общий линтинг

[English version](../../en/code-style/linting.md)

Нормативный контракт будущего `lint` и линтинга в `check`. Диагностика выдаётся только при подтверждённом условии правила. Недостающая информация или неразрешённая ссылка не считаются нарушением. Полный ID канонический, краткий — его равнозначный псевдоним. Нумерация начинается с 1 в каждом подразделе; выданные номера не используются повторно.

[Python](python/linting.md) · [Kotlin](kotlin/linting.md) · [C++](cpp/linting.md)

<a id="id-registry"></a>

## Реестр ID

Канонические ID имеют вид `Stylebrella-<вид>-<язык>-<подраздел>-<номер>`; псевдонимы используют `SB-<вид>-<язык>-<подраздел>-<номер>` и токены ниже.

| Канонический токен     | Токен псевдонима |
| ---------------------- | ---------------- |
| `formatter` / `linter` | `F` / `L`        |
| `general`              | `G`              |
| `python`               | `P`              |
| `kotlin`               | `K`              |
| `cpp`                  | `C`              |
| `blank_lines`          | `BL`             |
| `layout`               | `LY`             |
| `mark`                 | `M`              |
| `separators`           | `SP`             |
| `imports`              | `I`              |
| `source`               | `S`              |
| `expressions`          | `E`              |
| `naming`               | `N`              |
| `json`                 | `J`              |
| `comments`             | `CM`             |
| `constants`            | `C`              |
| `loops`                | `LP`             |
| `types`                | `T`              |
| `idioms`               | `ID`             |
| `testing`              | `TS`             |
| `documentation`        | `D`              |
| `nulls`                | `NU`             |
| `files`                | `FI`             |
| `exceptions`           | `EX`             |

Каждый пример изолирует указанное правило. Вне правил документации фрагмент функции может показывать только сигнатуру или инструкции, относящиеся к правилу; обязательная документация остаётся за пределами фрагмента.

## `naming`

<a id="stylebrella-linter-general-naming-1"></a>

<a id="sb-l-g-n-1"></a>

### SB-L-G-N-1 — Имена по виду объявления

`Stylebrella-linter-general-naming-1`

Имена проверяются по синтаксическому виду объявления и правилам языка; позиция диагностики — объявляемый идентификатор. Вид объявления не выводится из имени, линтинг его не переименовывает.

**Python**

**Допустимо**:

```python
total_count = count()
```

**Нарушение**:

```python
totalCount = count()
```

**Kotlin**

**Допустимо**:

```kotlin
val totalCount = count()
```

**Нарушение**:

```kotlin
val total_count = count()
```

**C++**

**Допустимо**:

```cpp
int total_count = count();
```

**Нарушение**:

```cpp
int totalCount = count();
```

<a id="stylebrella-linter-general-naming-2"></a>

<a id="sb-l-g-n-2"></a>

### SB-L-G-N-2 — Начальное подчёркивание у приватных методов

`Stylebrella-linter-general-naming-2`

Имя каждого приватного метода должно начинаться ровно с одного подчёркивания, после которого идёт имя по правилам языка. В Kotlin и C++ приватность определяется объявлением. В Python правило применяется, если метаданные проекта или явная секция `MARK: Private Helpers` указывают на приватный метод; по одному имени намерение определить нельзя. Специальные методы Python, имя которых начинается и заканчивается двумя подчёркиваниями, исключены. Диагностика ставится на имени метода; автоматического переименования нет.

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

## `json`

<a id="stylebrella-linter-general-json-1"></a>

<a id="sb-l-g-j-1"></a>

### SB-L-G-J-1 — Регистр статически разрешённых ключей JSON

`Stylebrella-linter-general-json-1`

Регистр проверяется только у статически разрешённого постоянного ключа поля сериализации или доступа к JSON. Диагностика ставится на ключе, не на произвольной строке. Допустимый регистр задаёт язык. Написание не меняется, а вне метаданных схемы продолжают действовать правила констант.

**Python**

**Допустимо**:

```python
record["user_id"]
```

**Нарушение**:

```python
record["User-ID"]
```

**Kotlin**

**Допустимо**:

```kotlin
@JsonProperty("user_id")
```

**Нарушение**:

```kotlin
@JsonProperty("userId")
```

**C++**

**Допустимо**:

```cpp
record["user_id"]
```

**Нарушение**:

```cpp
record["userId"]
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
```

**Нарушение**:

```python
# MARK: buildResponse
```

**Kotlin**

**Допустимо**:

```kotlin
// MARK: Build response
```

**Нарушение**:

```kotlin
// MARK: buildResponse
```

**C++**

**Допустимо**:

```cpp
// MARK: Build response
```

**Нарушение**:

```cpp
// MARK: BuildResponse
```

<a id="stylebrella-linter-general-mark-2"></a>

<a id="sb-l-g-m-2"></a>

### SB-L-G-M-2 — MARK для нетривиальных публичных и важных функций

`Stylebrella-linter-general-mark-2`

Функция нетривиальна, если её каноническое тело содержит хотя бы две исполняемые строки, хотя бы две исполняемые инструкции или вложенное управление. Важной считается функция, в которую разрешаются хотя бы две разные ссылки проекта помимо саморекурсии; ссылки в одном вызывающем методе считаются отдельно. Для публичной нетривиальной или важной функции нужен описательный MARK, кроме покрытия действительной группой или [SB-L-G-M-5](#stylebrella-linter-general-mark-5). Иначе достаточно разделителя. Строки тела считаются в каноническом формате без пустых строк, строк только с комментариями, документацией или разделителями синтаксиса.

**Python**

**Допустимо**:

```python
# MARK: Read data
# --------------------------------------------------
def read():
  load()
  validate()
```

**Нарушение**:

```python
# --------------------------------------------------
def read():
  load()
  validate()
```

**Kotlin**

**Допустимо**:

```kotlin
// MARK: Read data
// --------------------------------------------------
fun read() { load(); validate() }
```

**Нарушение**:

```kotlin
// --------------------------------------------------
fun read() { load(); validate() }
```

**C++**

**Допустимо**:

```cpp
// MARK: Read data
// --------------------------------------------------
void read() { load(); validate(); }
```

**Нарушение**:

```cpp
// --------------------------------------------------
void read() { load(); validate(); }
```

<a id="stylebrella-linter-general-mark-3"></a>

<a id="sb-l-g-m-3"></a>

### SB-L-G-M-3 — Группы MARK со счётчиком

`Stylebrella-linter-general-mark-3`

`MARK: <title> [group:N]` покрывает ровно следующие `N` объявлений функций или методов того же уровня текущей области. `N` — десятичное целое от 2. Вложенные определения и объявления иных видов не считаются. Диагностика: некорректный счётчик, другой MARK до N-го объявления или конец области при числе меньше N. Объявление N+1 вне группы.

**Python**

**Допустимо**:

```python
# MARK: Load adapters [group:2]
def a(): ...
def b(): ...
```

**Нарушение** (конец файла):

```python
# MARK: Load adapters [group:2]
def a(): ...
```

**Kotlin**

**Допустимо**:

```kotlin
// MARK: Load adapters [group:2]
fun a() = Unit
fun b() = Unit
```

**Нарушение**:

```kotlin
// MARK: Load adapters [group:1]
fun a() = Unit
```

**C++**

**Допустимо**:

```cpp
// MARK: Load adapters [group:2]
void a();
void b();
```

**Нарушение**:

```cpp
// MARK: Load adapters [group:1]
void a();
```

<a id="stylebrella-linter-general-mark-4"></a>

<a id="sb-l-g-m-4"></a>

### SB-L-G-M-4 — Соседние MARK

`Stylebrella-linter-general-mark-4`

Диагностика ставится на втором из двух MARK одного уровня, если между ними нет объявления или исполняемой инструкции. Пустые строки, разделители и документация соседство не разрывают. После `Private Helpers` диагностика предлагает заменить второй MARK блоком документации языка. `Private Helpers` — структурный, без счётчика и не закрывается более поздним описательным MARK.

**Python**

**Допустимо**:

```python
# MARK: Read
def read(): ...
# MARK: Write
```

**Нарушение**:

```python
# MARK: Private Helpers
# --------------------------------------------------
# MARK: Read
```

**Kotlin**

**Допустимо**:

```kotlin
// MARK: Read
fun read() = Unit
// MARK: Write
```

**Нарушение**:

```kotlin
// MARK: Private Helpers
// --------------------------------------------------
// MARK: Read
```

**C++**

**Допустимо**:

```cpp
// MARK: Read
void read();
// MARK: Write
```

**Нарушение**:

```cpp
// MARK: Private Helpers
// --------------------------------------------------
// MARK: Read
```

<a id="stylebrella-linter-general-mark-5"></a>

<a id="sb-l-g-m-5"></a>

### SB-L-G-M-5 — Документация первого важного приватного метода

`Stylebrella-linter-general-mark-5`

Структурный заголовок покрывает только первый приватный метод. Если он важный, вместо дополнительного MARK нужен блок документации на языке метода. Документированное объявление C++ удовлетворяет требованию и для определения.

**Python**

**Допустимо**:

```python
  # MARK: Private Helpers
  def _read(self) -> None:
    """
    Заполняет кэш из последнего сохранённого снимка.
    """

    self._cache.reload_snapshot()
```

**Нарушение**:

```python
  # MARK: Private Helpers
  def _read(self) -> None:
    self._cache.reload_snapshot()
```

**Kotlin**

**Допустимо**:

```kotlin
  // MARK: Private Helpers
  /**
   * Заполняет кэш из последнего сохранённого снимка.
   */
  private fun _read(): Unit = cache.reloadSnapshot()
```

**Нарушение**:

```kotlin
  // MARK: Private Helpers
  private fun _read(): Unit = cache.reloadSnapshot()
```

**C++**

**Допустимо**:

```cpp
  // MARK: Private Helpers
  /**
   * @brief Заполняет кэш из последнего сохранённого снимка.
   */
  void _read();
```

**Нарушение**:

```cpp
  // MARK: Private Helpers
  void _read();
```

<a id="stylebrella-linter-general-mark-6"></a>

<a id="sb-l-g-m-6"></a>

### SB-L-G-M-6 — Обязательные MARK класса

`Stylebrella-linter-general-mark-6`

Конкретному классу с публичными и приватными методами нужны MARK у публичного метода и отдельный структурный MARK. Группа со счётчиком не заменяет оба. Если важных публичных методов нет, MARK ставится у любого публичного. Исключения: интерфейсы, подтверждённые DTO; классу без приватных методов структурный MARK не нужен.

**Python**

**Допустимо**:

```python
  # MARK: Read
  def read(self): ...
  # MARK: Private Helpers
  def _read(self): ...
```

**Нарушение**:

```python
  def read(self): ...
  # MARK: Private Helpers
  def _read(self): ...
```

**Kotlin**

**Допустимо**:

```kotlin
  // MARK: Read
  fun read() = Unit
  // MARK: Private Helpers
  private fun _helper() = Unit
```

**Нарушение**:

```kotlin
  fun read() = Unit
  // MARK: Private Helpers
  private fun _helper() = Unit
```

**C++**

**Допустимо**:

```cpp
 public:
  // MARK: Read
  void read();
 private:
  // MARK: Private Helpers
  void _read();
```

**Нарушение**:

```cpp
 public:
  void read();
 private:
  // MARK: Private Helpers
  void _read();
```

<a id="stylebrella-linter-general-mark-7"></a>

<a id="sb-l-g-m-7"></a>

### SB-L-G-M-7 — Приватные методы после публичных

`Stylebrella-linter-general-mark-7`

В классе, подпадающем под [SB-L-G-M-6](#stylebrella-linter-general-mark-6), приватный метод не может стоять раньше публичного. Диагностика указывает на раннее приватное объявление; порядок не меняется. Видимость определяется по правилам языка, не по использованию.

**Python**

**Допустимо**:

```python
  def read(self): ...
  def _read(self): ...
```

**Нарушение**:

```python
  def _read(self): ...
  def read(self): ...
```

**Kotlin**

**Допустимо**:

```kotlin
  fun read() = Unit
  private fun _helper() = Unit
```

**Нарушение**:

```kotlin
  private fun _helper() = Unit
  fun read() = Unit
```

**C++**

**Допустимо**:

```cpp
public: void read();
private: void _read();
```

**Нарушение**:

```cpp
private: void _read();
public: void read();
```

## `documentation`

<a id="stylebrella-linter-general-documentation-1"></a>

<a id="sb-l-g-d-1"></a>

### SB-L-G-D-1 — Перед функцией разрешена только формальная документация

`Stylebrella-linter-general-documentation-1`

Документация обязательна для каждой именованной функции и метода, включая приватные, локальные и тривиальные. Проверяйте наличие и формат: [docstring в Python](python/linting.md#stylebrella-linter-python-documentation-1), [KDoc](kotlin/linting.md#stylebrella-linter-kotlin-documentation-1) и [Doxygen](cpp/linting.md#stylebrella-linter-cpp-documentation-1). Краткое описание обязательно: оно объясняет назначение, эффект или гарантию, а не просто пересказывает имя функции. Описания параметров и результата нужны только там, где они применимы; пустых секций быть не должно. Об отсутствии документации сообщайте у объявления, о неверном формате — у самого блока, не считая имеющийся, но неверно оформленный блок отсутствующим.

Диагностируйте любой обычный комментарий, прикреплённый сверху к функции, даже при наличии корректной документации. Комментарий считается прикреплённым, если он входит в непрерывную группу на уровне отступа функции без пустой строки перед объявлением или его аннотациями/декораторами; аннотации и декораторы группу не разрывают. Структурные MARK, разделители и распознанные директивы инструментов допустимы на своих местах. В Python внешнего блока документации функции нет: docstring находится внутри тела. Отделённые комментарии об окружающем коде не подпадают под правило.

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

Диагностика на заголовке `while`, если счётчик непосредственно перед циклом получает ноль, сравнивается через `<` с размером неизменяемой коллекции, увеличивается на один в конце каждой итерации и иначе используется только для индексации этой коллекции. Если счётчик нужен после цикла, обновления, ранний выход или побочные эффекты могут нарушить эквивалентность либо не хватает разрешения, диагностики нет. Цикл не переписывается.

**Python**

**Допустимо**:

```python
for item in items: visit(item)
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
for (item in items) visit(item)
```

**Нарушение**:

```kotlin
var i = 0
while (i < items.size) { visit(items[i]); i++ }
```

**C++**

**Допустимо**:

```cpp
for (const auto& item : items) visit(item);
```

**Нарушение**:

```cpp
int i = 0;
while (i < items.size()) { visit(items[i]); ++i; }
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
  sys.exit()
```

**Нарушение**:

```python
def run() -> None:
  sys.exit()


import sys
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
