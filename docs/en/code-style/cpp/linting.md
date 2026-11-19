# C++ linting

[Русская версия](../../../ru/code-style/cpp/linting.md)

Target contract for `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, and `.hh`, alongside [shared linting](../linting.md) and [C++ formatting](formatting.md). Use clang-tidy for applicable semantic rules. When `compile_commands.json` is missing, report skipped semantic coverage; independent file and syntax rules still run. Do not lint CMake as C++.

## Names and file structure

Use `PascalCase` for classes/structs/enums; `snake_case` for functions, methods, variables, fields and parameters. Private fields and methods use one leading underscore (`_gain_max_dbi`); do not produce reserved identifiers. Constants declared `const`/`constexpr`, including local constants, use `SCREAMING_SNAKE_CASE`; this does not apply to cv qualification of a member function, a pointee/referent, or a read-only parameter. Namespaces use lowercase. Resolved JSON keys use `snake_case` (e.g. `"center_freq_hz"`). Private naming and constant naming intersect: keep the private prefix for private member constants while using uppercase for the base name.

Check the namespace directory, one primary type per file, matching PascalCase file stem for single-type files, and snake_case stems for utility files. Use `.hpp` for project-owned headers except explicit legacy/third-party exclusions; a header must use `#pragma once` and not a macro include guard. Use `shared/` for types used across classes and `components/` for types used by a single class when references establish coverage; review incomplete graphs. Keep declarations and implementations in the project's `include/` and `src/` trees respectively.

| Pass                                                            | Report                                                            |
| --------------------------------------------------------------- | ----------------------------------------------------------------- |
| `include/app/core/Antenna.hpp` declaring `core::Antenna`        | `include/app/math/antenna.h` for the same project-owned type      |
| `void make_signal();`, `int _gain_max_dbi;` as a private member | `void MakeSignal();`, `int gain_max_dbi;` as a private member     |
| `constexpr int MAX_RETRIES = 3;` in the constants namespace     | `constexpr int maxRetries = 3;`                                   |
| `#pragma once` in a project-owned header                        | Missing pragma or a traditional `#ifndef`/`#define` include guard |

## Language usage and ownership

- No C-style casts. Use `static_cast`, `dynamic_cast`, `const_cast` or `reinterpret_cast` as warranted. E.g. `static_cast<double>(count)` passes, `(double)count` is reported. Use `nullptr`, not `NULL` or `0` in resolved pointer contexts.
- Use `auto` for iterators, complex result types and suitable lambda captures/returns. Prefer explicit fundamental types where not obvious; flag forms such as `auto count = 5;` rather than guessing type readability in arbitrary cases.
- Use `const` for locals that are not modified (check resolved mutations and escape), `constexpr` for compile-time constants, and mark methods `const` when known not to modify object state and compatible with their signatures. Require `override` for actual overrides. Use `final` for classes intended not to be derived from; absence of subclasses alone does not establish intent.
- Prefer `std::unique_ptr` for exclusive ownership, `std::shared_ptr` only for actual shared ownership, and `std::weak_ptr` to break shared cycles. Review ownership and cycles where unresolved; do not automatically change pointer types.
- Prefer `for` per [the shared loop rule](../linting.md#loop-intent). Reject bare standalone compound statements used only to group code. Do not confuse those with a control-flow/function/namespace/class body or initializer; review blocks required for scope or lifetime.

```cpp
class Loader final : public Input {
 public:
  Result read() const override;
};

auto position = items.begin();  // iterator
auto loader = std::make_unique<Loader>();  // exclusive ownership
```

## MARKs, method order, and documentation

Apply [shared MARK and private method rules](../linting.md#mark-text-and-importance). Respect default `class`/`struct` access and access sections; protected methods are not private. For struct fields check a side comment for meaning and units, except `id` and `name`. Use separators and MARKs in `.cpp` sections as well as headers.

Every function, class, struct and enum declaration in `.hpp` needs multiline Doxygen directly above it. Also require it for an important first private helper covered by `Private Helpers` when it has no documented declaration. Put `/**` and `*/` on separate lines, use `*` before text and ` *` on a blank line, and start with `@brief`. The brief describes what a function does or what a type is. Add `@details` for complex algorithms/design, `@param name description` in signature order, and `@return description` for non-void results. Use `@throws Type description` when documenting a known exception. Use no colon suffix after parameter names and no `Type:` prefix in return prose; omit inapplicable tags. The long-function threshold (over 15 nonblank, non-comment executable lines) or a dedicated MARK can identify functions needing review of the detail section. Prose must be Russian; ignore tags, symbols and code when checking language. Report missing blocks, malformed required tags and single-line documentation (`///` or `/** @brief ... */`).

```cpp
// MARK: Read settings
// --------------------------------------------------

/**
 * @brief Загружает настройки из файла.
 * @param path путь к файлу настроек
 * @return прочитанные настройки
 */
Settings read_settings(const std::filesystem::path& path);
```

## Constants, exceptions, and console output

Put shared values in a dedicated header (e.g. `constants.hpp`) under a project constants namespace. Use categories such as `math`, `entity`, `config`, `validation`, `pattern`, and fully qualified references such as `app::constants::config::CENTER_FREQ_HZ`. Do not inline shared numeric values, paths, patterns or defaults. Apply [the shared literal rule](../linting.md#constants-and-inline-literals).

Standard exceptions use `[Source] Message: key=value, key=value`, where `Source` is `Class::method` or `namespace::function`. Include the failing `path`, `field`, or enum `type` as appropriate. Use `std::invalid_argument` for bad input and enums, `std::runtime_error` for I/O/environment failures, and `std::logic_error` for invalid state/order of API calls. Check statically known message shape, source and keys; use a known failure category for type diagnostics, otherwise review. Exception/log/print text can be inline. Console output must be English and ASCII; check literal ASCII and review meaning.

```cpp
throw std::runtime_error("[Loader::load] Cannot read file: path=" + path.string());
```

Report `throw std::runtime_error("Cannot read file");` for missing source and context at a recognized file failure; do not require a named constant just for that message text.
