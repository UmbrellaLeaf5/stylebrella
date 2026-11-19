# Linting C++

[Русская версия](../../../ru/code-style/cpp/linting.md)

Scope: `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, `.hh`; CMake is excluded. Use the project compilation database to resolve semantic rules. Without it, report that semantic checks were skipped and still run independent file and syntax rules. Access follows `class`/`struct` defaults and access labels; protected methods are not private.

[Shared rules](../linting.md)

Examples isolate the rule being illustrated.

## `naming`

<a id="stylebrella-linter-cpp-naming-1"></a>

<a id="sb-l-c-n-1"></a>

### SB-L-C-N-1 — Naming conventions for C++ declarations

`Stylebrella-linter-cpp-naming-1`

Use PascalCase for classes, structs and enums; snake_case for functions, variables, fields and parameters; lowercase for namespaces. Private fields have one leading underscore unless that spelling is reserved by C++. Private method names follow [SB-L-G-N-2](../linting.md#sb-l-g-n-2); protected members do not get a private prefix.

**Allowed** (private field):

```cpp
int _gain_max_dbi;
```

**Violation** (private field):

```cpp
int gain_max_dbi;
```

<a id="stylebrella-linter-cpp-naming-2"></a>

<a id="sb-l-c-n-2"></a>

### SB-L-C-N-2 — Names of const and constexpr values

`Stylebrella-linter-cpp-naming-2`

A constant declared `const` or `constexpr`, including a local one, uses SCREAMING_SNAKE_CASE. A private constant member retains its leading underscore if the resulting identifier is legal; if reserved, use the non-prefixed constant spelling. Method cv-qualifiers, pointed-to/referred-to constness, and read-only parameters are not constant declarations.

**Allowed**:

```cpp
constexpr int MAX_RETRIES = 3;
```

**Violation**:

```cpp
constexpr int maxRetries = 3;
```

## `json`

<a id="stylebrella-linter-cpp-json-1"></a>

<a id="sb-l-c-j-1"></a>

### SB-L-C-J-1 — Snake_case JSON keys

`Stylebrella-linter-cpp-json-1`

A resolved C++ JSON key uses snake_case. Ignore strings with no established JSON role.

**Allowed**:

```cpp
record["center_freq_hz"]
```

**Violation**:

```cpp
record["centerFreqHz"]
```

## `files`

<a id="stylebrella-linter-cpp-files-1"></a>

<a id="sb-l-c-fi-1"></a>

### SB-L-C-FI-1 — Header extensions and pragma once

`Stylebrella-linter-cpp-files-1`

An owned header has the `.hpp` extension unless explicitly marked legacy or third-party. Its preamble contains `#pragma once` and no file-wide macro include guard.

**Allowed** (`Antenna.hpp`):

```cpp
#pragma once

class Antenna;
```

**Violation** (`Antenna.h`):

```cpp
#ifndef ANTENNA_H
#define ANTENNA_H

class Antenna;

#endif
```

<a id="stylebrella-linter-cpp-files-2"></a>

<a id="sb-l-c-fi-2"></a>

### SB-L-C-FI-2 — File names, primary types, and source layout

`Stylebrella-linter-cpp-files-2`

A type file contains one primary named class, struct or enum and has the same PascalCase stem. Files classified as function helpers use snake_case. A resolved namespace directory follows the namespace path relative to the configured source root; project declarations and implementations belong to `include/` and `src/`, respectively, with inline/template definitions allowed in headers.

**Allowed** (`include/app/core/Antenna.hpp`):

```cpp
namespace core {
class Antenna;
}
```

**Violation** (`include/app/math/antenna.hpp`):

```cpp
namespace core {
class Antenna;
}
```

<a id="stylebrella-linter-cpp-files-3"></a>

<a id="sb-l-c-fi-3"></a>

### SB-L-C-FI-3 — Shared types and component types

`Stylebrella-linter-cpp-files-3`

For a project type with a known complete reference graph and at least one consuming class, use `components/` when exactly one class uses it and `shared/` when at least two do. Do not infer this rule from an incomplete graph.

**Allowed** (`shared/Point.hpp`; used by two classes):

```cpp
class Point {};
```

**Violation** (`components/Point.hpp`; used by two classes):

```cpp
class Point {};
```

## `types`

<a id="stylebrella-linter-cpp-types-1"></a>

<a id="sb-l-c-t-1"></a>

### SB-L-C-T-1 — No C-style casts

`Stylebrella-linter-cpp-types-1`

Reject C-style cast syntax. Report at the cast, not at ordinary grouping parentheses. Named casts are permitted subject to ordinary type correctness.

**Allowed**:

```cpp
static_cast<double>(count)
```

**Violation**:

```cpp
(double)count
```

<a id="stylebrella-linter-cpp-types-2"></a>

<a id="sb-l-c-t-2"></a>

### SB-L-C-T-2 — Nullptr for null pointers

`Stylebrella-linter-cpp-types-2`

Require `nullptr` for a resolved null pointer constant written as `NULL` or integer `0` in a pointer context.

**Allowed**:

```cpp
Widget* ptr = nullptr;
```

**Violation**:

```cpp
Widget* ptr = 0;
```

<a id="stylebrella-linter-cpp-types-3"></a>

<a id="sb-l-c-t-3"></a>

### SB-L-C-T-3 — When to use auto

`Stylebrella-linter-cpp-types-3`

Require `auto` for resolved iterator, lambda closure, or unnamed result types; reject `auto` for a plain fundamental literal initializer. Other choices of inferred versus explicit type are reviewed separately.

**Allowed**:

```cpp
int count = 5;
```

**Violation**:

```cpp
auto count = 5;
```

<a id="stylebrella-linter-cpp-types-4"></a>

<a id="sb-l-c-t-4"></a>

### SB-L-C-T-4 — Const and constexpr for local values

`Stylebrella-linter-cpp-types-4`

Require `const` on an initialized local variable if analysis establishes no modification, mutable access or escape and adding it preserves validity and overload resolution. Require `constexpr` for a constant value when initialization is evaluable at compile time and the declaration can be `constexpr` without changing its contract. Report only established cases.

**Allowed**:

```cpp
constexpr int MAX_RETRIES = 3;
```

**Violation**:

```cpp
const int MAX_RETRIES = 3;
```

<a id="stylebrella-linter-cpp-types-5"></a>

<a id="sb-l-c-t-5"></a>

### SB-L-C-T-5 — Const methods and explicit override

`Stylebrella-linter-cpp-types-5`

Require a `const` instance method when its complete definition is known not to mutate the object and the qualification preserves its signature requirements and overload resolution. For an actual override require the `override` specifier on the in-class declaration. Lack of derived classes does not trigger a `final` diagnostic.

**Allowed**:

```cpp
Result read() const override;
```

**Violation** (confirmed override):

```cpp
Result read() const;
```

## `idioms`

<a id="stylebrella-linter-cpp-idioms-1"></a>

<a id="sb-l-c-id-1"></a>

### SB-L-C-ID-1 — Redundant standalone blocks

`Stylebrella-linter-cpp-idioms-1`

Report a standalone compound statement used only for grouping when it creates no required scope or lifetime boundary. Exclude control, function, namespace and class bodies and initializers. Unknown scope or lifetime effects suppress the finding.

**Allowed**:

```cpp
log();
flush();
```

**Violation**:

```cpp
{ log(); flush(); }
```

## `documentation`

<a id="stylebrella-linter-cpp-documentation-1"></a>

<a id="sb-l-c-d-1"></a>

### SB-L-C-D-1 — When declarations and functions need Doxygen

`Stylebrella-linter-cpp-documentation-1`

Require multiline Doxygen immediately above function, class, struct and enum declarations in `.hpp`. For any in-scope function with more than 15 executable body lines or more than 3 declared function parameters, require Doxygen on the declaration or definition; an attached valid declaration block suffices. Count variadic packs once and body lines using [SB-L-G-M-2](../linting.md#stylebrella-linter-general-mark-2). Apply [SB-L-G-M-5](../linting.md#stylebrella-linter-general-mark-5) to the first important private method.

**Allowed**:

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

**Violation**:

```cpp
void f(int a, int b, int c, int d) {}
```

<a id="stylebrella-linter-cpp-documentation-2"></a>

<a id="sb-l-c-d-2"></a>

### SB-L-C-D-2 — Doxygen structure, tags, and language

`Stylebrella-linter-cpp-documentation-2`

Doxygen opens with `/**` and closes with `*/` on separate lines; use a starred line for each interior line, including empty ones. Begin with a nonempty `@brief`; use `@param name description` in signature order, `@return` for non-void, and `@throws Type description` for known documented exceptions. Omit inapplicable tags, a colon after the parameter name, and a `Type:` label for the result. Apply the same Cyrillic/Latin prose check as [SB-L-K-D-2](../kotlin/linting.md#stylebrella-linter-kotlin-documentation-2); `///` and one-line blocks do not satisfy the multiline requirement.

**Allowed**:

```cpp
/**
 * @brief Читает настройки.
 * @return Прочитанные настройки.
 */
Settings read();
```

**Violation**:

```cpp
/// Reads settings.
Settings read();
```

## `comments`

<a id="stylebrella-linter-cpp-comments-1"></a>

<a id="sb-l-c-cm-1"></a>

### SB-L-C-CM-1 — Comments on struct fields

`Stylebrella-linter-cpp-comments-1`

Apply [SB-L-G-CM-1](../linting.md#stylebrella-linter-general-comments-1) to struct fields except exact names `id` and `name`. Apply MARK and separator rules in both headers and implementation files.

**Allowed**:

```cpp
double df_hz; // Смещение, Гц
```

**Violation**:

```cpp
double df_hz;
```

## `constants`

<a id="stylebrella-linter-cpp-constants-1"></a>

<a id="sb-l-c-c-1"></a>

### SB-L-C-C-1 — Shared constant placement and qualification

`Stylebrella-linter-cpp-constants-1`

Place a named constant used across two or more source files in the configured project constants header and namespace, with a category such as `math`, `entity`, `config`, `validation` or `pattern`. Require a fully qualified reference to resolved shared constants outside that namespace.

**Allowed**:

```cpp
app::constants::config::CENTER_FREQ_HZ
```

**Violation** (unqualified use outside namespace):

```cpp
CENTER_FREQ_HZ
```

<a id="stylebrella-linter-cpp-constants-2"></a>

<a id="sb-l-c-c-2"></a>

### SB-L-C-C-2 — Inline literals and message exemptions

`Stylebrella-linter-cpp-constants-2`

Apply [SB-L-G-C-1](../linting.md#stylebrella-linter-general-constants-1) to C++ shared numbers, patterns, paths and default values at runtime use sites. The text of exception, log or console messages is exempt only in the message position defined by [SB-L-G-C-2](../linting.md#stylebrella-linter-general-constants-2).

**Allowed**:

```cpp
connect(app::constants::config::PORT);
```

**Violation**:

```cpp
connect(8080);
```

## `exceptions`

<a id="stylebrella-linter-cpp-exceptions-1"></a>

<a id="sb-l-c-ex-1"></a>

### SB-L-C-EX-1 — Exception message structure

`Stylebrella-linter-cpp-exceptions-1`

A statically known standard exception message uses `[Source] Message: key=value, key=value`. Source is the enclosing `Class::method` or `namespace::function`, and a resolved path, field or enum failure requires its corresponding `path`, `field` or `type` key. Check known structure and keys even if a value is computed. Do not invent message text.

**Allowed**:

```cpp
throw std::runtime_error("[Loader::load] Cannot read file: path=" + path.string());
```

**Violation** (confirmed file failure):

```cpp
throw std::runtime_error("Cannot read file");
```

<a id="stylebrella-linter-cpp-exceptions-2"></a>

<a id="sb-l-c-ex-2"></a>

### SB-L-C-EX-2 — Exception type by failure category

`Stylebrella-linter-cpp-exceptions-2`

For a confirmed failure category, use `std::invalid_argument` for invalid caller input or enum values, `std::runtime_error` for I/O or environment failures, and `std::logic_error` for invalid internal state or API call order. An unknown category does not trigger a type finding.

**Allowed** (invalid input):

```cpp
throw std::invalid_argument(message);
```

**Violation** (invalid input):

```cpp
throw std::runtime_error(message);
```

<a id="stylebrella-linter-cpp-exceptions-3"></a>

<a id="sb-l-c-ex-3"></a>

### SB-L-C-EX-3 — ASCII console messages

`Stylebrella-linter-cpp-exceptions-3`

Require ASCII in the statically known text of a resolved console message. Do not guess the language of arbitrary strings or the contents of dynamic values.

**Allowed**:

```cpp
std::cout << "Loaded";
```

**Violation**:

```cpp
std::cout << "Загружено";
```

## Recommendations (no diagnostic)

Use `std::unique_ptr` for exclusive ownership, `std::shared_ptr` only for shared ownership, and `std::weak_ptr` to break shared cycles. Review lifetime and scope before removing a standalone block. Use `final` when extending a class is intentionally prohibited. Explain complex algorithms and type layout in `@details`. Use English for console messages. Review exception classes when the failure category cannot be proven.
