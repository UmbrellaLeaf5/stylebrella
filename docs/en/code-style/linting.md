# Shared linting

[Русская версия](../../ru/code-style/linting.md)

Target contract for the `template` preset. `lint` reports the rules here and in the language pages: [Python](python/linting.md), [Kotlin](kotlin/linting.md), [C++](cpp/linting.md). `check` also verifies [formatting](formatting.md). Examples isolate one rule; other diagnostics may apply to the same excerpt. **Review** requirements are normative but produce no diagnostic unless the stated condition is mechanically established. A rule's presence here does not mean it is implemented.

## Declaration names

Check names by declaration category, including local bindings, parameters, fields, routines, types, and constants, using the case rules in the language pages. Do not guess a symbol's purpose from the spelling alone. Prefer descriptive names, with units when relevant; name quality is a review requirement. Report the declaration, without auto-renaming it or changing external APIs.

| Language | Pass                         | Report                      |
| -------- | ---------------------------- | --------------------------- |
| Python   | `total_count = count()`      | `totalCount = count()`      |
| Kotlin   | `val totalCount = count()`   | `val total_count = count()` |
| C++      | `int total_count = count();` | `int totalCount = count();` |

## JSON keys

Check keys where a serialization field or JSON access is statically identifiable; use the language page's casing and declaration rules. Do not treat arbitrary strings as JSON keys or rewrite wire names. A separately valid schema key may still need a named constant at use sites.

| Language                    | Valid key                  | Report                    |
| --------------------------- | -------------------------- | ------------------------- |
| Python (snake-case project) | `"user_id"`                | `"User-ID"`               |
| Kotlin                      | `@JsonProperty("user_id")` | `@JsonProperty("userId")` |
| C++                         | `"user_id"`                | `"userId"`                |

## MARK text and importance

Use readable English text starting with a capital letter or in ALL CAPS. Reject empty/lowercase titles and titles that are merely a camelCase or PascalCase declaration identifier. Full prose quality is reviewed. For endpoints, the HTTP verb and path can be the title. A function is non-trivial if executable content spans multiple lines, has multiple executable statements, or contains nested control flow. Count executable lines after canonical formatting so line wrapping cannot be used to evade lint. A method/function is important if used at two or more distinct resolved project sites, even in one caller; exclude self-recursion, comments and unresolved occurrences. Require a descriptive MARK for non-trivial public routines and important routines; a valid group may cover them. Trivial, non-important routines can use a separator alone.

| Language | Pass (above the declaration) | Report if attached to the same declaration |
| -------- | ---------------------------- | ------------------------------------------ |
| Python   | `# MARK: Build response`     | `# MARK: buildResponse`                    |
| Kotlin   | `// MARK: Build response`    | `// MARK: buildResponse`                   |
| C++      | `// MARK: Build response`    | `// MARK: BuildResponse`                   |

## MARK groups and adjacency

Groups are optional. `MARK: <title> [group:N]` covers exactly the next `N` peer function/method declarations in the current scope, with `N` a decimal integer at least 2. Nested definitions and non-function declarations do not count. Report a group interrupted by another MARK, the end of the scope before the count is met, or an invalid count; the next declaration after `N` is outside the group. Reject consecutive MARKs at the same level with no intervening declaration or substantive statement; blank lines and separator comments do not break adjacency. `Private Helpers` is a structural header without a count: subsequent descriptive MARKs do not end that section. Do not stack a descriptive MARK immediately after the structural header; a single counted group cannot stand in for the distinct public marker and helpers section.

The structural header satisfies the MARK requirement for the first private helper. If that helper is important, require its language's documentation block instead of a second MARK. A documentation block does not break MARK adjacency.

| Language | Pass                                                              | Report                                                        |
| -------- | ----------------------------------------------------------------- | ------------------------------------------------------------- |
| Python   | `# MARK: Load adapters [group:2]` followed by two peer functions  | same header followed by just one before the scope ends        |
| Kotlin   | `// MARK: Load adapters [group:2]` followed by two peer methods   | same header followed by another MARK before the second method |
| C++      | `// MARK: Load adapters [group:2]` followed by two peer functions | `// MARK: Load adapters [group:1]`                            |

| Language | Pass for an important first helper                                       | Report                                                                        |
| -------- | ------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| Python   | `# MARK: Private Helpers`, separator, `_read` with a multiline docstring | `_read` has no docstring, or `# MARK: Read input` is stacked under the header |
| Kotlin   | `// MARK: Private Helpers`, separator, KDoc, `private fun readInput`     | missing KDoc or a stacked `// MARK: Read input`                               |
| C++      | `// MARK: Private Helpers`, separator, Doxygen, private `_read`          | missing Doxygen or a stacked `// MARK: Read input`                            |

## Class section coverage

For an applicable concrete class with public methods and private helpers, require at least two MARKs: one associated with a public method (selected by the author if none is automatically important) and `MARK: Private Helpers`. A class with no private methods uses the formatter's placeholder instead; do not require an empty helpers MARK. Exempt interfaces and DTO-only declarations from the minimum.

| Language | Pass, in member order                                                                  | Report                                               |
| -------- | -------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| Python   | `# MARK: Read input`, `read`, `# MARK: Private Helpers`, `_read`                       | public `read` has only a separator in the same class |
| Kotlin   | `// MARK: Read input`, `fun read`, `// MARK: Private Helpers`, `private fun readInput` | helpers header is the only MARK                      |
| C++      | `// MARK: Read input`, public `read`, `// MARK: Private Helpers`, private `_read`      | both MARKs describe private methods                  |

## Private method order

Private methods must come after all public methods in applicable classes. Determine visibility from syntax or language conventions; do not infer it from usage. Diagnose the out-of-order declaration. Do not move declarations automatically.

| Language | Pass                                    | Report                                  |
| -------- | --------------------------------------- | --------------------------------------- |
| Python   | `read` then `_read`                     | `_read` then `read`                     |
| Kotlin   | `fun read` then `private fun readInput` | `private fun readInput` then `fun read` |
| C++      | `public: read` then `private: _read`    | `private: _read` then `public: read`    |

## Side comments and comment language

Require side comments for fields listed by each language page, except the page's explicit self-explanatory names. A comment must explain meaning and units where relevant; check presence and leave good prose and correct units to review. MARK text is English. Ordinary comments may be English or Russian, with Russian preferred for new or modified comments. Language-specific rules govern docstrings, KDoc, and Doxygen.

| Language | Pass                                                | Report                                       |
| -------- | --------------------------------------------------- | -------------------------------------------- |
| Python   | `df_hz: float = 0.0  # offset from center, Hz`      | same dataclass field without its comment     |
| Kotlin   | `val dfHz: Double = 0.0, // offset from center, Hz` | same data-class property without its comment |
| C++      | `double df_hz = 0.0; // offset from center, Hz`     | same struct field without its comment        |

## Constants and inline literals

Use one named definition per shared domain value in the language's designated constants container. Require a named value for otherwise unclassified inline data, including default domain values, paths, regexes, conversion factors, precision/sentinel values and test inputs. Permit `0`, `1`, and `""` for trivial initialization (including equivalent typed zero/one), and allow language booleans and null markers. Permit inline message **text** in resolved printing, logging and exception message positions, including interpolated text; that exemption does not extend to domain values supplied as arguments. Recognize only the explicitly documented syntax/schema/metadata exceptions, not arbitrary call names or strings. Diagnose the literal; do not extract it automatically or assume equal values have equal meanings.

| Language | Pass                                                                          | Report           |
| -------- | ----------------------------------------------------------------------------- | ---------------- |
| Python   | `connect(Constants.Network.DEFAULT_PORT)`; `raise ValueError("Invalid port")` | `connect(8080)`  |
| Kotlin   | `connect(Constants.Network.DEFAULT_PORT)`; `logger.warn("Invalid port")`      | `connect(8080)`  |
| C++      | `connect(app::constants::network::DEFAULT_PORT);`                             | `connect(8080);` |

## Loop intent

Prefer `for` for iteration over a known range or collection; use `while` for state-driven or complex termination. Report only recognizable counted loops equivalent to ordinary iteration; review cases where updates, side effects, or termination are unclear. A formatter never changes the loop kind.

| Language | Pass                                          | Candidate for a diagnostic                                                                         |
| -------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Python   | `for item in items: visit(item)`              | `i = 0; while i < len(items)` with only a normal `i += 1` step                                     |
| Kotlin   | `for (item in items) visit(item)`             | `while (i < items.size)` with an index initialized to zero and incremented once on every iteration |
| C++      | `for (const auto& item : items) visit(item);` | `while (i < items.size())` with the same counted-loop pattern                                      |
