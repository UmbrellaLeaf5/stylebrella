# Formatting C++

[Русская версия](../../../ru/code-style/cpp/formatting.md)

Scope: `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, and `.hh`; never CMake or standalone data/configuration files. Locate the applicable formatting configuration using the real source path, including for stdin. See [C++ linting](linting.md) for member visibility.

[Shared rules](../general/formatting.md)

Examples isolate the rule being illustrated.

## `layout`

<a id="stylebrella-formatter-cpp-layout-1"></a>

<a id="sb-f-c-ly-1"></a>

### SB-F-C-LY-1 — Google baseline and brace placement

`Stylebrella-formatter-cpp-layout-1`

Use Google C++ layout as the baseline, with the exceptions on this page and in the shared contract. Opening braces for function definitions stay on the declaration line. Continuation indentation is governed by the shared layout rules.

**C++**

**Before:**

```cpp
void process()
{
  prepare();
  execute();
}
```

**After:**

```cpp
void process() {
  prepare();
  execute();
}
```

<a id="stylebrella-formatter-cpp-layout-2"></a>

<a id="sb-f-c-ly-2"></a>

### SB-F-C-LY-2 — Array-of-structs alignment

`Stylebrella-formatter-cpp-layout-2`

Align columns of a multiline array-of-structs initializer to the left when each row has the same number of elements.

**C++**

**Before:**

```cpp
Pair pairs[] = {
  {1, 200},
  {20, 3},
};
```

**After:**

```cpp
Pair pairs[] = {
  {1,  200},
  {20, 3  },
};
```

<a id="stylebrella-formatter-cpp-layout-3"></a>

<a id="sb-f-c-ly-3"></a>

### SB-F-C-LY-3 — Wrapping binary operators

`Stylebrella-formatter-cpp-layout-3`

When wrapping a binary expression, put the operator at the start of its continuation line, except assignment operators, which stay on the previous line.

**C++**

**Before:**

```cpp
const int total = left +
  right;
```

**After:**

```cpp
const int total = left
  + right;
```

<a id="stylebrella-formatter-cpp-layout-4"></a>

<a id="sb-f-c-ly-4"></a>

### SB-F-C-LY-4 — Wrapping constructor initializer lists

`Stylebrella-formatter-cpp-layout-4`

When a constructor initializer list wraps, break before the colon.

**C++**

**Before:**

```cpp
C::C() :
  x_(x) {}
```

**After:**

```cpp
C::C()
  : x_(x) {}
```

<a id="stylebrella-formatter-cpp-layout-5"></a>

<a id="sb-f-c-ly-5"></a>

### SB-F-C-LY-5 — Inline member function layout

`Stylebrella-formatter-cpp-layout-5`

Collapse an inline member function whose body contains one statement to a single line when it fits within 100 columns. The shared rule governs unbraced `if` and `for` bodies.

**C++**

**Before:**

```cpp
class Sender {
 public:
  void send() const {
    notify();
  }
};
```

**After:**

```cpp
class Sender {
 public:
  void send() const { notify(); }
};
```

## `imports`

<a id="stylebrella-formatter-cpp-imports-1"></a>

<a id="sb-f-c-i-1"></a>

### SB-F-C-I-1 — Include categories and sorting

`Stylebrella-formatter-cpp-imports-1`

Regroup contiguous leading includes and order them by category, then case-sensitively by header spelling: matching main header (including `-test`, `_test`, `-unittest`, `_unittest` source suffixes), angle-bracket `.h` except `ext/`, remaining angle-bracket headers, then quoted headers. Use one blank line between nonempty categories. Standalone grouping comments and preprocessor branch boundaries split sortable runs; do not move late includes.

**C++**

**Before:**

```cpp
#include "other.hpp"
#include <vector>
#include <stdio.h>
```

**After:**

```cpp
#include <stdio.h>

#include <vector>

#include "other.hpp"
```

## `blank_lines`

<a id="stylebrella-formatter-cpp-blank_lines-1"></a>

<a id="sb-f-c-bl-1"></a>

### SB-F-C-BL-1 — Namespace spacing and field assignment groups

`Stylebrella-formatter-cpp-blank_lines-1`

Use exactly one blank line immediately after the opening `{` and immediately before the closing `}` of each namespace definition, including anonymous and nested namespaces. In an empty namespace, one blank line serves both boundaries. Keep the closing brace on its own line with a clang-format-style comment: `}  // namespace` for an anonymous namespace, `}  // namespace name` for a named namespace, and `}  // namespace app::detail` for a compact nested namespace such as `namespace app::detail`. Require the comment even for a short or empty namespace; add or correct it as needed.

Use one blank line between declaration blocks inside a namespace, including around separators; keep attached Doxygen with its declaration. Consecutive assignments to different fields of the same newly declared object stay in its declaration group.

**C++**

**Before:**

```cpp
namespace n {
void a();
// --------------------------------------------------
void b();
}  // namespace wrong
```

```cpp
namespace {
void a();
}
```

```cpp
namespace app::detail {
void a();
}
```

```cpp
namespace outer {
namespace inner {
void a();
}
}
```

```cpp
namespace empty {
}
```

**After:**

```cpp
namespace n {

void a();

// --------------------------------------------------

void b();

}  // namespace n
```

```cpp
namespace {

void a();

}  // namespace
```

```cpp
namespace app::detail {

void a();

}  // namespace app::detail
```

```cpp
namespace outer {

namespace inner {

void a();

}  // namespace inner

}  // namespace outer
```

```cpp
namespace empty {

}  // namespace empty
```

<a id="stylebrella-formatter-cpp-blank_lines-2"></a>

<a id="sb-f-c-bl-2"></a>

### SB-F-C-BL-2 — Line breaks before else and catch

`Stylebrella-formatter-cpp-blank_lines-2`

Start `else if`, `else`, and `catch` on a new line; apply [SB-F-G-BL-7](../general/formatting.md#stylebrella-formatter-general-blank_lines-7) to the branch boundary.

**C++**

**Before:**

```cpp
if (x) {
  f();
  g();
} else {
  h();
  i();
}
```

**After:**

```cpp
if (x) {
  f();
  g();
}

else {
  h();
  i();
}
```

## Recommendations (no diagnostic)

**Compatibility:** the reference Google-style configuration keeps at most one consecutive blank line. The Stylebrella preset needs `MaxEmptyLinesToKeep: 2` to preserve the file-scope spacing in [SB-F-G-BL-1](../general/formatting.md#stylebrella-formatter-general-blank_lines-1) and [SB-F-G-BL-2](../general/formatting.md#stylebrella-formatter-general-blank_lines-2). This setting alone does not establish compatibility with every rule.

Use standalone headings to keep intentionally distinct include blocks apart. Do not rely on include order to provide another header’s declarations implicitly.

```cpp
// plotting
#include "plot/Canvas.hpp"

// application state
#include "state/Session.hpp"
```
