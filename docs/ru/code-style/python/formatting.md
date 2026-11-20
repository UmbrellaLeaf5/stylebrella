# Форматирование Python

[English version](../../../en/code-style/python/formatting.md)

Область: `.py`. Ноутбуки и отдельные файлы конфигурации вне контракта. Видимость методов задана в [линтинге Python](linting.md).

[Общие правила](../formatting.md)

Каждый пример изолирует указанное правило.

## `source`

<a id="stylebrella-formatter-python-source-1"></a>

<a id="sb-f-p-s-1"></a>

### SB-F-P-S-1 — Переводы строк LF

`Stylebrella-formatter-python-source-1`

Переводы строк файла нормализуются в LF без изменения содержимого литералов.

**До** (CRLF):

```python
x = 1
```

**После** (LF):

```python
x = 1
```

<a id="stylebrella-formatter-python-source-2"></a>

<a id="sb-f-p-s-2"></a>

### SB-F-P-S-2 — Двойные кавычки для строк

`Stylebrella-formatter-python-source-2`

Строковые литералы оформляются двойными кавычками, если значение сохраняется и не появляются дополнительные экранирования.

**До**:

```python
x = 'hello'
```

**После**:

```python
x = "hello"
```

## `imports`

<a id="stylebrella-formatter-python-imports-1"></a>

<a id="sb-f-p-i-1"></a>

### SB-F-P-I-1 — Группировка и сортировка импортов

`Stylebrella-formatter-python-imports-1`

В начальном блоке порядок групп: импорты из `__future__`, стандартная библиотека, сторонние пакеты, локальные модули. Между имеющимися группами одна пустая строка. Внутри группы импорты сортируются по модулю и импортируемому имени, прикреплённые комментарии перемещаются вместе с ними. Побочные эффекты не отменяют сортировку. Импорты из других мест в блок не переносятся.

**До**:

```python
import requests
import sys
```

**После**:

```python
import sys

import requests
```

## `blank_lines`

<a id="stylebrella-formatter-python-blank_lines-1"></a>

<a id="sb-f-p-bl-1"></a>

### SB-F-P-BL-1 — Пустая строка после docstring

`Stylebrella-formatter-python-blank_lines-1`

Между docstring функции или класса и следующей инструкцией тела ставится одна пустая строка; если инструкция единственная, строка не добавляется.

**До**:

```python
def _read() -> None:
  """
  Читает данные.
  """
  read()
```

**После**:

```python
def _read() -> None:
  """
  Читает данные.
  """

  read()
```

<a id="stylebrella-formatter-python-blank_lines-2"></a>

<a id="sb-f-p-bl-2"></a>

### SB-F-P-BL-2 — Пустые строки перед инструкциями и заголовками ветвей

`Stylebrella-formatter-python-blank_lines-2`

Ставьте одну пустую строку перед соседней инструкцией `while`, `with`, `raise`, `assert`, `continue` и перед заголовком `else` цикла после его тела. Для интервалов вокруг `if`, `for`, `try` и `match` и между их ветвями применяйте [SB-F-G-BL-7](../formatting.md#stylebrella-formatter-general-blank_lines-7). Не разбивайте однострочное тело ради пустой строки.

**До**:

```python
  log()
  assert ready
```

**После**:

```python
  log()

  assert ready
```

<a id="stylebrella-formatter-python-blank_lines-3"></a>

<a id="sb-f-p-bl-3"></a>

### SB-F-P-BL-3 — Пустые строки возле комментариев категорий конструктора

`Stylebrella-formatter-python-blank_lines-3`

Комментарии категорий параметров конструктора сохраняются в списке параметров, пустые строки непосредственно перед ними и после них удаляются.

**До**:

```python
def __init__(
  self,

  # services:
  service: Service,
): ...
```

**После**:

```python
def __init__(
  self,
  # services:
  service: Service,
): ...
```

## `mark`

<a id="stylebrella-formatter-python-mark-1"></a>

<a id="sb-f-p-m-1"></a>

### SB-F-P-M-1 — Пробел в концевых комментариях

`Stylebrella-formatter-python-mark-1`

Концевой комментарий остаётся на строке своего кода. Пробел после `#` оформляется по [SB-F-G-CM-1](../formatting.md#sb-f-g-cm-1). Директивы интерпретатора, объявления кодировки и текст служебных меток сохраняются.

**До**:

```python
x = load()  #note
```

**После**:

```python
x = load()  # note
```

## Рекомендации (без диагностики)

Не делайте побочные эффекты импорта зависимыми от порядка. Вместо обхода правил размещения вынесите логику в отдельный модуль.
