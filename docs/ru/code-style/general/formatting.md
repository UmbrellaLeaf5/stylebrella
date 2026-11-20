# Общее форматирование

[English version](../../../en/code-style/general/formatting.md)

Нормативный контракт будущих `format`, `check` и `format --check` (обе проверки только для чтения и только форматирования). Правила описывают целевое поведение, а не статус реализации. Для каждого языка применяется один канонический форматтер, без цепочки альтернативных форматтеров. После Stylebrella и повторный проход Stylebrella, и отдельный проход форматтера языка с тем же пресетом Stylebrella должны оставлять файл побайтно неизменным. Несовместимость блокирует поддержку затронутого правила до пересмотра контракта или выбора форматтера.

[Python](../python/formatting.md) · [Kotlin](../kotlin/formatting.md) · [C++](../cpp/formatting.md)

[Общий линтинг](linting.md) · [Реестр ID](../README.md#id-registry)

Каждый пример изолирует указанное правило.

## `source`

<a id="stylebrella-formatter-general-source-1"></a>

<a id="sb-f-g-s-1"></a>

### SB-F-G-S-1 — Исходники UTF-8 с переводами строк LF

`Stylebrella-formatter-general-source-1`

Записывайте исходники в UTF-8 без BOM и нормализуйте переводы строк до LF. Удаляйте концевые пробелы и табуляции вне литералов, а каждый непустой файл завершайте ровно одним LF. Не изменяйте значения литералов при нормализации. Явно отключённый диапазон форматирования освобождён от правила, если языковое правило это указывает. В примерах `<BOM>`, `··`, `→`, `<CRLF>`, `<LF>` и `<EOF>` обозначают маркер порядка байтов, два концевых пробела, концевую табуляцию, переводы строк и немедленный конец файла.

**Python**

**До**:

```text
<BOM>value = "данные"··→<CRLF><EOF>
```

**После**:

```text
value = "данные"<LF><EOF>
```

**Kotlin**

**До**:

```text
<BOM>val value = "данные"··→<CRLF><EOF>
```

**После**:

```text
val value = "данные"<LF><EOF>
```

**C++**

**До**:

```text
<BOM>const char* value = "данные";··→<CRLF><EOF>
```

**После**:

```text
const char* value = "данные";<LF><EOF>
```

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

### SB-F-G-BL-4 — Пустые строки между группами локальных объявлений и операций с получателем

`Stylebrella-formatter-general-blank_lines-4`

В списке инструкций группируйте соседние локальные объявления, которые не начинают группы операций с получателем. Группируйте последовательные операции с одной и той же переменной даже без соседнего объявления, например если это параметр функции. Группа операций с получателем состоит из двух или более операций либо из локального объявления и одной или более следующих сразу за ним операций с этой переменной. К операциям относятся прямые вызовы методов переменной, присваивания её полям, повторные присваивания самой переменной и прямые вызовы, где эта переменная — единственный аргумент (например, `use(f)`). Вызов без получателя с дополнительными аргументами, например `use(f, g)`, или вызов через другой объект, например `g.func(f, a)`, не входит в группу для `f`. От соседней несвязанной инструкции, включая операцию с другой переменной, группа отделяется одной пустой строкой. Внутри группы пустых строк нет, кроме случаев, когда [SB-F-G-BL-6](#stylebrella-formatter-general-blank_lines-6) требует пустую строку после многострочной операции. Не переставляйте инструкции и не объединяйте операции через промежуточные инструкции или явные границы по комментариям. Для комментариев к объявлениям применяйте [SB-F-G-BL-5](#stylebrella-formatter-general-blank_lines-5).

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

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

**После**:

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

### SB-F-G-BL-5 — Пустые строки перед объявлениями с комментарием

`Stylebrella-formatter-general-blank_lines-5`

Соседние объявления полей класса и локальных переменных не разделяются, кроме случая, когда у следующего объявления есть предшествующий комментарий или аннотация: перед прикреплённым комментарием или аннотацией ставится одна пустая строка. Комментарий справа на строке объявления не разрывает группу. Для локальных переменных граница перед предшествующим комментарием или аннотацией имеет приоритет над группировкой по [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4).

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

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

**После**:

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

### SB-F-G-BL-6 — Пустая строка после многострочной инструкции

`Stylebrella-formatter-general-blank_lines-6`

Перед следующей инструкцией того же уровня ставится одна пустая строка, если предыдущая инструкция остаётся многострочной после форматирования. Не добавляйте её лишь потому, что во входном файле инструкция разбита на строки, хотя форматтер свёл бы её к одной. Вызовы длиннее лимита должны оставаться перенесёнными согласно [SB-F-G-LY-2](#stylebrella-formatter-general-layout-2). Перед закрывающим разделителем пустая строка не добавляется. Для границ управляющих конструкций, охваченных [SB-F-G-BL-7](#stylebrella-formatter-general-blank_lines-7), применяйте это правило. Внутри непрерывной группы объявлений действует [SB-F-G-BL-4](#stylebrella-formatter-general-blank_lines-4).

**Python**

**До**:

```python
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )
  g()
```

**После**:

```python
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )

  g()
```

**Kotlin**

**До**:

```kotlin
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )
  g()
```

**После**:

```kotlin
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context,
  )

  g()
```

**C++**

**До**:

```cpp
  run_request_processing_pipeline(
    project_configuration,
    request_processing_settings,
    transaction_logging_context
  );
  g();
```

**После**:

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

### SB-F-G-BL-7 — Пустые строки вокруг управляющих конструкций и между ветвями

`Stylebrella-formatter-general-blank_lines-7`

Отделяйте `if`, `for`, `try` и инструкции выбора (`match` в Python, `when` в Kotlin, `switch` в C++) от соседних инструкций того же уровня одной пустой строкой, в том числе после тела из одной инструкции без скобок. Ставьте одну пустую строку между последовательными ветвями `if` и между частями `try` (`except`/`catch`, `else` и `finally`, где они доступны), если после языкового форматирования следующая ветвь начинается на отдельной строке; не переносите её только ради пустой строки. Каждую ветвь выбора (`case`, ветвь `when` или `default`) начинайте на отдельной строке и отделяйте от предыдущей одной пустой строкой. Последовательные метки C++ с общим телом не разделяйте пустыми строками. Не добавляйте пустую строку перед первым или после последнего элемента тела либо между заголовком и телом конструкции.

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

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

**После**:

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

### SB-F-G-BL-8 — Пустая строка перед return или yield

`Stylebrella-formatter-general-blank_lines-8`

Ставьте ровно одну пустую строку перед `return` или самостоятельной инструкцией выдачи значения, если ей предшествует инструкция того же уровня. Это относится к `yield` и `yield from` в Python, а также к `co_return` и самостоятельному `co_yield` в C++. В Kotlin правило действует перед ключевым словом `return` независимо от возвращаемого выражения, в том числе в `return try`, `return when` и `return if`. Не добавляйте пустую строку перед первой инструкцией блока, внутри однострочного управляющего тела или перед `yield` внутри другого выражения.

**Python**

**До**:

```python
  send()
  return result
```

```python
  record(item)
  yield item
  yield from remaining_items
```

**После**:

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

**До**:

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

**После**:

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

**До**:

```cpp
  send();
  return result;
```

```cpp
  record(item);
  co_yield item;
  co_return;
```

**После**:

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

Если между MARK и его разделителем есть только пустые строки, все они удаляются. Отсутствующий разделитель вставляется по [SB-F-G-M-1](#sb-f-g-m-1).

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

<a id="stylebrella-formatter-general-blank_lines-11"></a>

<a id="sb-f-g-bl-11"></a>

### SB-F-G-BL-11 — Пустая строка перед ведущими комментариями

`Stylebrella-formatter-general-blank_lines-11`

В списке инструкций перед непрерывной группой отдельных комментариев, непосредственно предшествующей инструкции или объявлению, ставится одна пустая строка, если перед группой есть соседняя инструкция. Это относится к одиночным строчным комментариям, нескольким строчным комментариям подряд и блочным комментариям, в том числе многострочным `/* ... */` в Kotlin и C++. Сохраняйте интервалы внутри блочных комментариев и оставляйте группу прикреплённой к следующей инструкции или объявлению. В начале файла или списка инструкций предшествующая пустая строка не добавляется. Правило не применяется к комментариям справа от кода или внутри заголовков, списков параметров и других вложенных конструкций. Если на этой границе более конкретное правило задаёт интервал, применяется указанное им число строк, без сложения с этим правилом.

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

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

**После**:

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

Для вызовов и сигнатур предел переноса — 100 столбцов. Вызов ровно с одним аргументом всегда сводится к одной строке, если полностью оформленный вызов помещается в лимит, независимо от переносов во входном коде; завершающая запятая удаляется, если язык её допускает. Иначе, если однострочная форма длиннее лимита, используется висячий отступ в два дополнительных пробела по одному элементу на строку с оформлением закрывающего разделителя по языку. В C++ закрывающие `);` перенесённого вызова ставятся на отдельной строке с отступом начала вызова; завершающая запятая не добавляется. Неделимый токен не разрывается даже при превышении лимита.

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

**До — один аргумент**:

```python
f(
  x,
)
```

**После**:

```python
f(x)
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

**До — один аргумент**:

```kotlin
f(
  x,
)
```

**После**:

```kotlin
f(x)
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
  transaction_logging_context
);
```

**До — один аргумент**:

```cpp
f(
  x
);
```

**После**:

```cpp
f(x);
```

<a id="stylebrella-formatter-general-layout-3"></a>

<a id="sb-f-g-ly-3"></a>

### SB-F-G-LY-3 — Свёртка допустимых блоков управления с одним оператором

`Stylebrella-formatter-general-layout-3`

В Kotlin и C++ удаляйте скобки `if` или `for`, если тело состоит ровно из одной инструкции, язык допускает тело без скобок, а каждая строка результата укладывается в 100 колонок. Оставляйте одиночную инструкцию, которая сама не является управляющей конструкцией, на строке заголовка, если она помещается и не входит в цепочку вложенных конструкций без скобок. В такой цепочке ставьте каждый заголовок на отдельную строку, а его тело — на следующую с дополнительным отступом в два пробела, включая самое внутреннее тело. Сохраняйте скобки, если они нужны синтаксису или области видимости, если в блоке есть комментарии или несколько инструкций либо удаление может изменить привязку управления. К Python правило неприменимо: блоки языка задаются отступами, а не скобками.

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
if (ready)
  for (item in items)
    send(item)
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
if (ready)
  for (const auto& item : items)
    send(item);
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

<a id="stylebrella-formatter-general-mark-1"></a>

<a id="sb-f-g-m-1"></a>

### SB-F-G-M-1 — Обязательный разделитель после MARK

`Stylebrella-formatter-general-mark-1`

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

<a id="stylebrella-formatter-general-mark-2"></a>

<a id="sb-f-g-m-2"></a>

### SB-F-G-M-2 — Секция Private Helpers

`Stylebrella-formatter-general-mark-2`

В конкретном классе хотя бы с одним приватным методом, даже без публичных методов, перед первым приватным должен быть ровно один блок `MARK: Private Helpers`. Команда `format` вставляет или переносит блок с разделителем на это место и удаляет дубли структурного блока и устаревшие комментарии `(no private methods)`. Команда `check` сообщает о тех же необходимых изменениях без записи. Метки доступа, порядок членов и все описательные MARK сохраняются даже при диагностике соседних MARK. Правило не применяется к интерфейсам и подтверждённым DTO.

Если публичный метод идёт после приватного, пропускайте все правки секции Private Helpers и заглушки в этом классе и продолжайте форматирование. О нарушении порядка сообщает `lint` по [SB-L-G-M-7](linting.md#sb-l-g-m-7). Определяйте видимость до правок по правилам объявлений языка; в Python сохраняйте классификацию методов, определённую существующей приватной секцией. Для класса с публичными методами, но без приватных применяется [SB-F-G-CM-4](#sb-f-g-cm-4).

В каждом примере ниже, если в классе остаётся только приватный `_copy`, применяется та же правка: добавьте заголовок перед существующим разделителем. Если же `_copy` стоит раньше `read`, оставьте структурные маркеры класса как есть. Если приватных методов нет, см. примеры с заглушкой в [SB-F-G-CM-4](#sb-f-g-cm-4).

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

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

**После**:

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

### SB-F-G-M-3 — Один пробел после `MARK:`

`Stylebrella-formatter-general-mark-3`

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

<a id="stylebrella-formatter-general-mark-4"></a>

<a id="sb-f-g-m-4"></a>

### SB-F-G-M-4 — Удаление суффикса одиночной группы

`Stylebrella-formatter-general-mark-4`

Если заголовок MARK заканчивается точным суффиксом ` [group:1]`, удаляйте суффикс вместе с предшествующим пробелом. Остальной текст заголовка и структуру MARK сохраняйте. Счётчики групп от 2 и выше сохраняются.

**Python**

**До**:

```python
# MARK: Load adapters [group:1]
# --------------------------------------------------
```

**После**:

```python
# MARK: Load adapters
# --------------------------------------------------
```

**Kotlin**

**До**:

```kotlin
// MARK: Load adapters [group:1]
// --------------------------------------------------
```

**После**:

```kotlin
// MARK: Load adapters
// --------------------------------------------------
```

**C++**

**До**:

```cpp
// MARK: Load adapters [group:1]
// --------------------------------------------------
```

**После**:

```cpp
// MARK: Load adapters
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
#load the saved settings
```

**После**:

```python
# load the saved settings
```

**Kotlin**

**До**:

```kotlin
//load the saved settings
```

**После**:

```kotlin
// load the saved settings
```

**C++**

**До**:

```cpp
//load the saved settings
```

**После**:

```cpp
// load the saved settings
```

<a id="stylebrella-formatter-general-comments-2"></a>

<a id="sb-f-g-cm-2"></a>

### SB-F-G-CM-2 — Строчная буква в начале однострочного комментария

`Stylebrella-formatter-general-comments-2`

Если первый непробельный символ обычного однострочного комментария — прописная буква, заменяйте её строчной, в том числе вне ASCII. Это касается отдельных комментариев и комментариев после кода с `#`, `//` и обычных `/* ... */`, занимающих одну строку. Остальной текст сохраняйте; не пропускайте знаки препинания, цифры или разделители кода в поисках буквы. Сохраняйте структурные комментарии: MARK, разделители, shebang, документацию и директивы инструментов.

**Python**

**До**:

```python
# Load the saved settings
```

**После**:

```python
# load the saved settings
```

**Kotlin**

**До**:

```kotlin
  val retries: Int, // Максимум повторов
```

**После**:

```kotlin
  val retries: Int, // максимум повторов
```

**C++**

**До**:

```cpp
  /* Skip the cached value. */
  refresh();
```

**После**:

```cpp
  /* skip the cached value. */
  refresh();
```

<a id="stylebrella-formatter-general-comments-3"></a>

<a id="sb-f-g-cm-3"></a>

### SB-F-G-CM-3 — Перенос концевых комментариев объявлений переменных

`Stylebrella-formatter-general-comments-3`

Для обычного прозаического концевого комментария `#` или `//` у объявления переменной или поля переносите текст по границам слов, чтобы по возможности каждая строка укладывалась в 100 столбцов. Сохраняйте все слова и знаки препинания; пробелы между словами нормализуйте до одного ASCII-пробела. Если комментарий продолжается, ставьте маркер продолжения в той же колонке, что и исходный маркер, и оставляйте после него один пробел. Неделимые слова не разрывайте. Не переносите отдельные комментарии, комментарии у других инструкций, документацию, MARK, разделители, директивы инструментов, а также комментарии с выравниванием или форматированным кодом. Также применяйте языковые правила префиксов и регистра.

**Python**

**До**:

```python
value = 10  # key feature of this configuration and its required behavior for every request, including retries and recovery
```

**После**:

```python
value = 10  # key feature of this configuration and its required behavior for every request,
            # including retries and recovery
```

**Kotlin**

**До**:

```kotlin
val value = 10  // key feature of this configuration and its required behavior for every request, including retries and recovery
```

**После**:

```kotlin
val value = 10  // key feature of this configuration and its required behavior for every request,
                // including retries and recovery
```

**C++**

**До**:

```cpp
int value; // key feature of this configuration and its required behavior for every request, including retries and recovery
```

**После**:

```cpp
int value; // key feature of this configuration and its required behavior for every request,
           // including retries and recovery
```

<a id="stylebrella-formatter-general-comments-4"></a>

<a id="sb-f-g-cm-4"></a>

### SB-F-G-CM-4 — Заглушка для класса без приватных методов

`Stylebrella-formatter-general-comments-4`

В конкретном классе с публичными методами, но без приватных, после последнего публичного метода должен быть ровно один отдельный комментарий `(no private methods)` с одной пустой строкой перед ним. Команда `format` заменяет устаревшие блоки `Private Helpers`, вставляет или переносит заглушку и удаляет дубли; `check` сообщает о тех же необходимых изменениях без записи. В классе без публичных и приватных методов заглушка не нужна. Если приватные методы есть, применяйте [SB-F-G-M-2](#sb-f-g-m-2), в том числе его условие о порядке методов. Интерфейсы и подтверждённые DTO освобождены от правила.

**Python**

**До**:

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

**После**:

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

**До**:

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

**После**:

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

**До**:

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

**После**:

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

## Рекомендации (без диагностики)

Обозначайте нужную группировку явными пробелами или комментариями, если её нельзя определить по коду. Сокращайте имена аргументов, только если смысл остаётся понятным. Не разрывайте неделимые токены и не меняйте значения литералов ради ширины.
