# Linting Kotlin

[Русская версия](../../../ru/code-style/kotlin/linting.md)

Scope: `.kt` and `.kts`. Visibility is as declared, including implicit `public`; `internal` is not `private`.

[Shared rules](../general/linting.md)

Examples isolate the rule being illustrated. Function examples are focused excerpts; unless documentation is the rule under test, any required documentation omitted from an excerpt is assumed present in the full source context.

## `naming`

<a id="stylebrella-linter-kotlin-naming-1"></a>

<a id="sb-l-k-n-1"></a>

### SB-L-K-N-1 — Naming conventions for Kotlin declarations

`Stylebrella-linter-kotlin-naming-1`

Require camelCase for variables, parameters, properties and public callables; PascalCase for class, interface, object, enum and annotation types; and SCREAMING_SNAKE_CASE for `const val`. Private method names use the underscore prefix required by [SB-L-G-N-3](../general/linting.md#sb-l-g-n-3), followed by a camelCase name.

**Allowed**:

```kotlin
const val MAX_RETRIES = 3
```

**Violation**:

```kotlin
const val maxRetries = 3
```

<a id="stylebrella-linter-kotlin-naming-2"></a>

<a id="sb-l-k-n-2"></a>

### SB-L-K-N-2 — DTO suffixes by role

`Stylebrella-linter-kotlin-naming-2`

A DTO named `*Request` or `*Response` must appear directly in a resolved controller signature or explicit HTTP serialization metadata for the matching direction. Report misuse only if classified as non-HTTP or mismatched; unresolved external use is not evidence. A DTO confirmed to be internal must end in `Input`, `State`, or `Plan`.

**Allowed** (internal DTO):

```kotlin
data class CalculationInput(val count: Int)
```

**Violation** (internal DTO):

```kotlin
data class CalculationRequest(val count: Int)
```

## `files`

<a id="stylebrella-linter-kotlin-files-1"></a>

<a id="sb-l-k-fi-1"></a>

### SB-L-K-FI-1 — File naming and one primary type per file

`Stylebrella-linter-kotlin-files-1`

A `.kt` filename is PascalCase and matches its primary top-level type, if present. Apply [SB-L-G-FI-1](../general/linting.md#sb-l-g-fi-1) to classes. The one-type limit also covers top-level interfaces, named objects, enums and annotation classes; do not duplicate a diagnostic from the shared rule. Nested types and standalone functions do not count. Use the first eligible type for the filename check if the file contains multiple types. A functions-only file has a PascalCase descriptive filename; only its casing is checked.

**Allowed** (`FilterConfig.kt`):

```kotlin
data class FilterConfig(val size: Int)
```

**Violation** (`filter_config.kt`):

```kotlin
data class FilterConfig(val size: Int)
```

## `json`

<a id="stylebrella-linter-kotlin-json-1"></a>

<a id="sb-l-k-j-1"></a>

### SB-L-K-J-1 — Explicit snake_case JSON properties

`Stylebrella-linter-kotlin-json-1`

Each positively identified JSON property must explicitly declare `@JsonProperty` with a statically known snake_case key. Inspect only properties resolved to the project's serializer; arbitrary strings do not match.

**Allowed**:

```kotlin
@JsonProperty("created_at") val createdAt: Instant
```

**Violation**:

```kotlin
@JsonProperty("createdAt") val createdAt: Instant
```

## `nulls`

<a id="stylebrella-linter-kotlin-nulls-1"></a>

<a id="sb-l-k-nu-1"></a>

### SB-L-K-NU-1 — No non-null assertions

`Stylebrella-linter-kotlin-nulls-1`

Report each Kotlin non-null assertion operator `!!` at the operator. A text occurrence in a string or comment is not an operator. Do not replace it automatically.

**Allowed**:

```kotlin
val itemId = request.itemId.requireFieldNotNullByName { "itemId" }
```

**Violation**:

```kotlin
val itemId = request.itemId!!
```

<a id="stylebrella-linter-kotlin-nulls-2"></a>

<a id="sb-l-k-nu-2"></a>

### SB-L-K-NU-2 — Named null validation and required lookups

`Stylebrella-linter-kotlin-nulls-2`

Report direct calls resolved to standard `requireNotNull` or `checkNotNull`. Plain `require` and `check` on booleans are permitted. The named forms for caller input are `requireNotNullByName` and `requireFieldNotNullByName`; for state, `checkNotNullByName` and `checkFieldNotNullByName`; for required repository lookup, use resolved `findByIdOrThrow` when project metadata identifies that role and a required lookup is instead unwrapped at the call site. Do not infer the caller/state split from an argument name or auto-fix it.

**Allowed**:

```kotlin
val processor = processorInput.checkNotNullByName { "processorInput" }
```

**Violation**:

```kotlin
val processor = checkNotNull(processorInput)
```

## `types`

<a id="stylebrella-linter-kotlin-types-1"></a>

<a id="sb-l-k-t-1"></a>

### SB-L-K-T-1 — Numeric parsing and null handling

`Stylebrella-linter-kotlin-types-1`

For resolved String-to-number operations use standard `toInt`, `toLong`, `toDouble` or nullable variants with an explicit null branch. Report a resolved numeric conversion used where the input is positively identified as an object cast, or a nullable parse used as a definite result without handling null. Other conversion intents are not inferred.

**Allowed**:

```kotlin
val n: Int = raw.toIntOrNull() ?: return
```

**Violation**:

```kotlin
val n: Int = raw.toIntOrNull()
```

<a id="stylebrella-linter-kotlin-types-2"></a>

<a id="sb-l-k-t-2"></a>

### SB-L-K-T-2 — Unsafe casts

`Stylebrella-linter-kotlin-types-2`

Report an unsafe `as` when analysis proves the source may be incompatible and no dominating type test or explicit, validated test invariant establishes the cast. Do not report an unresolved case merely because it uses `as`.

**Allowed**:

```kotlin
val point = input as? Point ?: return
```

**Violation** (input can be a String):

```kotlin
val point = input as Point
```

## `idioms`

<a id="stylebrella-linter-kotlin-idioms-1"></a>

<a id="sb-l-k-id-1"></a>

### SB-L-K-ID-1 — Scope function body length

`Stylebrella-linter-kotlin-idioms-1`

Report blocks passed to resolved standard `run`, `let`, `apply`, `also` or `with` when they exceed five executable body lines counted as in [SB-L-G-M-2](../general/linting.md#stylebrella-linter-general-mark-2). Exactly five does not trigger this rule.

**Allowed** (2 lines):

```kotlin
item.let {
  a(it)
  b(it)
}
```

**Violation**:

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

### SB-L-K-ID-2 — No run or with used only for grouping

`Stylebrella-linter-kotlin-idioms-2`

Report resolved standard `run` or `with` if the result is unused `Unit` and the block makes no explicit or implicit reference to its receiver. Do not apply to `apply`.

**Allowed**:

```kotlin
obj.apply { clear() }
```

**Violation** (unused Unit, no receiver use):

```kotlin
with(obj) { logStart(); logEnd() }
```

## `documentation`

<a id="stylebrella-linter-kotlin-documentation-1"></a>

<a id="sb-l-k-d-1"></a>

### SB-L-K-D-1 — Required function KDoc

`Stylebrella-linter-kotlin-documentation-1`

Require KDoc for every named function, including local functions, methods, and expression-bodied functions. Attach it before any declaration annotations. Check present KDoc separately under [SB-L-K-D-2](#stylebrella-linter-kotlin-documentation-2). Lambdas and anonymous functions are not applicable.

**Allowed**:

```kotlin
/**
 * Сохраняет накопленные записи перед закрытием журнала.
 */
fun flushLog(): Unit = log.flush()
```

**Violation**:

```kotlin
fun flushLog(): Unit = log.flush()
```

<a id="stylebrella-linter-kotlin-documentation-2"></a>

<a id="sb-l-k-d-2"></a>

### SB-L-K-D-2 — KDoc structure, tags, and language

`Stylebrella-linter-kotlin-documentation-2`

KDoc opening `/**` and closing `*/` occupy separate lines; each interior line begins with `*` after indentation. Begin with a nonempty untagged summary. Before the tag list use one empty starred line. Require `@param name description` for each declared value parameter in signature order (including a `vararg` once), `@return` for resolved non-`Unit` results, and `@throws Type description` for known escaping exceptions. All applicable tags need nonempty descriptions. No colon after parameter names or repeated type labels. Omit inapplicable tags and the tag delimiter when there are none. Require Russian prose; report nonempty prose consisting of Latin-script words with no Cyrillic after excluding tags, code, links and identifiers. Other language ambiguity is reviewed separately. Report incorrect delimiters, layout, tags, or descriptions even when KDoc is present.

**Allowed**:

```kotlin
/**
 * Возвращает задержку, после которой можно повторить запрос.
 *
 * @param attempt номер попытки, начиная с единицы
 * @return задержка до следующей попытки
 */
fun retryDelay(attempt: Int): Duration = backoff.delayFor(attempt)
```

**Violation**:

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

### SB-L-K-CM-1 — Comments on data class properties

`Stylebrella-linter-kotlin-comments-1`

Apply [SB-L-G-CM-1](../general/linting.md#stylebrella-linter-general-comments-1) to data-class properties except exact names `id`, `userId` and `itemId`.

**Allowed**:

```kotlin
val dfHz: Double, // Смещение, Гц
```

**Violation**:

```kotlin
val dfHz: Double,
```

<a id="stylebrella-linter-kotlin-comments-2"></a>

<a id="sb-l-k-cm-2"></a>

### SB-L-K-CM-2 — Constructor parameter categories

`Stylebrella-linter-kotlin-comments-2`

Group resolved constructor parameter types with suffix `Repository`, `Mapper` or `Service` under matching `// repositories:`, `// mappers:` or `// services:` markers, or an explicit configured category. Unknown types do not trigger the rule.

**Allowed**:

```kotlin
// services:
val service: OrderService,
```

**Violation**:

```kotlin
// repositories:
val service: OrderService,
```

## `constants`

<a id="stylebrella-linter-kotlin-constants-1"></a>

<a id="sb-l-k-c-1"></a>

### SB-L-K-C-1 — Shared constants in Constants.kt

`Stylebrella-linter-kotlin-constants-1`

Put shared constants in `Constants.kt`, under `object Constants` and a nested category such as `Math`, `Entity`, `ErrorDescription`, `Validation` or `Pattern`. Require `Constants.<Object>.<FIELD>` for resolved shared constants; a value referenced across files outside that container violates the rule.

**Allowed**:

```kotlin
Constants.Validation.INVALID_EMAIL
```

**Violation** (shared declaration outside Constants):

```kotlin
INVALID_EMAIL
```

<a id="stylebrella-linter-kotlin-constants-2"></a>

<a id="sb-l-k-c-2"></a>

### SB-L-K-C-2 — Literal exceptions for schema and validation metadata

`Stylebrella-linter-kotlin-constants-2`

Exempt the schema key in a resolved `@JsonProperty` and the field name in a recognized named null-validation call from [SB-L-G-C-1](../general/linting.md#stylebrella-linter-general-constants-1). Runtime defaults supplied to such a call remain subject to the rule; test data are not exempt.

**Allowed**:

```kotlin
request.itemId.requireFieldNotNullByName { "itemId" }
```

**Violation** (runtime argument):

```kotlin
connect(8080)
```

## Recommendations (no diagnostic)

Use named null validation for the appropriate failure owner. Prefer smart casts or `as?` with a null branch. Keep one-line control statements readable and avoid nesting unrelated scope functions. Name internal DTOs by purpose. In KDoc explain the purpose, not just the signature.
