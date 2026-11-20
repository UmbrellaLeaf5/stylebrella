# Стиль кода

[English version](../../en/code-style/README.md)

Эти спецификации описывают ожидаемое поведение форматтера и линтера Stylebrella. Начните с общих правил в `general/`, затем обратитесь к правилам для конкретного языка: Python, Kotlin или C++.

| Область       | Форматирование                          | Линтинг                       |
| ------------- | --------------------------------------- | ----------------------------- |
| Общие правила | [Форматирование](general/formatting.md) | [Линтинг](general/linting.md) |
| Python        | [Форматирование](python/formatting.md)  | [Линтинг](python/linting.md)  |
| Kotlin        | [Форматирование](kotlin/formatting.md)  | [Линтинг](kotlin/linting.md)  |
| C++           | [Форматирование](cpp/formatting.md)     | [Линтинг](cpp/linting.md)     |

<a id="id-registry"></a>

## Реестр ID

Канонические ID имеют вид `Stylebrella-<вид>-<язык>-<подраздел>-<номер>`. Равнозначные короткие псевдонимы используют `SB-<вид>-<язык>-<подраздел>-<номер>` и токены ниже.

Для каждого вида и языка нумеруйте правила внутри подраздела подряд с 1 в порядке документа. При перенумерации обновляйте оба ID, их якоря и все ссылки в обеих языковых версиях.

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
