# Линтинг Python

[English version](../../../en/code-style/python/linting.md)

Область: `.py`; ноутбуки вне области. Как правила Stylebrella включены только проверки, прямо определённые контрактом. Даже одно оставшееся нарушение означает провал линтинга; порядок импортов проверяет форматтер. Для MARK имена ровно с одним начальным подчёркиванием считаются приватными; специальные методы вроде `__init__` не относятся к приватным вспомогательным.

[Общие правила](../general/linting.md)

Каждый пример изолирует указанное правило. Примеры функций являются фрагментами; если правило не касается документации, пропущенный в примере обязательный docstring считается присутствующим в полном теле функции.

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

### SB-L-P-J-1 — Ключи JSON в snake_case

`Stylebrella-linter-python-json-1`

Для каждого статически разрешённого JSON-ключа требуется `snake_case`. JSON-ключи считаются метаданными схемы и освобождены от правила встроенных литералов; посторонние строки без установленной роли JSON не диагностируются.

**Допустимо** (`record` — разрешённый JSON-объект):

```python
record["user_id"]
```

**Нарушение** (`record` — разрешённый JSON-объект):

```python
record["userId"]
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

Файл, отмеченный в метаданных как исполняемый скрипт проекта с `.venv` или `uv`, начинается с `#!/usr/bin/env python3`. В любом исполняемом скрипте, даже из одной инструкции и независимо от возможности импорта, на уровне модуля объявляется `def main() -> None:` без параметров и вызывается через `if __name__ == "__main__":` на уровне модуля. Внутри условия допустим только вызов `main()`; ветки `else` нет. Правило точки входа применяется к файлам, распознанным как скрипты по метаданным, Python shebang или уже имеющемуся условию входа. Сообщение при отсутствии или неверной точке входа: «Объявите `main()` и вызовите её под `if __name__ == "__main__":`, даже в простейшем скрипте».

**Допустимо**:

```python
#!/usr/bin/env python3

def main() -> None:
  """
  Обновляет локальный кэш перед запуском приложения.
  """

  refresh_cache()


if __name__ == "__main__":
  main()
```

**Нарушение** (исполняемый скрипт: условие входа есть, но `main` не объявлена):

```python
#!/usr/bin/env python3

if __name__ == "__main__":
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

<a id="stylebrella-linter-python-idioms-7"></a>

<a id="sb-l-p-id-7"></a>

### SB-L-P-ID-7 — Исполняемый код находится внутри функции

`Stylebrella-linter-python-idioms-7`

Диагностируйте исполняемый код вне функций и методов, в том числе отдельные вызовы, вычисляемые присваивания, циклы и управляющие конструкции на уровне модуля или в теле класса. Допускаются объявления и их синтаксис (включая декораторы, аннотации и значения по умолчанию), импорты и условия преамбулы, управляющие только импортами, docstring, декларативные константы модуля и поля класса, а также экспорт через `__all__`. Эти исключения не разрешают отдельные вызовы или работу скрипта при импорте; инициализацию, требующую вычислений, помещайте в функцию. Условие входа верхнего уровня и его тело проверяются по [SB-L-P-ID-5](#stylebrella-linter-python-idioms-5). Сообщение у нарушающей инструкции: «Перенесите исполняемый код в функцию. Для скрипта объявите `main()` и вызовите её под `if __name__ == "__main__":`».

**Допустимо** (библиотечный модуль):

```python
def refresh_cache() -> None:
  """
  Обновляет локальный кэш по последнему сохранённому снимку.
  """

  cache.reload_snapshot()
```

**Нарушение** (библиотечный модуль):

```python
cache.reload_snapshot()
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

### SB-L-P-D-1 — Обязательный docstring функции

`Stylebrella-linter-python-documentation-1`

Docstring обязателен первой инструкцией в каждой `def` и `async def`, в том числе в методах и вложенных функциях. Комментарии над объявлением не заменяют docstring. У `lambda` нет позиции для docstring, поэтому правило к ней неприменимо. Формат имеющегося docstring проверяется отдельно по [SB-L-P-D-2](#stylebrella-linter-python-documentation-2).

**Допустимо**:

```python
def flush_log() -> None:
  """
  Сохраняет накопленные записи перед закрытием журнала.
  """

  log.flush()
```

**Нарушение**:

```python
def flush_log() -> None:
  log.flush()
```

<a id="stylebrella-linter-python-documentation-2"></a>

<a id="sb-l-p-d-2"></a>

### SB-L-P-D-2 — Структура docstring в стиле Google

`Stylebrella-linter-python-documentation-2`

Используйте многострочный docstring в стиле Google. Открывающие и закрывающие тройные двойные кавычки ставьте на отдельных строках с отступом тела. Краткое описание, остальной текст и заголовки разделов выравнивайте по кавычкам; элементы разделов получают ровно два дополнительных ASCII-пробела, без табуляции. Между абзацами и разделами ставьте одну пустую строку. `Args:` обязателен для объявленных параметров, кроме `self` и `cls` (включая `*args` и `**kwargs` со звёздочками), в порядке сигнатуры. `Returns:` нужен для установленного результата не `None`; выдаваемые генератором значения описываются в `Yields:` вместо `Returns:`. `Raises:` нужен для известных выходящих исключений, `Attributes:` — для атрибутов, где применимо. Требуются точные имена и непустые описания, без повторения аннотаций типов в метках. Неприменимые разделы опускаются. Неверные разделители, отступы, разделы или элементы диагностируются, даже если docstring присутствует.

**Допустимо**:

```python
def row_count(table: Table) -> int:
  """
  Считает строки, видимые в текущем снимке таблицы.

  Args:
    table: Таблица с открытым снимком.

  Returns:
    Число строк в снимке.
  """

  return table.snapshot_size()
```

**Нарушение** (нет отступа у элемента раздела):

```python
def row_count(table: Table) -> int:
  """
  Считает строки, видимые в текущем снимке таблицы.

  Args:
  table: Таблица с открытым снимком.

  Returns:
    Число строк в снимке.
  """

  return table.snapshot_size()
```

## `comments`

<a id="stylebrella-linter-python-comments-1"></a>

<a id="sb-l-p-cm-1"></a>

### SB-L-P-CM-1 — Комментарии к полям dataclass

`Stylebrella-linter-python-comments-1`

[SB-L-G-CM-1](../general/linting.md#stylebrella-linter-general-comments-1) применяется к полям экземпляра dataclass, кроме точных имён `id`, `user_id`, `item_id`.

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

Общие константы размещаются в `constants.py` как `Constants.<Category>.<FIELD>` (например, Math, Entity, Validation, Pattern или предметная категория). Диагностируется именованное значение, на которое ссылаются два или более модуля вне контейнера, либо неканоническая ссылка на разрешённую общую константу. `__main__` в точной проверке точки входа, имена в `__all__` и имена параметров/фикстур pytest освобождены от [SB-L-G-C-1](../general/linting.md#stylebrella-linter-general-constants-1).

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

Тесты, явно классифицированные как зависящие от окружения, помечаются разрешёнными pytest `skip` или `skipif` на тесте, его классе или модуле. К литеральным входным данным тестов и параметризации применяется [SB-L-G-C-1](../general/linting.md#stylebrella-linter-general-constants-1).

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
