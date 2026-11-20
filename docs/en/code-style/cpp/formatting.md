# Formatting C++

[Русская версия](../../../ru/code-style/cpp/formatting.md)

Scope: `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, and `.hh`; never CMake or standalone data/configuration files. Locate the applicable formatting configuration using the real source path, including for stdin. See [C++ linting](linting.md) for member visibility.

[Shared rules](../general/formatting.md)

Examples isolate the rule being illustrated.

## `layout`

<a id="stylebrella-formatter-cpp-layout-1"></a>

<a id="sb-f-c-ly-1"></a>

### SB-F-C-LY-1 — Baseline layout and continuation indentation

`Stylebrella-formatter-cpp-layout-1`

Use Google C++ layout as the baseline, with the exceptions on this page and in the shared contract. Continuation indentation adds two spaces relative to the construct’s line.

**Before**:

```cpp
void process(
    const Request& incoming_request,
    const Configuration& processing_configuration,
    ResultCollector& result_collector);
```

**After**:

```cpp
void process(
  const Request& incoming_request,
  const Configuration& processing_configuration,
  ResultCollector& result_collector);
```

<a id="stylebrella-formatter-cpp-layout-2"></a>

<a id="sb-f-c-ly-2"></a>

### SB-F-C-LY-2 — Array-of-structs alignment

`Stylebrella-formatter-cpp-layout-2`

Align columns of a multiline array-of-structs initializer to the left when each row has the same number of elements.

**Before**:

```cpp
Pair pairs[] = {
  {1, 200},
  {20, 3},
};
```

**After**:

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

**Before**:

```cpp
const int total = left +
  right;
```

**After**:

```cpp
const int total = left
  + right;
```

<a id="stylebrella-formatter-cpp-layout-4"></a>

<a id="sb-f-c-ly-4"></a>

### SB-F-C-LY-4 — Wrapping constructor initializer lists

`Stylebrella-formatter-cpp-layout-4`

When a constructor initializer list wraps, break before the colon.

**Before**:

```cpp
C::C() :
  x_(x) {}
```

**After**:

```cpp
C::C()
  : x_(x) {}
```

<a id="stylebrella-formatter-cpp-layout-5"></a>

<a id="sb-f-c-ly-5"></a>

### SB-F-C-LY-5 — Preserving comment lines

`Stylebrella-formatter-cpp-layout-5`

Do not reflow comment text. Keep a field’s trailing comment on its declaration line even if the line exceeds the width target.

**Before**:

```cpp
int count;  // Number of bytes in the payload before compression.
```

**After**:

```cpp
int count;  // Number of bytes in the payload before compression.
```

<a id="stylebrella-formatter-cpp-layout-6"></a>

<a id="sb-f-c-ly-6"></a>

### SB-F-C-LY-6 — Short functions and unbraced control bodies

`Stylebrella-formatter-cpp-layout-6`

Allow inline member functions and short unbraced `if` statements without `else` to occupy one line when they fit, subject to the nested-control layout in [SB-F-G-LY-3](../general/formatting.md#stylebrella-formatter-general-layout-3). Do not force braces around short unbraced `if` or `for` bodies. The shared rule may remove braces from eligible single-statement bodies.

**Before**:

```cpp
if (ready)
  send();
```

**After**:

```cpp
if (ready) send();
```

## `imports`

<a id="stylebrella-formatter-cpp-imports-1"></a>

<a id="sb-f-c-i-1"></a>

### SB-F-C-I-1 — Include categories and sorting

`Stylebrella-formatter-cpp-imports-1`

Regroup contiguous leading includes and order them by category, then case-sensitively by header spelling: matching main header (including `-test`, `_test`, `-unittest`, `_unittest` source suffixes), angle-bracket `.h` except `ext/`, remaining angle-bracket headers, then quoted headers. Use one blank line between nonempty categories. Standalone grouping comments and preprocessor branch boundaries split sortable runs; do not move late includes.

**Before**:

```cpp
#include "other.hpp"
#include <vector>
#include <stdio.h>
```

**After**:

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

Use one blank line between declaration blocks inside a namespace, including around separators; keep attached Doxygen with its declaration. Consecutive assignments to different fields of the same newly declared object stay in its declaration group.

**Before**:

```cpp
namespace n {
void a();
// --------------------------------------------------
void b();
}  // namespace n
```

**After**:

```cpp
namespace n {
void a();

// --------------------------------------------------

void b();
}  // namespace n
```

<a id="stylebrella-formatter-cpp-blank_lines-2"></a>

<a id="sb-f-c-bl-2"></a>

### SB-F-C-BL-2 — Line breaks before else and catch

`Stylebrella-formatter-cpp-blank_lines-2`

Start `else if`, `else`, and `catch` on a new line; apply [SB-F-G-BL-7](../general/formatting.md#stylebrella-formatter-general-blank_lines-7) to the branch boundary.

**Before**:

```cpp
if (x) {
  f();
  g();
} else {
  h();
  i();
}
```

**After**:

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
