# Kotlin linting

[Русская версия](../../../ru/code-style/kotlin/linting.md)

Target contract for `.kt` and `.kts`, alongside [shared linting](../linting.md) and [Kotlin formatting](formatting.md). Only non-formatting diagnostics are part of `lint`. Prefer resolved Kotlin syntax and symbols; report only conditions established by the analyzer. The selected formatter need not be the lint engine.

## Names, files, and serialization

Use `camelCase` for variables, parameters, fields, functions and methods; `PascalCase` for types (including data classes, interfaces, objects, and enums); `SCREAMING_SNAKE_CASE` for `const val`. A `.kt` name is `PascalCase` and matches its primary type, or describes its purpose for a function-only file (`StringExtensions.kt`). Require one primary declaration of a class, data class, interface, object, enum or annotation type per file. Do not count the accompanying `companion object` or an application `main` as another type. Keep separately defined types in their own files; sanctioned nested constant categories are not independent primary types.

Resolve serializers before checking fields. Every JSON property must explicitly declare `@JsonProperty("snake_case")`. A declaration using `*Request` or `*Response` must participate directly in HTTP request/response serialization in a controller; look for resolved route signatures or explicit project metadata, not guessed framework conventions. Internal DTOs use a role-appropriate `*Input`, `*State`, or `*Plan`. Review unresolved external use and whether the name describes the role.

| Pass                                                           | Report                                                         |
| -------------------------------------------------------------- | -------------------------------------------------------------- |
| `val itemCount`, `fun calculateTotal`, `const val MAX_RETRIES` | `val item_count`, `fun CalculateTotal`, `const val maxRetries` |
| `FilterConfig.kt` containing `data class FilterConfig`         | `filter_config.kt` or two independent types in one file        |
| `@JsonProperty("created_at") val createdAt: Instant`           | Missing annotation or `@JsonProperty("createdAt")`             |
| HTTP `CreateOrderRequest`; internal `CalculationInput`         | Internal-only `CalculationRequest`                             |

## Null handling, conversions, loops and scoping

- Reject `!!` and calls resolving to raw `requireNotNull` or `checkNotNull`. Use `requireNotNullByName` / `requireFieldNotNullByName` for invalid caller input (`IllegalArgumentException`), `checkNotNullByName` / `checkFieldNotNullByName` for internal state (`IllegalStateException`), and `findByIdOrThrow()` for required repository lookups. Plain boolean `check` and `require` are allowed. Choose by context; no automatic replacement.
- Parse strings with `toDouble`, `toInt`, `toLong` or the nullable variants with explicit null handling. Do not use numeric conversions as object casts. Prefer `as?` with null handling or an `is` smart cast. Use `as` only when type is guaranteed (e.g. a proved invariant or an explicitly classified test); report demonstrably unsafe casts and review other unresolved ones.
- Use [the shared loop rule](../linting.md#loop-intent). One-line `if`/`for` is fine when readable.
- `run`, `let`, `apply`, `also`, `with` suit short transformations, setup, logging without breaking the chain, and nullable execution (`?.let`). Report scoping blocks exceeding five executable source lines in canonical layout (extract a function), and a resolved `run`/`with` used only as an unused `Unit` grouping with no receiver-dependent work. Review unrelated grouping and unnecessary nesting; do not flag `apply` used to configure its receiver merely because the block returns `Unit`.

```kotlin
val itemId = request.itemId.requireFieldNotNullByName { "itemId" }
val processor = this.processor.checkFieldNotNullByName { "processor" }
val point = input as? Point ?: return

if (input is Point)
  processPoint(input)
```

Report `request.itemId!!` or `requireNotNull(request.itemId)`. Inline names passed to resolved named-null helpers are validation field metadata; any actual domain value/default passed alongside is subject to the literal policy.

## MARKs, method order, and documentation

Apply [shared MARK and private method rules](../linting.md#mark-text-and-importance). Match `// MARK:` and use declared Kotlin visibility (implicit public included; do not treat `internal` as `private`). A controller MARK may be `// MARK: POST /api/order/calculate`.

Require multiline KDoc for functions with more than 15 nonblank, non-comment lines of executable body in canonical layout, important routines (including the first private helper under the structural MARK), and a function with its own descriptive MARK. Check attachment to the declaration, before annotations where present.

Use `/**` and `*/` on their own lines, `*` before text, and ` *` on blank lines. Start with an untagged summary, then optional detail paragraphs, a blank KDoc line, and applicable tags: `@param name description` in signature order, `@return description` for non-`Unit` results, and `@throws Type description` for known exceptions. Do not append a colon to the name or repeat types. Omit inapplicable tags and the preceding blank line when there are no tags. Prose must be Russian; ignore identifiers, tags, links and code when checking language. Diagnose clear violations and review ambiguous prose. A one-line `/** ... */` does not satisfy this requirement.

Check side comments on data-class properties except `id`, `userId`, `itemId`. Group constructor parameters with `// repositories:`, `// mappers:`, and `// services:` matched by resolved type suffix; allow configured categories, review unknown classifications.

```kotlin
// MARK: Calculate total
// --------------------------------------------------

/**
 * Вычисляет итоговую стоимость заказа.
 *
 * @param order заказ для расчёта
 * @return итоговая стоимость
 */
fun calculateTotal(order: Order) = calculator.total(order)
```

## Constants

Define shared values under `object Constants` in `Constants.kt`, organized by nested objects such as `Math`, `Entity`, `ErrorDescription`, `Validation`, and `Pattern`. Use `Constants.<Object>.<FIELD>` (e.g. `Constants.Validation.INVALID_EMAIL`) rather than embedding a regex or a domain value. The [shared inline-literal policy](../linting.md#constants-and-inline-literals) allows message text in recognized printing, logging and exception calls. `@JsonProperty("user_id")` declares a schema name and is exempt. Other literals in a test remain subject to the rules.
