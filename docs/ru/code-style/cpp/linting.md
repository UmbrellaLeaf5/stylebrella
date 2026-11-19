# Линтинг C++

[English version](../../../en/code-style/cpp/linting.md)

Область: `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, `.hh`; CMake исключён. Для семантических правил используется база компиляции проекта. Без неё сообщайте о пропуске семантических проверок и продолжайте независимые файловые и синтаксические проверки. Видимость берётся из умолчаний `class`/`struct` и меток доступа; protected-методы не считаются приватными.

[Общие правила](../linting.md)

Каждый пример изолирует указанное правило.

## `naming`

<a id="stylebrella-linter-cpp-naming-1"></a>

<a id="sb-l-c-n-1"></a>

### SB-L-C-N-1 — Имена объявлений C++

`Stylebrella-linter-cpp-naming-1`

Для классов, структур и перечислений требуется PascalCase, для функций, переменных, полей и параметров — snake_case, для пространств имён — строчные буквы. У приватных полей одно начальное подчёркивание, если имя не зарезервировано C++. Имена приватных методов задаёт [SB-L-G-N-2](../linting.md#sb-l-g-n-2); у protected-членов приватного префикса нет.

**Допустимо** (приватное поле):

```cpp
int _gain_max_dbi;
```

**Нарушение** (приватное поле):

```cpp
int gain_max_dbi;
```

<a id="stylebrella-linter-cpp-naming-2"></a>

<a id="sb-l-c-n-2"></a>

### SB-L-C-N-2 — Имена значений const и constexpr

`Stylebrella-linter-cpp-naming-2`

Константа, объявленная `const` или `constexpr`, в том числе локальная, называется в SCREAMING_SNAKE_CASE. Приватная константа-член сохраняет подчёркивание, если имя допустимо; для зарезервированного результата используется имя без префикса. Cv-квалификатор метода, const-объект по указателю или ссылке и параметр только для чтения не считаются объявлением константы.

**Допустимо**:

```cpp
constexpr int MAX_RETRIES = 3;
```

**Нарушение**:

```cpp
constexpr int maxRetries = 3;
```

## `json`

<a id="stylebrella-linter-cpp-json-1"></a>

<a id="sb-l-c-j-1"></a>

### SB-L-C-J-1 — Ключи JSON в snake_case

`Stylebrella-linter-cpp-json-1`

Разрешённый ключ JSON в C++ пишется в snake_case. Строки без установленной роли JSON не проверяются.

**Допустимо**:

```cpp
record["center_freq_hz"]
```

**Нарушение**:

```cpp
record["centerFreqHz"]
```

## `files`

<a id="stylebrella-linter-cpp-files-1"></a>

<a id="sb-l-c-fi-1"></a>

### SB-L-C-FI-1 — Расширения заголовков и pragma once

`Stylebrella-linter-cpp-files-1`

Заголовок проекта имеет расширение `.hpp`, если явно не отмечен как устаревший или сторонний. В преамбуле обязателен `#pragma once`, макрос защиты всего файла запрещён.

**Допустимо** (`Antenna.hpp`):

```cpp
#pragma once

class Antenna;
```

**Нарушение** (`Antenna.h`):

```cpp
#ifndef ANTENNA_H
#define ANTENNA_H

class Antenna;

#endif
```

<a id="stylebrella-linter-cpp-files-2"></a>

<a id="sb-l-c-fi-2"></a>

### SB-L-C-FI-2 — Имена файлов, основные типы и структура исходников

`Stylebrella-linter-cpp-files-2`

Файл типа содержит один основной именованный класс, структуру или перечисление; имя без расширения совпадает с ним и пишется в PascalCase. Файл вспомогательных функций имеет имя в snake_case. Каталог разрешённого пространства имён следует его пути относительно настроенного корня; объявления и реализации проекта находятся в `include/` и `src/` соответственно, а inline- и шаблонные определения допустимы в заголовках.

**Допустимо** (`include/app/core/Antenna.hpp`):

```cpp
namespace core {
class Antenna;
}
```

**Нарушение** (`include/app/math/antenna.hpp`):

```cpp
namespace core {
class Antenna;
}
```

<a id="stylebrella-linter-cpp-files-3"></a>

<a id="sb-l-c-fi-3"></a>

### SB-L-C-FI-3 — Общие типы и типы компонентов

`Stylebrella-linter-cpp-files-3`

При известном полном графе ссылок и хотя бы одном использующем классе тип размещается в `components/`, если класс один, и в `shared/`, если их два или больше. По неполному графу правило не применяется.

**Допустимо** (`shared/Point.hpp`; используется двумя классами):

```cpp
class Point {};
```

**Нарушение** (`components/Point.hpp`; используется двумя классами):

```cpp
class Point {};
```

## `types`

<a id="stylebrella-linter-cpp-types-1"></a>

<a id="sb-l-c-t-1"></a>

### SB-L-C-T-1 — Запрет приведений в стиле C

`Stylebrella-linter-cpp-types-1`

Приведения в стиле C запрещены. Диагностика на приведении, не на обычных группирующих скобках. Именованные приведения допустимы при корректных типах.

**Допустимо**:

```cpp
static_cast<double>(count)
```

**Нарушение**:

```cpp
(double)count
```

<a id="stylebrella-linter-cpp-types-2"></a>

<a id="sb-l-c-t-2"></a>

### SB-L-C-T-2 — Nullptr для нулевых указателей

`Stylebrella-linter-cpp-types-2`

В подтверждённом контексте указателя вместо `NULL` или целого `0` требуется `nullptr`.

**Допустимо**:

```cpp
Widget* ptr = nullptr;
```

**Нарушение**:

```cpp
Widget* ptr = 0;
```

<a id="stylebrella-linter-cpp-types-3"></a>

<a id="sb-l-c-t-3"></a>

### SB-L-C-T-3 — Когда использовать auto

`Stylebrella-linter-cpp-types-3`

Для разрешённого типа итератора, замыкания лямбды или безымянного результата требуется `auto`; для непосредственного литерала фундаментального типа `auto` не допускается. Прочие случаи выбора вывода или явного типа проверяются отдельно.

**Допустимо**:

```cpp
int count = 5;
```

**Нарушение**:

```cpp
auto count = 5;
```

<a id="stylebrella-linter-cpp-types-4"></a>

<a id="sb-l-c-t-4"></a>

### SB-L-C-T-4 — Const и constexpr для локальных значений

`Stylebrella-linter-cpp-types-4`

Локальная инициализированная переменная должна быть `const`, если установлено отсутствие записи, изменяемого доступа или выхода из области и добавление сохраняет корректность и выбор перегрузок. Для константы требуется `constexpr`, если инициализация вычисляется при компиляции и такое объявление сохраняет контракт. Диагностируются только установленные случаи.

**Допустимо**:

```cpp
constexpr int MAX_RETRIES = 3;
```

**Нарушение**:

```cpp
const int MAX_RETRIES = 3;
```

<a id="stylebrella-linter-cpp-types-5"></a>

<a id="sb-l-c-t-5"></a>

### SB-L-C-T-5 — Const-методы и явный override

`Stylebrella-linter-cpp-types-5`

Метод экземпляра должен быть `const`, если полное определение доказуемо не меняет объект и квалификация сохраняет требования к сигнатуре и выбор перегрузки. Для фактического переопределения `override` обязателен в объявлении внутри класса. Отсутствие подклассов не вызывает диагностику `final`.

**Допустимо**:

```cpp
Result read() const override;
```

**Нарушение** (подтверждённое переопределение):

```cpp
Result read() const;
```

## `idioms`

<a id="stylebrella-linter-cpp-idioms-1"></a>

<a id="sb-l-c-id-1"></a>

### SB-L-C-ID-1 — Избыточные отдельные блоки

`Stylebrella-linter-cpp-idioms-1`

Диагностируется отдельный блок только для группировки, если он не создаёт необходимой границы видимости или времени жизни. Тела управляющих конструкций, функций, пространств имён, классов и инициализаторы исключаются. При неизвестном влиянии на видимость или время жизни диагностики нет.

**Допустимо**:

```cpp
log();
flush();
```

**Нарушение**:

```cpp
{ log(); flush(); }
```

## `documentation`

<a id="stylebrella-linter-cpp-documentation-1"></a>

<a id="sb-l-c-d-1"></a>

### SB-L-C-D-1 — Когда объявлениям и функциям нужен Doxygen

`Stylebrella-linter-cpp-documentation-1`

Непосредственно над объявлениями функций, классов, структур и перечислений в `.hpp` обязателен многострочный Doxygen. Для любой функции из области правил с более чем 15 исполняемыми строками тела или более чем 3 объявленными параметрами функции нужен Doxygen над объявлением или определением; достаточно корректного блока объявления. Пакет параметров считается один раз, строки — по [SB-L-G-M-2](../linting.md#stylebrella-linter-general-mark-2). Для первого важного приватного метода действует [SB-L-G-M-5](../linting.md#stylebrella-linter-general-mark-5).

**Допустимо**:

```cpp
/**
 * @brief Обрабатывает данные.
 * @param a первое значение
 * @param b второе значение
 * @param c третье значение
 * @param d четвёртое значение
 */
void f(int a, int b, int c, int d) {}
```

**Нарушение**:

```cpp
void f(int a, int b, int c, int d) {}
```

<a id="stylebrella-linter-cpp-documentation-2"></a>

<a id="sb-l-c-d-2"></a>

### SB-L-C-D-2 — Структура, теги и язык Doxygen

`Stylebrella-linter-cpp-documentation-2`

Doxygen открывается `/**` и закрывается `*/` на отдельных строках; у каждой внутренней строки, в том числе пустой, есть звёздочка. Первый тег — непустой `@brief`; `@param name description` идут в порядке сигнатуры, `@return` требуется для результата не `void`, `@throws Type description` — для известных документируемых исключений. Неприменимые теги, двоеточие после имени параметра и помета `Type:` результата опускаются. Проверка кириллицы и латиницы как в [SB-L-K-D-2](../kotlin/linting.md#stylebrella-linter-kotlin-documentation-2); `///` и однострочный блок не заменяют многострочный.

**Допустимо**:

```cpp
/**
 * @brief Читает настройки.
 * @return Прочитанные настройки.
 */
Settings read();
```

**Нарушение**:

```cpp
/// Reads settings.
Settings read();
```

## `comments`

<a id="stylebrella-linter-cpp-comments-1"></a>

<a id="sb-l-c-cm-1"></a>

### SB-L-C-CM-1 — Комментарии к полям структур

`Stylebrella-linter-cpp-comments-1`

[SB-L-G-CM-1](../linting.md#stylebrella-linter-general-comments-1) применяется к полям структур, кроме точных имён `id` и `name`. MARK и разделители применяются и в заголовках, и в файлах реализации.

**Допустимо**:

```cpp
double df_hz; // Смещение, Гц
```

**Нарушение**:

```cpp
double df_hz;
```

## `constants`

<a id="stylebrella-linter-cpp-constants-1"></a>

<a id="sb-l-c-c-1"></a>

### SB-L-C-C-1 — Размещение и квалификация общих констант

`Stylebrella-linter-cpp-constants-1`

Именованная константа, используемая в двух или более исходных файлах, размещается в настроенном заголовке и пространстве имён констант проекта, в категории вроде `math`, `entity`, `config`, `validation`, `pattern`. Вне этого пространства для разрешённых общих констант требуется полностью квалифицированная ссылка.

**Допустимо**:

```cpp
app::constants::config::CENTER_FREQ_HZ
```

**Нарушение** (неквалифицированная ссылка вне пространства имён):

```cpp
CENTER_FREQ_HZ
```

<a id="stylebrella-linter-cpp-constants-2"></a>

<a id="sb-l-c-c-2"></a>

### SB-L-C-C-2 — Встроенные литералы и исключения для сообщений

`Stylebrella-linter-cpp-constants-2`

В местах использования общих чисел, шаблонов, путей и значений по умолчанию применяется [SB-L-G-C-1](../linting.md#stylebrella-linter-general-constants-1). Текст исключения, лога или вывода освобождается только в позиции сообщения по [SB-L-G-C-2](../linting.md#stylebrella-linter-general-constants-2).

**Допустимо**:

```cpp
connect(app::constants::config::PORT);
```

**Нарушение**:

```cpp
connect(8080);
```

## `exceptions`

<a id="stylebrella-linter-cpp-exceptions-1"></a>

<a id="sb-l-c-ex-1"></a>

### SB-L-C-EX-1 — Структура сообщения исключения

`Stylebrella-linter-cpp-exceptions-1`

Статически известное сообщение стандартного исключения имеет форму `[Source] Message: key=value, key=value`. Source — охватывающий `Class::method` или `namespace::function`; подтверждённая ошибка пути, поля или перечисления требует ключ `path`, `field` или `type` соответственно. Известная структура и ключи проверяются и при вычисляемом значении. Текст не генерируется.

**Допустимо**:

```cpp
throw std::runtime_error("[Loader::load] Cannot read file: path=" + path.string());
```

**Нарушение** (подтверждённая ошибка работы с файлом):

```cpp
throw std::runtime_error("Cannot read file");
```

<a id="stylebrella-linter-cpp-exceptions-2"></a>

<a id="sb-l-c-ex-2"></a>

### SB-L-C-EX-2 — Тип исключения по категории сбоя

`Stylebrella-linter-cpp-exceptions-2`

Для подтверждённой категории ошибки требуется `std::invalid_argument` при неверном входе или значении перечисления, `std::runtime_error` при ошибке ввода-вывода или окружения, `std::logic_error` при неверном внутреннем состоянии или порядке вызовов API. Неизвестная категория не вызывает диагностику типа.

**Допустимо** (неверные входные данные):

```cpp
throw std::invalid_argument(message);
```

**Нарушение** (неверные входные данные):

```cpp
throw std::runtime_error(message);
```

<a id="stylebrella-linter-cpp-exceptions-3"></a>

<a id="sb-l-c-ex-3"></a>

### SB-L-C-EX-3 — Сообщения консоли в ASCII

`Stylebrella-linter-cpp-exceptions-3`

В статически известном тексте разрешённого сообщения консоли допускается только ASCII. Язык произвольных строк и содержимое динамических значений не определяются.

**Допустимо**:

```cpp
std::cout << "Loaded";
```

**Нарушение**:

```cpp
std::cout << "Загружено";
```

## Рекомендации (без диагностики)

Используйте `std::unique_ptr` для исключительного владения, `std::shared_ptr` только для совместного, `std::weak_ptr` — для разрыва циклов. Перед удалением отдельного блока проверьте область видимости и время жизни. Используйте `final`, когда наследование намеренно запрещено. Сложные алгоритмы и устройство типа поясняйте в `@details`. Сообщения для консоли пишите на английском. Если категория ошибки не установлена, проверяйте тип исключения на ревью.
