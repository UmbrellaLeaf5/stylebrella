# C++ formatting

[Русская версия](../../../ru/code-style/cpp/formatting.md)

This page adds C++-specific rules to [shared formatting](../formatting.md). It covers `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, and `.hh`. See [C++ linting](linting.md). Never run clang-format on `CMakeLists.txt`, `*.cmake`, or standalone data/config files.

## Layout

Use clang-format with the Google base style, two-space continuation indent, left-aligned arrays of structures, non-assignment binary operators broken before the operator, constructor initializer breaks before the colon, and no comment reflow. Inline short functions and unbraced short `if`/`for` statements are permitted where they fit; do not force braces or change control flow. Use the actual source filename to resolve `.clang-format`, including for stdin. The layout fixtures define the desired result even when further engine compatibility work is needed.

Before:

```cpp
void apply(const std::string& resolved_configuration_path, const Settings& resolved_settings, Logger& event_logger) {
    load(resolved_configuration_path,resolved_settings,event_logger);
}
```

After:

```cpp
void apply(
  const std::string& resolved_configuration_path,
  const Settings& resolved_settings,
  Logger& event_logger) {
  load(resolved_configuration_path, resolved_settings, event_logger);
}
```

## Blank lines

- Namespace-scope declarations inside a namespace block use the one-blank-line rule. Keep `else if`, `else`, and `catch` on their own line.
- Contiguous field assignments such as `signal.id = …` and `signal.freq = …` stay with `Signal signal;` under the shared same-receiver rule, including when they populate different fields.

Before:

```cpp
announce();
Signal signal;

signal.id = id;
signal.freq = freq;
if (ready) { send(signal); } else { retry(signal); }
```

After:

```cpp
announce();

Signal signal;
signal.id = id;
signal.freq = freq;

if (ready) { send(signal); }

else { retry(signal); }
```

## Separators and private helpers

Use `//` for the shared separators and structural comments in headers and sources. Keep Doxygen directly attached to the declaration and leave side comments on the same line as their field. [C++ linting](linting.md#marks-method-order-and-documentation) defines which methods count as private.

```cpp
class Reader {
 public:
  Result read();

  // (no private methods)
};
```
