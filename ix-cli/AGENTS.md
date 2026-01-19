# IXCHEL AGENTS (ix-cli)

## Scope

Applies to the `ix-cli/` crate.

## Guidelines

- Keep CLI “thin”: argument parsing + formatting only.
- No direct adapter usage; call into `ix-core` (domain) and `ix-app` (wiring).

## Commands

```bash
cargo test -p ix-cli
dprint fmt
```
