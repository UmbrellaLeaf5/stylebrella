# Форматирование C++

[English version](../../../en/code-style/cpp/formatting.md)

Область: `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, `.hh`; не CMake и не отдельные файлы данных или конфигурации. Применимая конфигурация определяется по реальному пути исходника, включая stdin. Видимость членов задана в [линтинге C++](linting.md).

[Общие правила](../formatting.md)

Каждый пример изолирует указанное правило.

## `layout`

<a id="stylebrella-formatter-cpp-layout-1"></a>

<a id="sb-f-c-ly-1"></a>

### SB-F-C-LY-1 — Базовое оформление и отступ продолжения

`Stylebrella-formatter-cpp-layout-1`

Базовая раскладка — Google C++ с исключениями этой страницы и общего контракта. Продолжение добавляет два пробела к отступу строки конструкции.

**До**:

```cpp
void process(
    const Request& incoming_request,
    const Configuration& processing_configuration,
    ResultCollector& result_collector);
```

**После**:

```cpp
void process(
  const Request& incoming_request,
  const Configuration& processing_configuration,
  ResultCollector& result_collector);
```

<a id="stylebrella-formatter-cpp-layout-2"></a>

<a id="sb-f-c-ly-2"></a>

### SB-F-C-LY-2 — Выравнивание массивов структур

`Stylebrella-formatter-cpp-layout-2`

В многострочном инициализаторе массива структур с одинаковым числом элементов во всех строках столбцы выравниваются по левому краю.

**До**:

```cpp
Pair pairs[] = {
  {1, 200},
  {20, 3},
};
```

**После**:

```cpp
Pair pairs[] = {
  {1,  200},
  {20, 3  },
};
```

<a id="stylebrella-formatter-cpp-layout-3"></a>

<a id="sb-f-c-ly-3"></a>

### SB-F-C-LY-3 — Перенос бинарных операторов

`Stylebrella-formatter-cpp-layout-3`

При переносе бинарного выражения оператор ставится в начало продолжения, кроме операторов присваивания, остающихся на предыдущей строке.

**До**:

```cpp
const int total = left +
  right;
```

**После**:

```cpp
const int total = left
  + right;
```

<a id="stylebrella-formatter-cpp-layout-4"></a>

<a id="sb-f-c-ly-4"></a>

### SB-F-C-LY-4 — Перенос списков инициализации конструктора

`Stylebrella-formatter-cpp-layout-4`

При переносе списка инициализации конструктора строка разрывается перед двоеточием.

**До**:

```cpp
C::C() :
  x_(x) {}
```

**После**:

```cpp
C::C()
  : x_(x) {}
```

<a id="stylebrella-formatter-cpp-layout-5"></a>

<a id="sb-f-c-ly-5"></a>

### SB-F-C-LY-5 — Сохранение строк комментариев

`Stylebrella-formatter-cpp-layout-5`

Текст комментариев не переносится. Концевой комментарий поля остаётся на строке объявления даже при превышении целевой ширины.

**До**:

```cpp
int count;  // Number of bytes in the payload before compression.
```

**После**:

```cpp
int count;  // Number of bytes in the payload before compression.
```

<a id="stylebrella-formatter-cpp-layout-6"></a>

<a id="sb-f-c-ly-6"></a>

### SB-F-C-LY-6 — Короткие функции и управляющие тела без скобок

`Stylebrella-formatter-cpp-layout-6`

Встроенная функция-член или короткий `if` без скобок и без `else` могут занимать одну строку, если помещаются. Короткие тела `if` или `for` без скобок не оборачиваются в них; структура управления не меняется.

**До**:

```cpp
if (ready)
  send();
```

**После**:

```cpp
if (ready) send();
```

## `imports`

<a id="stylebrella-formatter-cpp-imports-1"></a>

<a id="sb-f-c-i-1"></a>

### SB-F-C-I-1 — Категории и сортировка включений

`Stylebrella-formatter-cpp-imports-1`

Смежные ведущие включения перегруппировываются и сортируются по категории, затем по имени заголовка с учётом регистра: соответствующий основной заголовок (в том числе при суффиксах исходника `-test`, `_test`, `-unittest`, `_unittest`), заголовки `.h` в угловых скобках кроме `ext/`, остальные в угловых скобках, затем в кавычках. Между непустыми категориями одна пустая строка. Отдельные группирующие комментарии и границы ветвей препроцессора разрывают сортируемый блок; поздние включения не перемещаются.

**До**:

```cpp
#include "other.hpp"
#include <vector>
#include <stdio.h>
```

**После**:

```cpp
#include <stdio.h>

#include <vector>

#include "other.hpp"
```

## `blank_lines`

<a id="stylebrella-formatter-cpp-blank_lines-1"></a>

<a id="sb-f-c-bl-1"></a>

### SB-F-C-BL-1 — Пустые строки в пространствах имён и группы присваиваний полям

`Stylebrella-formatter-cpp-blank_lines-1`

Между блоками объявлений в пространстве имён ставится одна пустая строка, включая границы разделителей; Doxygen остаётся с объявлением. Последовательные присваивания разным полям только что объявленного объекта остаются в его группе.

**До**:

```cpp
namespace n {
void a();
// --------------------------------------------------
void b();
}  // namespace n  // namespace n
```

**После**:

```cpp
namespace n {
void a();

// --------------------------------------------------

void b();
}  // namespace n  // namespace n
```

<a id="stylebrella-formatter-cpp-blank_lines-2"></a>

<a id="sb-f-c-bl-2"></a>

### SB-F-C-BL-2 — Перенос перед else и catch

`Stylebrella-formatter-cpp-blank_lines-2`

`else if`, `else` и `catch` начинаются с новой строки; к границе ветвей применяется [SB-F-G-BL-7](../formatting.md#stylebrella-formatter-general-blank_lines-7).

**До**:

```cpp
if (x) {
  f();
} else {
  g();
}
```

**После**:

```cpp
if (x) {
  f();
}

else {
  g();
}
```

## Рекомендации (без диагностики)

**Совместимость:** исходная конфигурация на основе Google сохраняет не более одной пустой строки подряд. Пресету Stylebrella нужна настройка `MaxEmptyLinesToKeep: 2`, чтобы сохранить отступы верхнего уровня по [SB-F-G-BL-1](../formatting.md#stylebrella-formatter-general-blank_lines-1) и [SB-F-G-BL-2](../formatting.md#stylebrella-formatter-general-blank_lines-2). Одна эта настройка не подтверждает совместимость со всеми правилами.

Отмечайте намеренно разные группы включений отдельными комментариями. Не полагайтесь на порядок включений для неявного подключения объявлений другого заголовка.

```cpp
// Plotting
#include "plot/Canvas.hpp"

// Application state
#include "state/Session.hpp"
```
