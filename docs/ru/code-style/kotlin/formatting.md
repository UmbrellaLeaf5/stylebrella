# Форматирование Kotlin

[English version](../../../en/code-style/kotlin/formatting.md)

Область: `.kt` и `.kts`, включая скрипты Gradle. Для результата по контракту пресету не нужны специфичные настройки IDE. Видимость методов задана в [линтинге Kotlin](linting.md).

[Общие правила](../formatting.md)

Каждый пример изолирует указанное правило.

## `source`

<a id="stylebrella-formatter-kotlin-source-1"></a>

<a id="sb-f-k-s-1"></a>

### SB-F-K-S-1 — Кодировка, переводы строк и концевые пробелы

`Stylebrella-formatter-kotlin-source-1`

Файл записывается в UTF-8 с CRLF, концевые пробелы и табуляции вне литералов удаляются, непустой файл завершается переводом строки.

**До** (пробелы, затем конец файла):

```kotlin
val n = 1
```

**После** (CRLF, затем конец файла):

```kotlin
val n = 1
```

<a id="stylebrella-formatter-kotlin-source-2"></a>

<a id="sb-f-k-s-2"></a>

### SB-F-K-S-2 — Участки с отключённым форматированием

`Stylebrella-formatter-kotlin-source-2`

Байты от отдельной строки `// @formatter:off` до следующей отдельной `// @formatter:on` включительно, либо до конца файла, не меняются. На этот диапазон не распространяется и нормализация пробелов файла.

**До**:

```kotlin
// @formatter:off
val n=1
// @formatter:on
```

**После**:

```kotlin
// @formatter:off
val n=1
// @formatter:on
```

## `layout`

<a id="stylebrella-formatter-kotlin-layout-1"></a>

<a id="sb-f-k-ly-1"></a>

### SB-F-K-LY-1 — Перенос аргументов и параметров

`Stylebrella-formatter-kotlin-layout-1`

При переносе вызова или сигнатуры на каждой строке размещается один аргумент или параметр; закрывающая скобка стоит отдельно на отступе начала конструкции. Завершающая запятая ставится там, где разрешена Kotlin.

**До**:

```kotlin
call(
  x, y)
```

**После**:

```kotlin
call(
  x,
  y,
)
```

<a id="stylebrella-formatter-kotlin-layout-2"></a>

<a id="sb-f-k-ly-2"></a>

### SB-F-K-LY-2 — Пробелы после запятых и вокруг операторов

`Stylebrella-formatter-kotlin-layout-2`

После запятой перед следующим токеном той же строки и с обеих сторон бинарных арифметических операторов, операторов равенства и сравнения ставится один пробел. Унарные операторы и скобки параметров типов не считаются бинарными операторами.

**До**:

```kotlin
f(x,y+1==z)
```

**После**:

```kotlin
f(x, y + 1 == z)
```

## `imports`

<a id="stylebrella-formatter-kotlin-imports-1"></a>

<a id="sb-f-k-i-1"></a>

### SB-F-K-I-1 — Порядок групп импортов

`Stylebrella-formatter-kotlin-imports-1`

Ведущие импорты сортируются лексикографически внутри групп: обычные, `java`, `javax`, `kotlin`, затем псевдонимы. Прикреплённые комментарии перемещаются вместе с импортом. Поздние импорты не переносятся.

**До**:

```kotlin
import kotlin.math.PI
import app.run
```

**После**:

```kotlin
import app.run
import kotlin.math.PI
```

<a id="stylebrella-formatter-kotlin-imports-2"></a>

<a id="sb-f-k-i-2"></a>

### SB-F-K-I-2 — Пороги звёздочного импорта

`Stylebrella-formatter-kotlin-imports-2`

Импорты не менее пяти разных имён без псевдонима из пакета или трёх разных членов без псевдонима из типа заменяются одним звёздочным импортом, если сохраняются привязки всех разрешённых ссылок и не появляется неоднозначность. Псевдонимы в пороги не входят.

**До**:

```kotlin
import app.Tools.a
import app.Tools.b
import app.Tools.c
```

**После**:

```kotlin
import app.Tools.*
```

## `expressions`

<a id="stylebrella-formatter-kotlin-expressions-1"></a>

<a id="sb-f-k-e-1"></a>

### SB-F-K-E-1 — Тела-выражения

`Stylebrella-formatter-kotlin-expressions-1`

Блок из одного `return expression` без метки заменяется `= expression`, если нет контракта Kotlin и сохраняется поведение. Явный тип результата опускается только при совпадении выведенного типа, сохранении публичной и переопределяемой сигнатуры и семантики `Unit`. Ранние, помеченные или неразрешённые возвраты не меняются.

**До**:

```kotlin
fun count(): Int { return items.size }
```

**После**:

```kotlin
fun count() = items.size
```

## `blank_lines`

<a id="stylebrella-formatter-kotlin-blank_lines-1"></a>

<a id="sb-f-k-bl-1"></a>

### SB-F-K-BL-1 — Пустые строки между ветвями when

`Stylebrella-formatter-kotlin-blank_lines-1`

Применяйте [SB-F-G-BL-7](../formatting.md#stylebrella-formatter-general-blank_lines-7) между всеми соседними ветвями `when`, даже если тело каждой из них — выражение.

**До**:

```kotlin
  A -> { f() }
  B -> g()
```

**После**:

```kotlin
  A -> { f() }

  B -> g()
```

<a id="stylebrella-formatter-kotlin-blank_lines-2"></a>

<a id="sb-f-k-bl-2"></a>

### SB-F-K-BL-2 — Пустые строки у категорий конструктора и объявлений с аннотациями

`Stylebrella-formatter-kotlin-blank_lines-2`

Между категориями параметров конструктора ставится одна пустая строка перед следующим комментарием категории; она же ставится перед группой объявления с прикреплённым комментарием или аннотацией при наличии предыдущего объявления того же уровня.

**До**:

```kotlin
val a = 1
@JvmField
val b = 2
```

**После**:

```kotlin
val a = 1

@JvmField
val b = 2
```

<a id="stylebrella-formatter-kotlin-blank_lines-3"></a>

<a id="sb-f-k-bl-3"></a>

### SB-F-K-BL-3 — Перенос перед else, catch и finally

`Stylebrella-formatter-kotlin-blank_lines-3`

`else if`, `else`, `catch` и `finally` начинаются с новой строки. К границе ветвей применяется [SB-F-G-BL-7](../formatting.md#stylebrella-formatter-general-blank_lines-7). Скобки только ради форматирования не добавляются.

**До**:

```kotlin
if (x) {
  f()
  g()
} else {
  h()
  i()
}
```

**После**:

```kotlin
if (x) {
  f()
  g()
}

else {
  h()
  i()
}
```

## Рекомендации (без диагностики)

Оставляйте блок, если тело-выражение скроет управляющий поток. Не полагайтесь на порядок импортов.
