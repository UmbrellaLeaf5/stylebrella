# Линтинг Kotlin

[English version](../../../en/code-style/kotlin/linting.md)

Область: `.kt` и `.kts`. Видимость берётся из объявления, включая неявный `public`; `internal` не считается `private`.

[Общие правила](../linting.md)

Каждый пример изолирует указанное правило.

## `naming`

<a id="stylebrella-linter-kotlin-naming-1"></a>

<a id="sb-l-k-n-1"></a>

### SB-L-K-N-1 — Имена объявлений Kotlin

`Stylebrella-linter-kotlin-naming-1`

Для переменных, параметров, свойств и публичных функций требуется camelCase, для классов, интерфейсов, объектов, перечислений и аннотаций — PascalCase, для `const val` — SCREAMING_SNAKE_CASE. Приватные методы используют префикс подчёркивания по [SB-L-G-N-2](../linting.md#sb-l-g-n-2), за которым следует имя в camelCase.

**Допустимо**:

```kotlin
const val MAX_RETRIES = 3
```

**Нарушение**:

```kotlin
const val maxRetries = 3
```

<a id="stylebrella-linter-kotlin-naming-2"></a>

<a id="sb-l-k-n-2"></a>

### SB-L-K-N-2 — Суффиксы DTO по назначению

`Stylebrella-linter-kotlin-naming-2`

DTO с суффиксом `Request` или `Response` должен непосредственно входить в разрешённую сигнатуру контроллера или явные метаданные HTTP-сериализации нужного направления. Диагностика только при подтверждённой не-HTTP роли или несовпадении направления; неразрешённое внешнее использование не доказательство. Для подтверждённого внутреннего DTO обязателен суффикс `Input`, `State` или `Plan`.

**Допустимо** (внутренний DTO):

```kotlin
data class CalculationInput(val count: Int)
```

**Нарушение** (внутренний DTO):

```kotlin
data class CalculationRequest(val count: Int)
```

## `files`

<a id="stylebrella-linter-kotlin-files-1"></a>

<a id="sb-l-k-fi-1"></a>

### SB-L-K-FI-1 — Имена файлов и один основной тип на файл

`Stylebrella-linter-kotlin-files-1`

Имя `.kt` пишется в PascalCase и совпадает с единственным основным типом. Второй самостоятельный класс, интерфейс, объект, перечисление или аннотация диагностируются, включая самостоятельные вложенные типы. Исключения: `companion object`, `main` приложения и категории констант по [SB-L-K-C-1](#stylebrella-linter-kotlin-constants-1). Имя файла только с функциями описательное в PascalCase; автоматически проверяется регистр.

**Допустимо** (`FilterConfig.kt`):

```kotlin
data class FilterConfig(val size: Int)
```

**Нарушение** (`filter_config.kt`):

```kotlin
data class FilterConfig(val size: Int)
```

## `json`

<a id="stylebrella-linter-kotlin-json-1"></a>

<a id="sb-l-k-j-1"></a>

### SB-L-K-J-1 — Явные JSON-свойства в snake_case

`Stylebrella-linter-kotlin-json-1`

Для каждого подтверждённого свойства JSON требуется явная `@JsonProperty` со статически известным ключом snake_case. Проверяются только свойства, относящиеся к установленному сериализатору проекта; произвольные строки не подходят.

**Допустимо**:

```kotlin
@JsonProperty("created_at") val createdAt: Instant
```

**Нарушение**:

```kotlin
@JsonProperty("createdAt") val createdAt: Instant
```

## `nulls`

<a id="stylebrella-linter-kotlin-nulls-1"></a>

<a id="sb-l-k-nu-1"></a>

### SB-L-K-NU-1 — Запрет оператора !!

`Stylebrella-linter-kotlin-nulls-1`

Каждый оператор утверждения ненулевого значения `!!` диагностируется на операторе. Текст в строке или комментарии не считается оператором. Автозамены нет.

**Допустимо**:

```kotlin
val itemId = request.itemId.requireFieldNotNullByName { "itemId" }
```

**Нарушение**:

```kotlin
val itemId = request.itemId!!
```

<a id="stylebrella-linter-kotlin-nulls-2"></a>

<a id="sb-l-k-nu-2"></a>

### SB-L-K-NU-2 — Именованные проверки null и обязательный поиск

`Stylebrella-linter-kotlin-nulls-2`

Диагностируются прямые вызовы стандартных `requireNotNull` или `checkNotNull`. Обычные `require` и `check` для булевых условий разрешены. Именованные формы для входа: `requireNotNullByName`, `requireFieldNotNullByName`; для состояния: `checkNotNullByName`, `checkFieldNotNullByName`; для обязательного поиска — разрешённый `findByIdOrThrow`, если метаданные подтверждают роль, а результат обязательного поиска разворачивается на месте. По имени аргумента роль не выводится; автозамены нет.

**Допустимо**:

```kotlin
val processor = processorInput.checkNotNullByName { "processorInput" }
```

**Нарушение**:

```kotlin
val processor = checkNotNull(processorInput)
```

## `types`

<a id="stylebrella-linter-kotlin-types-1"></a>

<a id="sb-l-k-t-1"></a>

### SB-L-K-T-1 — Разбор чисел и обработка null

`Stylebrella-linter-kotlin-types-1`

Для подтверждённого преобразования String в число используются стандартные `toInt`, `toLong`, `toDouble` либо nullable-варианты с явной ветвью null. Диагностируется числовое преобразование в подтверждённом контексте приведения объекта либо использование nullable-результата разбора как гарантированного без обработки null. Другие намерения не выводятся.

**Допустимо**:

```kotlin
val n: Int = raw.toIntOrNull() ?: return
```

**Нарушение**:

```kotlin
val n: Int = raw.toIntOrNull()
```

<a id="stylebrella-linter-kotlin-types-2"></a>

<a id="sb-l-k-t-2"></a>

### SB-L-K-T-2 — Небезопасные приведения типов

`Stylebrella-linter-kotlin-types-2`

Диагностируется небезопасный `as`, если анализ подтверждает возможность несовместимости и нет доминирующей проверки типа или явного проверенного инварианта теста. Неразрешённый случай не диагностируется только за использование `as`.

**Допустимо**:

```kotlin
val point = input as? Point ?: return
```

**Нарушение** (`input` может иметь тип `String`):

```kotlin
val point = input as Point
```

## `idioms`

<a id="stylebrella-linter-kotlin-idioms-1"></a>

<a id="sb-l-k-id-1"></a>

### SB-L-K-ID-1 — Длина блока функции области видимости

`Stylebrella-linter-kotlin-idioms-1`

Блок стандартной функции `run`, `let`, `apply`, `also` или `with` диагностируется при превышении пяти исполняемых строк тела, подсчитанных по [SB-L-G-M-2](../linting.md#stylebrella-linter-general-mark-2). Ровно пять строк не нарушают правило.

**Допустимо** (две исполняемые строки):

```kotlin
item.let {
  a(it)
  b(it)
}
```

**Нарушение**:

```kotlin
item.let {
  a(it)
  b(it)
  c(it)
  d(it)
  e(it)
  f(it)
}
```

<a id="stylebrella-linter-kotlin-idioms-2"></a>

<a id="sb-l-k-id-2"></a>

### SB-L-K-ID-2 — Запрет run и with только для группировки

`Stylebrella-linter-kotlin-idioms-2`

Стандартные `run` или `with` диагностируются, если результат — неиспользуемый `Unit` и в блоке нет явной или неявной ссылки на получатель. К `apply` не применяется.

**Допустимо**:

```kotlin
obj.apply { clear() }
```

**Нарушение** (результат `Unit` не используется, обращения к получателю нет):

```kotlin
with(obj) { logStart(); logEnd() }
```

## `documentation`

<a id="stylebrella-linter-kotlin-documentation-1"></a>

<a id="sb-l-k-d-1"></a>

### SB-L-K-D-1 — Обязательный KDoc функции

`Stylebrella-linter-kotlin-documentation-1`

KDoc обязателен для каждой именованной функции, в том числе для локальных функций, методов и функций с телом-выражением. Размещайте KDoc перед аннотациями объявления. Формат имеющегося KDoc проверяется отдельно по [SB-L-K-D-2](#stylebrella-linter-kotlin-documentation-2). К лямбдам и анонимным функциям правило неприменимо.

**Допустимо**:

```kotlin
/**
 * Сохраняет накопленные записи перед закрытием журнала.
 */
fun flushLog(): Unit = log.flush()
```

**Нарушение**:

```kotlin
fun flushLog(): Unit = log.flush()
```

<a id="stylebrella-linter-kotlin-documentation-2"></a>

<a id="sb-l-k-d-2"></a>

### SB-L-K-D-2 — Структура, теги и язык KDoc

`Stylebrella-linter-kotlin-documentation-2`

Открытие `/**` и закрытие `*/` KDoc располагаются на отдельных строках; внутренняя строка после отступа начинается с `*`. Сначала непустое описание без тега. Перед списком тегов одна пустая строка со звёздочкой. Требуются `@param name description` для каждого объявленного параметра-значения в порядке сигнатуры (один раз для `vararg`), `@return` для установленного результата не `Unit` и `@throws Type description` для известных выходящих исключений. У каждого применимого тега должно быть непустое описание. После имени параметра нет двоеточия или повтора типа. Неприменимые теги и отделяющая строка при их отсутствии опускаются. Требуется русский текст; диагностируется непустой текст из латинских слов без кириллицы после исключения тегов, кода, ссылок и идентификаторов. Другие неоднозначные случаи проверяются на ревью. Неверные разделители, раскладка, теги или описания диагностируются, даже если KDoc присутствует.

**Допустимо**:

```kotlin
/**
 * Возвращает задержку, после которой можно повторить запрос.
 *
 * @param attempt номер попытки, начиная с единицы
 * @return задержка до следующей попытки
 */
fun retryDelay(attempt: Int): Duration = backoff.delayFor(attempt)
```

**Нарушение**:

```kotlin
/**
 * Возвращает задержку, после которой можно повторить запрос.
 *
 * @return задержка до следующей попытки
 */
fun retryDelay(attempt: Int): Duration = backoff.delayFor(attempt)
```

```kotlin
/**
 * Запускает задачи.
 */
fun startTasks(): Unit = scheduler.enqueueReady()
```

## `comments`

<a id="stylebrella-linter-kotlin-comments-1"></a>

<a id="sb-l-k-cm-1"></a>

### SB-L-K-CM-1 — Комментарии к свойствам data class

`Stylebrella-linter-kotlin-comments-1`

[SB-L-G-CM-1](../linting.md#stylebrella-linter-general-comments-1) применяется к свойствам data class, кроме точных имён `id`, `userId`, `itemId`.

**Допустимо**:

```kotlin
val dfHz: Double, // Смещение, Гц
```

**Нарушение**:

```kotlin
val dfHz: Double,
```

<a id="stylebrella-linter-kotlin-comments-2"></a>

<a id="sb-l-k-cm-2"></a>

### SB-L-K-CM-2 — Категории параметров конструктора

`Stylebrella-linter-kotlin-comments-2`

Разрешённые типы параметров конструктора с суффиксами `Repository`, `Mapper`, `Service` размещаются под соответствующими `// repositories:`, `// mappers:`, `// services:` либо явно настроенной категорией. Неизвестные типы правило не запускают.

**Допустимо**:

```kotlin
// services:
val service: OrderService,
```

**Нарушение**:

```kotlin
// repositories:
val service: OrderService,
```

## `constants`

<a id="stylebrella-linter-kotlin-constants-1"></a>

<a id="sb-l-k-c-1"></a>

### SB-L-K-C-1 — Общие константы в Constants.kt

`Stylebrella-linter-kotlin-constants-1`

Общие константы находятся в `Constants.kt` под `object Constants` во вложенной категории вроде `Math`, `Entity`, `ErrorDescription`, `Validation`, `Pattern`. Для разрешённых общих констант требуется `Constants.<Object>.<FIELD>`; значение, используемое из нескольких файлов вне контейнера, нарушает правило.

**Допустимо**:

```kotlin
Constants.Validation.INVALID_EMAIL
```

**Нарушение** (общее объявление вне `Constants`):

```kotlin
INVALID_EMAIL
```

<a id="stylebrella-linter-kotlin-constants-2"></a>

<a id="sb-l-k-c-2"></a>

### SB-L-K-C-2 — Исключения для литералов схемы и метаданных валидации

`Stylebrella-linter-kotlin-constants-2`

Ключ схемы в разрешённой `@JsonProperty` и имя поля в распознанной именованной проверке null освобождены от [SB-L-G-C-1](../linting.md#stylebrella-linter-general-constants-1). Передаваемые значения по умолчанию и тестовые данные не освобождаются.

**Допустимо**:

```kotlin
request.itemId.requireFieldNotNullByName { "itemId" }
```

**Нарушение** (аргумент времени выполнения):

```kotlin
connect(8080)
```

## Рекомендации (без диагностики)

Выбирайте именованную проверку null по источнику сбоя. Предпочитайте умное приведение или `as?` с ветвью для null. Следите за читаемостью однострочных конструкций и не вкладывайте несвязанные функции области видимости. Называйте внутренние DTO по назначению. В KDoc поясняйте смысл, а не повторяйте сигнатуру.
