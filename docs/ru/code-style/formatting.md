# Общее форматирование

[English version](../../en/code-style/formatting.md)

Нормативный контракт будущих `format`, `format --check` (без записи) и проверки форматирования в `check`. Правила описывают целевое поведение, а не статус реализации. Для каждого языка применяется один канонический форматтер, без цепочки альтернативных форматтеров. После Stylebrella и повторный проход Stylebrella, и отдельный проход форматтера языка с тем же пресетом Stylebrella должны оставлять файл побайтно неизменным. Несовместимость блокирует поддержку затронутого правила до пересмотра контракта или выбора форматтера.

[Python](python/formatting.md) · [Kotlin](kotlin/formatting.md) · [C++](cpp/formatting.md)

[Реестр ID и линтинг](linting.md#id-registry)

Каждый пример изолирует указанное правило.

## `blank_lines`

<a id="stylebrella-formatter-general-blank_lines-1"></a>

<a id="sb-f-g-bl-1"></a>

### SB-F-G-BL-1 — Пустые строки вокруг разделителя после импортов

`Stylebrella-formatter-general-blank_lines-1`

На верхнем уровне до и после разделителя по [SB-F-G-SP-3](#stylebrella-formatter-general-separators-3) оставляются ровно две пустые строки. Если разделитель относится к MARK, две строки перед ним ставятся перед самим MARK; после разделителя применяется [SB-F-G-BL-9](#stylebrella-formatter-general-blank_lines-9), если правило применимо.

**Python**

**До**:

```python
import time
# --------------------------------------------------
def run() -> float:
  return time.monotonic()
```

**После**:

```python
import time


# --------------------------------------------------


def run() -> float:
  return time.monotonic()
```

**Kotlin**

**До**:

```kotlin
import java.time.Instant
// --------------------------------------------------
fun run() = Instant.now()
```

**После**:

```kotlin
import java.time.Instant


// --------------------------------------------------


fun run() = Instant.now()
```

**C++**

**До**:

```cpp
#include "Clock.hpp"
// --------------------------------------------------
void run();
```

**После**:

```cpp
#include "Clock.hpp"


// --------------------------------------------------


void run();
```

<a id="stylebrella-formatter-general-blank_lines-2"></a>

<a id="sb-f-g-bl-2"></a>

### SB-F-G-BL-2 — Пустые строки между объявлениями верхнего уровня

`Stylebrella-formatter-general-blank_lines-2`

На верхнем уровне между соседними блоками объявлений и с каждой стороны их разделителя ставятся две пустые строки, кроме границы файла. Документация, декораторы и аннотации относятся к объявлению. Исключение между MARK и документацией задано [SB-F-G-BL-9](#stylebrella-formatter-general-blank_lines-9).

**Python**

**До**:

```python
def a() -> None:
  flush()
# --------------------------------------------------
def b() -> None:
  close()
```

**После**:

```python
def a() -> None:
  flush()


# --------------------------------------------------


def b() -> None:
  close()
```

**Kotlin**

**До**:

```kotlin
fun a() = Unit
// --------------------------------------------------
fun b() = Unit
```

**После**:

```kotlin
fun a() = Unit


// --------------------------------------------------


fun b() = Unit
```

**C++**

**До**:

```cpp
void a();
// --------------------------------------------------
void b();
```

**После**:

```cpp
void a();


// --------------------------------------------------


void b();
```

<a id="stylebrella-formatter-general-blank_lines-3"></a>

<a id="sb-f-g-bl-3"></a>

### SB-F-G-BL-3 — Пустые строки подряд в телах функций и методов

`Stylebrella-formatter-general-blank_lines-3`

В телах функций и методов каждая серия из двух и более пустых строк между инструкциями сокращается до одной. Правило одинаково применяется к отдельным функциям и методам класса. Оно не задаёт расстояние между объявлениями методов: это расстояние и разделитель перед первым методом регулируются [SB-F-G-SP-2](#stylebrella-formatter-general-separators-2). Между документацией, прикреплёнными аннотациями или декораторами и объявлением пустые строки не добавляются. Если на границе задано число строк, оно применяется один раз, а не складывается с другими требованиями.

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

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

**После**:

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

### SB-F-G-BL-4 — Отступы между группами локальных объявлений

`Stylebrella-formatter-general-blank_lines-4`

В списке инструкций соседние локальные объявления составляют одну группу. Прямые вызовы или присваивания полям последнего объявленного объекта продолжают его группу, если получатель разрешается в это объявление. От соседних инструкций вне группы её отделяет одна пустая строка; явные границы по комментариям сохраняются. Для серии присваиваний полям в Python действует [SB-F-P-BL-3](python/formatting.md#stylebrella-formatter-python-blank_lines-3).

**Python**

**До**:

```python
  log()
  f = Filter()

  f.init()
```

**После**:

```python
  log()

  f = Filter()
  f.init()
```

**Kotlin**

**До**:

```kotlin
  log()
  val f = Filter()

  f.init()
```

**После**:

```kotlin
  log()

  val f = Filter()
  f.init()
```

**C++**

**До**:

```cpp
  log();
  Filter f;

  f.init();
```

**После**:

```cpp
  log();

  Filter f;
  f.init();
```

<a id="stylebrella-formatter-general-blank_lines-5"></a>

<a id="sb-f-g-bl-5"></a>

### SB-F-G-BL-5 — Пустые строки между полями

`Stylebrella-formatter-general-blank_lines-5`

Соседние объявления полей не разделяются, кроме случая, когда у следующего поля есть предшествующий комментарий или аннотация: перед всей группой ставится одна пустая строка.

**Python**

**До**:

```python
  x: int

  y: int
```

**После**:

```python
  x: int
  y: int
```

**Kotlin**

**До**:

```kotlin
  val x: Int

  val y: Int
```

**После**:

```kotlin
  val x: Int
  val y: Int
```

**C++**

**До**:

```cpp
  int x;

  int y;
```

**После**:

```cpp
  int x;
  int y;
```

<a id="stylebrella-formatter-general-blank_lines-6"></a>

<a id="sb-f-g-bl-6"></a>

### SB-F-G-BL-6 — Пустая строка после многострочной инструкции

`Stylebrella-formatter-general-blank_lines-6`

После инструкции, занимающей несколько строк кода, перед следующей инструкцией того же уровня ставится одна пустая строка. Перед закрывающим разделителем она не добавляется. Внутри непрерывной группы объявлений действует [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4).

**Python**

**До**:

```python
  f(
    x,
  )
  g()
```

**После**:

```python
  f(
    x,
  )

  g()
```

**Kotlin**

**До**:

```kotlin
  f(
    x,
  )
  g()
```

**После**:

```kotlin
  f(
    x,
  )

  g()
```

**C++**

**До**:

```cpp
  f(
    x);
  g();
```

**После**:

```cpp
  f(
    x);

  g();
```

<a id="stylebrella-formatter-general-blank_lines-7"></a>

<a id="sb-f-g-bl-7"></a>

### SB-F-G-BL-7 — Пустые строки вокруг управляющих конструкций и между ветвями

`Stylebrella-formatter-general-blank_lines-7`

Отделяйте `if` и `for` от соседних инструкций того же уровня одной пустой строкой, в том числе после однострочного тела без скобок. Между последовательными ветвями условия или обработчиками исключений, начинающимися на разных строках, ставится одна пустая строка. Если языковое правило переносит ветвь с конца строки предыдущей ветви, применяйте этот интервал к новой границе; синтаксис с ветвями на одной строке регулируется языковым правилом переноса. Перед закрывающим разделителем и между заголовком и однострочным телом строка не добавляется.

**Python**

**До**:

```python
  if x: f()
  g()
```

**После**:

```python
  if x: f()

  g()
```

**Kotlin**

**До**:

```kotlin
  if (x) f()
  g()
```

**После**:

```kotlin
  if (x) f()

  g()
```

**C++**

**До**:

```cpp
  if (x) f();
  g();
```

**После**:

```cpp
  if (x) f();

  g();
```

<a id="stylebrella-formatter-general-blank_lines-8"></a>

<a id="sb-f-g-bl-8"></a>

### SB-F-G-BL-8 — Пустая строка перед return

`Stylebrella-formatter-general-blank_lines-8`

Перед `return`, следующим за соседней инструкцией того же уровня, ставится одна пустая строка. Перед единственной инструкцией блока и в однострочном управляющем теле она не добавляется.

**Python**

**До**:

```python
  send()
  return result
```

**После**:

```python
  send()

  return result
```

**Kotlin**

**До**:

```kotlin
  send()
  return result
```

**После**:

```kotlin
  send()

  return result
```

**C++**

**До**:

```cpp
  send();
  return result;
```

**После**:

```cpp
  send();

  return result;
```

<a id="stylebrella-formatter-general-blank_lines-9"></a>

<a id="sb-f-g-bl-9"></a>

### SB-F-G-BL-9 — Пустые строки после разделителя MARK

`Stylebrella-formatter-general-blank_lines-9`

Между разделителем MARK и прикреплённым KDoc или Doxygen ставится ровно одна пустая строка даже на верхнем уровне. В Python перед объявлением или его декоратором ставится одна строка, если активный пресет оставляет её неизменной; две допускаются только там, где он заменяет одну на две. Docstring остаётся внутри объявления.

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

```cpp
// MARK: Read input
// --------------------------------------------------
/**
 * @brief Читает входные данные.
 * @return входные данные
 */
int read();
```

**После**:

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

### SB-F-G-BL-10 — Без пустых строк между MARK и его разделителем

`Stylebrella-formatter-general-blank_lines-10`

Если между MARK и его разделителем есть только пустые строки, все они удаляются. Отсутствующий разделитель вставляется по [SB-F-G-M-3](#sb-f-g-m-3).

**Python**

**До**:

```python
# MARK: Build response

# --------------------------------------------------
```

**После**:

```python
# MARK: Build response
# --------------------------------------------------
```

**Kotlin**

**До**:

```kotlin
// MARK: Build response

// --------------------------------------------------
```

**После**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**C++**

**До**:

```cpp
// MARK: Build response

// --------------------------------------------------
```

**После**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

## `layout`

<a id="stylebrella-formatter-general-layout-1"></a>

<a id="sb-f-g-ly-1"></a>

### SB-F-G-LY-1 — Два пробела на уровень блока

`Stylebrella-formatter-general-layout-1`

Каждый уровень синтаксического блока добавляет ровно два пробела ASCII. Пробелы внутри литералов не меняются.

**Python**

**До**:

```python
if ready:
    send()
```

**После**:

```python
if ready:
  send()
```

**Kotlin**

**До**:

```kotlin
if (ready) {
    send()
    log()
}
```

**После**:

```kotlin
if (ready) {
  send()
  log()
}
```

**C++**

**До**:

```cpp
if (ready) {
    send();
    log();
}
```

**После**:

```cpp
if (ready) {
  send();
  log();
}
```

<a id="stylebrella-formatter-general-layout-2"></a>

<a id="sb-f-g-ly-2"></a>

### SB-F-G-LY-2 — Перенос длинных вызовов и сигнатур

`Stylebrella-formatter-general-layout-2`

Для вызовов и сигнатур предел переноса — 100 столбцов. Если однострочная форма длиннее, используется висячий отступ в два дополнительных пробела по одному элементу на строку с оформлением закрывающего разделителя по языку. Неделимый токен не разрывается даже при превышении лимита.

**Python**

**До**:

```python
run_request_processing_pipeline(project_configuration, request_processing_settings, transaction_logging_context)
```

**После**:

```python
run_request_processing_pipeline(
  project_configuration,
  request_processing_settings,
  transaction_logging_context,
)
```

**Kotlin**

**До**:

```kotlin
run_request_processing_pipeline(project_configuration, request_processing_settings, transaction_logging_context)
```

**После**:

```kotlin
run_request_processing_pipeline(
  project_configuration,
  request_processing_settings,
  transaction_logging_context,
)
```

**C++**

**До**:

```cpp
run_request_processing_pipeline(project_configuration, request_processing_settings, transaction_logging_context);
```

**После**:

```cpp
run_request_processing_pipeline(
  project_configuration,
  request_processing_settings,
  transaction_logging_context);
```

<a id="stylebrella-formatter-general-layout-3"></a>

<a id="sb-f-g-ly-3"></a>

### SB-F-G-LY-3 — Свёртка допустимых блоков управления с одним оператором

`Stylebrella-formatter-general-layout-3`

В Kotlin и C++ удаляйте скобки блока `if` или `for`, если его тело состоит ровно из одного оператора, язык допускает тело без скобок, а итоговый оператор помещается в 100 колонок. Применяйте это правило рекурсивно к вложенным блокам `for`. Сохраняйте скобки, если они нужны синтаксису или области видимости, если в блоке есть комментарии или несколько операторов, либо удаление может изменить привязку управления. К Python правило неприменимо: блоки языка задаются отступами, а не скобками.

**Kotlin**

**До**:

```kotlin
if (ready) {
  for (item in items) {
    send(item)
  }
}
```

**После**:

```kotlin
if (ready) for (item in items) send(item)
```

**C++**

**До**:

```cpp
if (ready) {
  for (const auto& item : items) {
    send(item);
  }
}
```

**После**:

```cpp
if (ready) for (const auto& item : items) send(item);
```

**Python:** неприменимо: Python не использует скобки для обозначения блоков управления.

## `separators`

В этой секции правила используют токен `separators` в канонических ID и `SP` в кратких псевдонимах.

<a id="stylebrella-formatter-general-separators-1"></a>

<a id="sb-f-g-sp-1"></a>

### SB-F-G-SP-1 — Синтаксис и длина разделителя

`Stylebrella-formatter-general-separators-1`

Разделитель — отдельный комментарий: префикс языка (`#` или `//`), один пробел и ровно 50 дефисов ASCII. Отдельный комментарий из двух и более дефисов в позиции разделителя нормализуется; прочие комментарии сохраняются.

**Python**

**До**:

```python
# ----
```

**После**:

```python
# --------------------------------------------------
```

**Kotlin**

**До**:

```kotlin
// ----
```

**После**:

```kotlin
// --------------------------------------------------
```

**C++**

**До**:

```cpp
// ----
```

**После**:

```cpp
// --------------------------------------------------
```

<a id="stylebrella-formatter-general-separators-2"></a>

<a id="sb-f-g-sp-2"></a>

### SB-F-G-SP-2 — Разделители между функциями и перед первым методом

`Stylebrella-formatter-general-separators-2`

Разделитель вставляется между соседними функциями или методами одного уровня и перед первым методом класса. Он ставится перед прикреплённой документацией или аннотациями, а в C++ — после метки доступа, открывающей секцию метода. Уже имеющийся разделитель блока MARK удовлетворяет правилу. Внутри класса отделяйте его одной пустой строкой от соседнего содержимого, кроме строки MARK над ним.

**Python**

**До**:

```python
class C:
  def f(self) -> None:
    run()

  # (no private methods)
```

**После**:

```python
class C:

  # --------------------------------------------------

  def f(self) -> None:
    run()

  # (no private methods)
```

**Kotlin**

**До**:

```kotlin
class C {
  fun f() = Unit

  // (no private methods)
}
```

**После**:

```kotlin
class C {

  // --------------------------------------------------

  fun f() = Unit

  // (no private methods)
}
```

**C++**

**До**:

```cpp
class C {
 public:
  void f();

  // (no private methods)
};
```

**После**:

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

### SB-F-G-SP-3 — Разделитель после импортов и включений

`Stylebrella-formatter-general-separators-3`

Если за начальными импортами или включениями следует объявление верхнего уровня, на этой границе требуется ровно один разделитель. Он ставится перед первым объявлением и его прикреплённой документацией, декораторами или аннотациями. Если у объявления уже есть MARK, используется его разделитель, второй не вставляется. Пустые строки оформляются по [SB-F-G-BL-1](#stylebrella-formatter-general-blank_lines-1).

**Python**

**До**:

```python
import time


def run() -> float:
  return time.monotonic()
```

**После**:

```python
import time


# --------------------------------------------------


def run() -> float:
  return time.monotonic()
```

**Kotlin**

**До**:

```kotlin
import java.time.Instant


fun run() = Instant.now()
```

**После**:

```kotlin
import java.time.Instant


// --------------------------------------------------


fun run() = Instant.now()
```

**C++** (`Clock.cpp`)

**До**:

```cpp
#include "Clock.hpp"


void run();
```

**После**:

```cpp
#include "Clock.hpp"


// --------------------------------------------------


void run();
```

## `mark`

<a id="stylebrella-formatter-general-mark-3"></a>

<a id="sb-f-g-m-3"></a>

### SB-F-G-M-3 — Обязательный разделитель после MARK

`Stylebrella-formatter-general-mark-3`

После каждого отдельного MARK требуется разделитель с таким же отступом. Если следующая непустая строка не является его разделителем, форматтер вставляет разделитель на следующей строке после MARK. Если он уже есть, форматтер сохраняет его и удаляет промежуточные пустые строки по [SB-F-G-BL-10](#sb-f-g-bl-10). Текст заголовка и счётчик группы сохраняются.

**Python**

**До**:

```python
# MARK: Build response
```

**После**:

```python
# MARK: Build response
# --------------------------------------------------
```

**Kotlin**

**До**:

```kotlin
// MARK: Build response
```

**После**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**C++**

**До**:

```cpp
// MARK: Build response
```

**После**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

<a id="stylebrella-formatter-general-mark-5"></a>

<a id="sb-f-g-m-5"></a>

### SB-F-G-M-5 — Секция Private Helpers

`Stylebrella-formatter-general-mark-5`

В конкретном классе хотя бы с одним публичным и одним приватным методом перед первым приватным ставится один `MARK: Private Helpers` с разделителем. Дубли структурного блока и устаревшие `(no private methods)` удаляются. Видимость, порядок членов и все описательные MARK сохраняются даже при диагностике соседних MARK. Правило не применяется к интерфейсам и типам, подтверждённым как используемые только в роли DTO.

**Python**

**До**:

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

**После**:

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

**До**:

```kotlin
class Reader {
  // MARK: Read input
  // --------------------------------------------------

  fun read(value: String): String = _copy(value)

  fun write(value: String): Int = value.length

  private fun _copy(value: String): String = value
}
```

**После**:

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

**До**:

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

**После**:

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

### SB-F-G-M-6 — Заглушка для класса без приватных методов

`Stylebrella-formatter-general-mark-6`

В таком же классе с публичными методами, но без приватных, после последнего публичного метода оставляется ровно один отдельный комментарий `(no private methods)` с одной пустой строкой перед ним. Устаревшие структурные блоки заменяются, дубли заглушки удаляются.

**Python**

**До**:

```python
  def read(self) -> None:
    load()
```

**После**:

```python
  def read(self) -> None:
    load()

  # (no private methods)
```

**Kotlin**

**До**:

```kotlin
  fun read() = Unit
```

**После**:

```kotlin
  fun read() = Unit

  // (no private methods)
```

**C++**

**До**:

```cpp
  void read();
```

**После**:

```cpp
  void read();

  // (no private methods)
```

<a id="stylebrella-formatter-general-mark-7"></a>

<a id="sb-f-g-m-7"></a>

### SB-F-G-M-7 — Один пробел после `MARK:`

`Stylebrella-formatter-general-mark-7`

Если у MARK есть заголовок, между `MARK:` и первым непробельным символом заголовка ставится ровно один пробел ASCII. Отсутствующие пробелы, лишние пробелы и табуляции на этой границе исправляются. Для пустого MARK форматтер не придумывает заголовок.

**Python**

**До**:

```python
# MARK:Build response
# --------------------------------------------------
```

**После**:

```python
# MARK: Build response
# --------------------------------------------------
```

**Kotlin**

**До**:

```kotlin
// MARK:  Build response
// --------------------------------------------------
```

**После**:

```kotlin
// MARK: Build response
// --------------------------------------------------
```

**C++**

**До**:

```cpp
// MARK:Build response
// --------------------------------------------------
```

**После**:

```cpp
// MARK: Build response
// --------------------------------------------------
```

## `comments`

<a id="stylebrella-formatter-general-comments-1"></a>

<a id="sb-f-g-cm-1"></a>

### SB-F-G-CM-1 — Пробел после префикса однострочного комментария

`Stylebrella-formatter-general-comments-1`

Первый символ после `#` в однострочном комментарии Python или после `//` в Kotlin и C++ — пробел ASCII, если у комментария есть содержимое. Это относится к отдельным и концевым комментариям, включая MARK и разделители. Если пробела нет, форматтер вставляет его перед текстом или заменяет первый пробельный символ; остальной текст комментария сохраняется. Пустой комментарий остаётся без пробела. Специальные префиксы с собственным синтаксисом сохраняются, например shebang Python (`#!`) и строчная документация C++ (`///`, `//!`). Правило применяется только к токенам комментариев, а не к строкам или директивам препроцессора C++.

**Python**

**До**:

```python
#Load the saved settings
```

**После**:

```python
# Load the saved settings
```

**Kotlin**

**До**:

```kotlin
//Load the saved settings
```

**После**:

```kotlin
// Load the saved settings
```

**C++**

**До**:

```cpp
//Load the saved settings
```

**После**:

```cpp
// Load the saved settings
```

## Рекомендации (без диагностики)

Обозначайте нужную группировку явными пробелами или комментариями, если её нельзя определить по коду. Сокращайте имена аргументов, только если смысл остаётся понятным. Не разрывайте неделимые токены и не меняйте значения литералов ради ширины.
