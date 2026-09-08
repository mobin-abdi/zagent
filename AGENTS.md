# ZAgent Instructions

## General Rules

- Before modifying files, inspect the relevant code first.
- Do not delete files unless explicitly requested.
- Do not modify unrelated files.
- Do not expose API keys, passwords, tokens, or secrets.
- Do not change project dependencies unless necessary.
- After making changes, verify the result.
- Prefer small, focused changes.

## Rust Rules

- Run `cargo check` after Rust code changes.
- Follow the existing project structure.
- Do not rewrite working code without a reason.

## Git Rules

- Do not run destructive git commands.
- Do not reset or discard user changes.
- Do not create commits unless explicitly requested.
