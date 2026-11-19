# Repository instructions

## Bilingual documentation

- Keep the English (`docs/en/`) and Russian (`docs/ru/`) documentation **fully synchronized**. Every page must have a counterpart at the same relative path in the other language, with a reciprocal language switch link.
- Add, edit, rename, or remove both versions in the same change. Keep all rules, exceptions, examples, commands, tables, links, and implementation-status notes equivalent. Do not leave placeholders, omissions, or updates that appear in only one language.
- Before finishing a documentation change, compare both versions and verify relative links and heading anchors, including links from the repository README when applicable.
- Every formatter **Before** example must violate only the formatting rule being illustrated; its paired **After** example must violate no formatting rule in the documentation. Check interactions with all shared and language-specific formatter rules, not just the local section.
- In linter sections, every **Violation** example must violate only the lint rule being illustrated, and every **Allowed** example must violate no lint rule in the documentation. Apply this isolation requirement to linter examples only against the lint rules; formatting examples are governed by the formatter requirement above.
- Audit all documentation examples affected by a changed rule before finishing. Keep paired examples equivalent in context and semantics where relevant, and update both language versions together.
