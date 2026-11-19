# Kotlin formatting

[Русская версия](../../../ru/code-style/kotlin/formatting.md)

This page adds Kotlin-specific rules to [shared formatting](../formatting.md). It covers `.kt` and `.kts`, including Gradle scripts. See [Kotlin linting](linting.md). Examples are excerpts intended for acceptance fixtures.

## Layout and engine

Use UTF-8, CRLF, no trailing whitespace, and a final newline. Once a call or signature wraps, put each argument or parameter and the closing parenthesis on its own line. Use spaces after commas and around equality, additive, multiplicative, and relational operators. Respect formatter-disable regions. Keep imports ordered with normal imports first, then `java`, `javax`, `kotlin`, and aliases. Use star-import thresholds of five names or three member names, and convert only where resolution is preserved.

The internal preset must not depend on IntelliJ or `ij_*` settings. The output examples are the contract. Choose a Kotlin formatter after comparing candidates (including ktfmt if it can meet the contract); the bundled ktlint preview is not a final compatibility decision. The pipeline must be idempotent.

Before:

```kotlin
fun calculateAndPersist(entityWithConfiguration: OrderEntity, selectedItem: ItemEntity?, evaluationTime: Instant): State {
    return calculate(entityWithConfiguration,selectedItem,evaluationTime)
}
```

After, when expression-body conversion preserves the signature:

```kotlin
fun calculateAndPersist(
  entityWithConfiguration: OrderEntity,
  selectedItem: ItemEntity?,
  evaluationTime: Instant,
) = calculate(entityWithConfiguration, selectedItem, evaluationTime)
```

## Expression bodies

Rewrite a block that is equivalent to a single expression as `= expression`, removing a now-unnecessary `return`. Omit an explicit return type only when inferred type and public contract, override behavior, and `Unit` semantics are unchanged. Do not rewrite early/labeled returns, contracts, or any unresolved case. Example: `fun count(): Int { return items.size }` becomes `fun count() = items.size` if `items.size` is known to be `Int`; an explicit broader public return type may need to remain.

## Blank lines

- Separate block `when` branches, constructor categories, and commented/annotated declarations with one blank line each.
- Keep `else if`, `else`, and `catch` on their own line. One-statement bodies may remain unbraced; formatting does not force braces.

Before:

```kotlin
val processor = Processor()
processor.init(config)
if (config.enabled) {
  processor.start()
} else {
  processor.stop()
}
```

After:

```kotlin
val processor = Processor()
processor.init(config)

if (config.enabled) {
  processor.start()
}

else {
  processor.stop()
}
```

## Separators and private helpers

Use `//` for the shared separators and structural comments. Keep KDoc directly attached to the declaration or its annotations. [Kotlin linting](linting.md#marks-method-order-and-documentation) defines which methods count as private.

```kotlin
class Reader {

  fun read() = source.read()

  // (no private methods)
}
```
