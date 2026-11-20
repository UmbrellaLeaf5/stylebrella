# Форматирование C++

[English version](../../../en/code-style/cpp/formatting.md)

Область: `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, `.hh`; не CMake и не отдельные файлы данных или конфигурации. Применимая конфигурация определяется по реальному пути исходника, включая stdin. Видимость членов задана в [линтинге C++](linting.md).

[Общие правила](../general/formatting.md)

Каждый пример изолирует указанное правило.

## `layout`

<a id="stylebrella-formatter-cpp-layout-1"></a>

<a id="sb-f-c-ly-1"></a>

### SB-F-C-LY-1 — Базовая раскладка Google и положение скобок

`Stylebrella-formatter-cpp-layout-1`

Базовая раскладка — Google C++ с исключениями этой страницы и общего контракта. Открывающие фигурные скобки определений функций остаются в строке объявления. Отступ продолжения задаётся общими правилами оформления.

**До**:

```cpp
void process()
{
  prepare();
  execute();
}
```

**После**:

```cpp
void process() {
  prepare();
  execute();
}
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

### SB-F-C-LY-5 — Оформление встроенных методов

`Stylebrella-formatter-cpp-layout-5`

Сворачивайте тело встроенного метода класса с одной инструкцией в одну строку, если оно помещается в 100 столбцов. Оформление тел `if` и `for` без скобок задаётся общим правилом.

**До**:

```cpp
class Sender {
 public:
  void send() const {
    notify();
  }
};
```

**После**:

```cpp
class Sender {
 public:
  void send() const { notify(); }
};
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

Ставьте ровно одну пустую строку сразу после открывающей `{` и перед закрывающей `}` каждого определения пространства имён, включая анонимные и вложенные. В пустом пространстве имён одна пустая строка служит обеим границам. Закрывающую скобку ставьте на отдельную строку с комментарием в стиле clang-format: `}  // namespace` для анонимного пространства, `}  // namespace name` для именованного и `}  // namespace app::detail` для сокращённой записи вложенного пространства `namespace app::detail`. Добавляйте или исправляйте комментарий даже у коротких и пустых пространств имён.

Между блоками объявлений в пространстве имён ставится одна пустая строка, включая границы разделителей; Doxygen остаётся с объявлением. Последовательные присваивания разным полям только что объявленного объекта остаются в его группе.

**До**:

```cpp
namespace n {
void a();
// --------------------------------------------------
void b();
}  // namespace wrong
```

```cpp
namespace {
void a();
}
```

```cpp
namespace app::detail {
void a();
}
```

```cpp
namespace outer {
namespace inner {
void a();
}
}
```

```cpp
namespace empty {
}
```

**После**:

```cpp
namespace n {

void a();

// --------------------------------------------------

void b();

}  // namespace n
```

```cpp
namespace {

void a();

}  // namespace
```

```cpp
namespace app::detail {

void a();

}  // namespace app::detail
```

```cpp
namespace outer {

namespace inner {

void a();

}  // namespace inner

}  // namespace outer
```

```cpp
namespace empty {

}  // namespace empty
```

<a id="stylebrella-formatter-cpp-blank_lines-2"></a>

<a id="sb-f-c-bl-2"></a>

### SB-F-C-BL-2 — Перенос перед else и catch

`Stylebrella-formatter-cpp-blank_lines-2`

`else if`, `else` и `catch` начинаются с новой строки; к границе ветвей применяется [SB-F-G-BL-7](../general/formatting.md#stylebrella-formatter-general-blank_lines-7).

**До**:

```cpp
if (x) {
  f();
  g();
} else {
  h();
  i();
}
```

**После**:

```cpp
if (x) {
  f();
  g();
}

else {
  h();
  i();
}
```

## Рекомендации (без диагностики)

**Совместимость:** исходная конфигурация на основе Google сохраняет не более одной пустой строки подряд. Пресету Stylebrella нужна настройка `MaxEmptyLinesToKeep: 2`, чтобы сохранить отступы верхнего уровня по [SB-F-G-BL-1](../general/formatting.md#stylebrella-formatter-general-blank_lines-1) и [SB-F-G-BL-2](../general/formatting.md#stylebrella-formatter-general-blank_lines-2). Одна эта настройка не подтверждает совместимость со всеми правилами.

Отмечайте намеренно разные группы включений отдельными комментариями. Не полагайтесь на порядок включений для неявного подключения объявлений другого заголовка.

```cpp
// plotting
#include "plot/Canvas.hpp"

// application state
#include "state/Session.hpp"
```
