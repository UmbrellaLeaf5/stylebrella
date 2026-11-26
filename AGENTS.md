# Repository instructions

## CLI command scope

- `format` applies the formatting rules and rewrites source files as needed. Apply the full set of formatting rules, including structural formatting changes explicitly required by the formatting specification.
- `check` is the read-only counterpart of `format`. Use the same formatting rules to report where source differs from the expected formatting. Do not write changes or run lint rules.
- `lint` is read-only and reports violations of the separate linting rules only. Do not run formatting checks, apply fixes, or rewrite sources.
- Keep this separation consistent in CLI behavior, help text, tests, and documentation. In particular, do not describe `check` as a combined formatting and linting command.

## Bilingual documentation

- Keep the English (`docs/en/`) and Russian (`docs/ru/`) documentation **fully synchronized**. Every page must have a counterpart at the same relative path in the other language, with a reciprocal language switch link.
- Add, edit, rename, or remove both versions in the same change. Keep all rules, exceptions, examples, commands, tables, links, and implementation-status notes equivalent. Do not leave placeholders, omissions, or updates that appear in only one language.
- Before finishing a documentation change, compare both versions and verify relative links and heading anchors, including links from the repository README when applicable.
- Every formatter **Before** example must violate only the formatting rule being illustrated; its paired **After** example must violate no formatting rule in the documentation. Check interactions with all shared and language-specific formatter rules, not just the local section.
- In linter sections, every **Violation** example must violate only the lint rule being illustrated, and every **Allowed** example must violate no lint rule in the documentation. Apply this isolation requirement to linter examples only against the lint rules; formatting examples are governed by the formatter requirement above.
- Write every linter example using this exact structure, with no language name or explanatory text combined with a status label: a standalone language heading (`Python`, `Kotlin`, or `C++`), then `**Allowed:**` followed by all applicable Allowed code fences consecutively, then `**Violation:**` followed by all applicable Violation code fences consecutively. Repeat the language heading and pair for each language. Put explanatory context in prose before the label, not in or alongside the label. Do not use combined headings such as `Python — Allowed / Violation` or `Allowed (Python, ...)`.
- Write every formatter example using this exact structure, with no language name or explanatory text combined with a state label: a standalone language heading (`Python`, `Kotlin`, or `C++`), then `**Before:**` followed by all applicable Before code fences consecutively, then `**After:**` followed by all applicable After code fences consecutively. Repeat the language heading and pair for each language. Put explanatory context in prose before the label, not in or alongside the label. Do not use labels such as `Before (CRLF)` or `Python: not applicable`; explicitly state non-applicability in prose where relevant.
- Examples must fully cover the rule's stated scope and applicable construct types; do not use examples that illustrate only a partial case. Explicitly mark genuinely non-applicable languages or constructs as not applicable.
- Audit all documentation examples affected by a changed rule before finishing. Keep paired examples equivalent in context and semantics where relevant, and update both language versions together.
