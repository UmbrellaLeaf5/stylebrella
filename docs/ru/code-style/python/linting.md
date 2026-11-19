# Линтинг Python

[English version](../../../en/code-style/python/linting.md)

Область: `.py`; ноутбуки вне области. Как правила Stylebrella включены только проверки, прямо определённые контрактом. Даже одно оставшееся нарушение означает провал линтинга; порядок импортов проверяет форматтер. Для MARK имена ровно с одним начальным подчёркиванием считаются приватными; специальные методы вроде `__init__` не относятся к приватным вспомогательным.

[Общие правила](../linting.md)

Каждый пример изолирует указанное правило.

## `naming`

<a id="stylebrella-linter-python-naming-1"></a>

<a id="sb-l-p-n-1"></a>

### SB-L-P-N-1 — Имена объявлений Python

`Stylebrella-linter-python-naming-1`

Имена функций, методов, переменных, параметров и полей — snake_case; классов, dataclass и исключений — PascalCase; констант — SCREAMING_SNAKE_CASE. Для разрешённой переменной типа требуется `T` или PascalCase с префиксом `T`. Исключение: специальные имена Python.

**Допустимо**:

```python
def load_config() -> Config:
  return read_config()
```

**Нарушение**:

```python
def loadConfig() -> Config:
  return read_config()
```

<a id="stylebrella-linter-python-naming-2"></a>

<a id="sb-l-p-n-2"></a>

### SB-L-P-N-2 — Начальное подчёркивание внутренних символов

`Stylebrella-linter-python-naming-2`

У символов, объявленных внутренними по метаданным или явному соглашению о видимости, требуется одно начальное подчёркивание. Отсутствие использования или имени в несвязанном списке экспорта не считается доказательством.

**Допустимо** (внутренний тип):

```python
class _InternalType:
  pass
```

**Нарушение** (внутренний тип):

```python
class InternalType:
  pass
```

## `json`

<a id="stylebrella-linter-python-json-1"></a>

<a id="sb-l-p-j-1"></a>

### SB-L-P-J-1 — Единый стиль имён ключей JSON

`Stylebrella-linter-python-json-1`

К статически разрешённым JSON-ключам всего проекта применяется одно настроенное соглашение: snake_case или camelCase. Соглашение не выводится из отдельного ключа.

**Допустимо** (проект использует `snake_case`):

```python
"user_id"
```

**Нарушение** (проект использует `snake_case`):

```python
"User-ID"
```

## `types`

<a id="stylebrella-linter-python-types-1"></a>

<a id="sb-l-p-t-1"></a>

### SB-L-P-T-1 — Аннотации параметров и результата

`Stylebrella-linter-python-types-1`

Каждый параметр и результат функции должен иметь аннотацию; исключение — фактический первый параметр-получатель `self` или `cls`. Результат без значения обозначается `-> None`. Диагностика на параметре или имени функции.

**Допустимо**:

```python
def clear(self, cache: Cache) -> None:
  cache.clear()
```

**Нарушение**:

```python
def clear(self, cache) -> None:
  cache.clear()
```

<a id="stylebrella-linter-python-types-2"></a>

<a id="sb-l-p-t-2"></a>

### SB-L-P-T-2 — Объединение с None вместо Optional

`Stylebrella-linter-python-types-2`

Для целей Python 3.10 и новее диагностика выдаётся для аннотаций, разрешённых в `typing.Optional[T]`; применяется `T | None`.

**Допустимо**:

```python
def find() -> User | None:
  return lookup_user()
```

**Нарушение**:

```python
def find() -> Optional[User]:
  return lookup_user()
```

<a id="stylebrella-linter-python-types-3"></a>

<a id="sb-l-p-t-3"></a>

### SB-L-P-T-3 — Запрет явного typing.Any

`Stylebrella-linter-python-types-3`

Диагностируются явные ссылки в аннотациях, разрешённые в `typing.Any`. Несвязанный символ с именем `Any` не подходит.

**Допустимо**:

```python
def load(data: object) -> Config:
  return parse_config(data)
```

**Нарушение**:

```python
def load(data: Any) -> Config:
  return parse_config(data)
```

<a id="stylebrella-linter-python-types-4"></a>

<a id="sb-l-p-t-4"></a>

### SB-L-P-T-4 — Тип Path для путей файловой системы

`Stylebrella-linter-python-types-4`

Если аргумент или результат определён как путь по явным метаданным или разрешённым API работы с путями, аннотация `str` диагностируется; требуется `pathlib.Path`. По одному имени путь не определяется.

**Допустимо**:

```python
def load(path: Path) -> Config:
  return parse_config(path)
```

**Нарушение** (подтверждённый путь):

```python
def load(path: str) -> Config:
  return parse_config(path)
```

<a id="stylebrella-linter-python-types-5"></a>

<a id="sb-l-p-t-5"></a>

### SB-L-P-T-5 — Блокирующие ошибки типов

`Stylebrella-linter-python-types-5`

Любая ошибка типов в Python из области контракта в режиме basic является блокирующей диагностикой. Неразрешённые ссылки проверяются только в пределах семантики этого режима.

**Допустимо**:

```python
value: int = len(items)
```

**Нарушение**:

```python
value: int = "wrong"
```

## `idioms`

<a id="stylebrella-linter-python-idioms-1"></a>

<a id="sb-l-p-id-1"></a>

### SB-L-P-ID-1 — Интерполяция строк и отложенное форматирование логов

`Stylebrella-linter-python-idioms-1`

Интерполяция через строковый `%` или разрешённый `str.format` заменяется требованием f-строки; арифметический остаток исключён. В разрешённом логировании с интерполяцией требуется отложенный `%`-шаблон и отдельные аргументы вместо f-строки или заранее форматированной строки.

**Допустимо**:

```python
logger.debug("Loaded %r", config)
```

**Нарушение**:

```python
logger.debug(f"Loaded {config!r}")
```

<a id="stylebrella-linter-python-idioms-2"></a>

<a id="sb-l-p-id-2"></a>

### SB-L-P-ID-2 — Контекстные менеджеры для локальных ресурсов

`Stylebrella-linter-python-idioms-2`

Диагностируется разрешённый `open` или `Path.open`, когда файл используется локально, не передан в `with`, не покидает область и не передан другому установленному менеджеру ресурсов. То же условие применяется к распознанным API соединений, блокировок и пользовательских ресурсов с поддержкой контекстных менеджеров.

**Допустимо**:

```python
with path.open() as stream: consume(stream)
```

**Нарушение**:

```python
stream = path.open(); stream.read()
```

<a id="stylebrella-linter-python-idioms-3"></a>

<a id="sb-l-p-id-3"></a>

### SB-L-P-ID-3 — Предел вложенности включений

`Stylebrella-linter-python-idioms-3`

Разрешено не более двух уровней генераторов включения. Считаются охватывающие секции генераторов на пути и секции самого вложенного включения; диагностика на секции, превысившей лимит.

**Допустимо**:

```python
[x for row in rows for x in row]
```

**Нарушение**:

```python
[x for table in tables for row in table for x in row]
```

<a id="stylebrella-linter-python-idioms-4"></a>

<a id="sb-l-p-id-4"></a>

### SB-L-P-ID-4 — Pathlib вместо os.path

`Stylebrella-linter-python-idioms-4`

Диагностируются разрешённые вызовы функций `os.path` для операций с путём; применяется `pathlib.Path`.

**Допустимо**:

```python
path.exists()
```

**Нарушение**:

```python
os.path.exists(path)
```

<a id="stylebrella-linter-python-idioms-5"></a>

<a id="sb-l-p-id-5"></a>

### SB-L-P-ID-5 — Заголовок исполняемого скрипта и защита точки входа

`Stylebrella-linter-python-idioms-5`

Файл, отмеченный в метаданных как исполняемый скрипт проекта с `.venv` или `uv`, начинается с `#!/usr/bin/env python3`. Если он также импортируется, вызов точки входа помещается в `if __name__ == "__main__":`.

**Допустимо**:

```python
#!/usr/bin/env python3
if __name__ == "__main__":
  main()
```

**Нарушение** (скрипт запускается и импортируется):

```python
main()
```

<a id="stylebrella-linter-python-idioms-6"></a>

<a id="sb-l-p-id-6"></a>

### SB-L-P-ID-6 — Логирование и сообщения консоли в ASCII

`Stylebrella-linter-python-idioms-6`

В файлах, явно отнесённых к рабочему коду, диагностируется вызов встроенной `print`. В разрешённом статическом тексте сообщения для консоли допустим только ASCII; язык текста проверяется отдельно.

**Допустимо**:

```python
logger.info("Loaded")
```

**Нарушение** (рабочий код):

```python
print("Loaded")
```

## `imports`

<a id="stylebrella-linter-python-imports-1"></a>

<a id="sb-l-p-i-1"></a>

### SB-L-P-I-1 — Звёздочные импорты и явный экспорт пакета

`Stylebrella-linter-python-imports-1`

Звёздочные импорты запрещены, кроме явных реэкспортов в `__init__.py` со статически разрешёнными экспортами источника и публичной областью, ограниченной явным `__all__`. Экспорты пакета объявляются через `__all__`, а не через избыточные псевдонимы или директивы подавления для реэкспорта.

**Допустимо**:

```python
from .service import UserService
__all__ = ["UserService"]
```

**Нарушение** (вне `__init__.py`):

```python
from .service import *
```

<a id="stylebrella-linter-python-imports-2"></a>

<a id="sb-l-p-i-2"></a>

### SB-L-P-I-2 — Циклические импорты

`Stylebrella-linter-python-imports-2`

Диагностируются разрешённые статически циклы графа безусловных импортов проекта. Динамические и неразрешённые импорты не образуют рёбер. Для разрыва цикла поздний импорт автоматически не добавляется.

**Допустимо**:

```python
# a.py
from b import y

# b.py
y = load()
```

**Нарушение**:

```python
# a.py
from b import y

# b.py
from a import x
```

## `source`

<a id="stylebrella-linter-python-source-1"></a>

<a id="sb-l-p-s-1"></a>

### SB-L-P-S-1 — Неутверждённые подавления диагностики

`Stylebrella-linter-python-source-1`

Диагностируются `# noqa` и исключения правил или файлов, явно не разрешённые конфигурацией проекта. Утверждённое исключение действует только на зарегистрированное правило и область.

**Допустимо**:

```python
value = read()
```

**Нарушение**:

```python
value = read()  # noqa
```

## `documentation`

<a id="stylebrella-linter-python-documentation-1"></a>

<a id="sb-l-p-d-1"></a>

### SB-L-P-D-1 — Когда функции нужен docstring

`Stylebrella-linter-python-documentation-1`

Многострочный docstring первой инструкцией обязателен для каждой публичной нетривиальной, важной или отмеченной описательным MARK функции, а также при более чем 15 исполняемых строках тела или более чем 3 объявленных параметрах, не считая `self` и `cls`. `*args` и `**kwargs` считаются по одному. Строки и важность считаются по [SB-L-G-M-2](../linting.md#stylebrella-linter-general-mark-2). Ровно 15 строк или 3 параметра сами по себе правило не включают.

**Допустимо**:

```python
def f(a: int, b: int, c: int, d: int) -> None:
  """
  Передаёт входные значения обработчику.

  Args:
    a: Первое входное значение.
    b: Второе входное значение.
    c: Третье входное значение.
    d: Четвёртое входное значение.
  """

  consume(a, b, c, d)
```

**Нарушение**:

```python
def f(a: int, b: int, c: int, d: int) -> None:
  consume(a, b, c, d)
```

<a id="stylebrella-linter-python-documentation-2"></a>

<a id="sb-l-p-d-2"></a>

### SB-L-P-D-2 — Структура docstring в стиле Google

`Stylebrella-linter-python-documentation-2`

Тройные двойные кавычки ставятся на отдельных строках. Первый текст — непустое краткое описание. Между абзацами и разделами одна пустая строка; применимые разделы Google `Args:`, `Returns:`, `Raises:`, `Attributes:` имеют отступ элементов два пробела. Имена параметров и атрибутов совпадают с объявлением и идут по порядку; получатель и повтор типа в метке не указываются, описания элементов непусты. `Returns:` нужен для установленного результата не `None`, `Raises:` — для известных выходящих исключений. Неприменимые разделы опускаются.

**Допустимо**:

```python
"""
  Returns the count.

  Returns:
    Number of rows.
  """
```

**Нарушение**:

```python
"""Returns the count."""
```

## `comments`

<a id="stylebrella-linter-python-comments-1"></a>

<a id="sb-l-p-cm-1"></a>

### SB-L-P-CM-1 — Комментарии к полям dataclass

`Stylebrella-linter-python-comments-1`

[SB-L-G-CM-1](../linting.md#stylebrella-linter-general-comments-1) применяется к полям экземпляра dataclass, кроме точных имён `id`, `user_id`, `item_id`.

**Допустимо**:

```python
df_hz: float = 0.0  # Смещение, Гц
```

**Нарушение**:

```python
df_hz: float = 0.0
```

<a id="stylebrella-linter-python-comments-2"></a>

<a id="sb-l-p-cm-2"></a>

### SB-L-P-CM-2 — Категории параметров конструктора

`Stylebrella-linter-python-comments-2`

Разрешённые типы параметров конструктора с суффиксами `Repository`, `Mapper`, `Service` размещаются под соответствующими `# repositories:`, `# mappers:`, `# services:` либо под явно настроенной категорией. Диагностика на известном параметре вне категории; неизвестные не диагностируются.

**Допустимо**:

```python
def __init__(
  self,
  # services:
  service: OrderService,
) -> None:
  self._service = service
```

**Нарушение**:

```python
def __init__(
  self,
  # mappers:
  service: OrderService,
) -> None:
  self._service = service
```

## `constants`

<a id="stylebrella-linter-python-constants-1"></a>

<a id="sb-l-p-c-1"></a>

### SB-L-P-C-1 — Общие константы и исключения для метаданных

`Stylebrella-linter-python-constants-1`

Общие константы размещаются в `constants.py` как `Constants.<Category>.<FIELD>` (например, Math, Entity, Validation, Pattern или предметная категория). Диагностируется именованное значение, на которое ссылаются два или более модуля вне контейнера, либо неканоническая ссылка на разрешённую общую константу. `__main__` в точной проверке точки входа, имена в `__all__` и имена параметров/фикстур pytest освобождены от [SB-L-G-C-1](../linting.md#stylebrella-linter-general-constants-1).

**Допустимо**:

```python
Constants.Math.DEG_TO_RAD
```

**Нарушение** (константа уровня модуля используется в двух модулях):

```python
DEG_TO_RAD
```

## `testing`

<a id="stylebrella-linter-python-testing-1"></a>

<a id="sb-l-p-ts-1"></a>

### SB-L-P-TS-1 — Имена файлов тестов, pytest и общие фикстуры

`Stylebrella-linter-python-testing-1`

Файл, классифицированный как тест модуля, называется `test_<module>.py`; тесты используют pytest, а не `unittest.TestCase`. Фикстуры, используемые в нескольких файлах, размещаются в ближайшем общем `conftest.py`.

**Допустимо** (`test_parser.py`):

```python
def test_parse() -> None:
  assert parse(DEFAULT_INPUT) == EXPECTED_RESULT
```

**Нарушение** (`parser_test.py`):

```python
class ParserTest(unittest.TestCase): ...
```

<a id="stylebrella-linter-python-testing-2"></a>

<a id="sb-l-p-ts-2"></a>

### SB-L-P-TS-2 — Тесты, зависящие от окружения, и тестовые данные

`Stylebrella-linter-python-testing-2`

Тесты, явно классифицированные как зависящие от окружения, помечаются разрешёнными pytest `skip` или `skipif` на тесте, его классе или модуле. К литеральным входным данным тестов и параметризации применяется [SB-L-G-C-1](../linting.md#stylebrella-linter-general-constants-1).

**Допустимо**:

```python
@pytest.mark.skipif(not has_service(), reason="Service unavailable")
def test_service() -> None:
  assert service.is_available()
```

**Нарушение** (тест зависит от окружения):

```python
def test_service() -> None:
  assert service.is_available()
```

## Рекомендации (без диагностики)

Используйте `repr()` или `!r`, когда важно точное представление. Для ленивого обхода большого набора используйте генератор. Применяйте dataclass для записей, frozen dataclass для неизменяемых данных, проверяемые модели для конфигурации, перечисления для закрытых наборов и Protocol для структурных интерфейсов. Продумывайте область фикстур и параметризуйте повторяющиеся случаи; для подмен на уровне модуля предпочитайте `pytest.MonkeyPatch`, на уровне экземпляра — моки экземпляра. Выбирайте уровень логирования по смыслу. Предпочитайте русский текст docstring и описывайте исключения, выходящие к вызывающему коду. Разрывайте циклы изменением структуры, а не переносом импорта в функцию.
