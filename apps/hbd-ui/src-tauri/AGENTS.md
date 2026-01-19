# HBD-UI TAURI AGENTS

**Parent:** See `../../../AGENTS.md` for workspace context and `../README.md` for
frontend details.

## Overview

Rust Tauri shell for the hbd UI. The frontend lives in `apps/hbd-ui/src/` and uses
Svelte; this crate wires the desktop app entrypoint.

## Structure

```
apps/hbd-ui/src-tauri/
├── src/
│   ├── main.rs            # Tauri entrypoint
│   └── lib.rs             # Tauri builder setup
├── tauri.conf.json        # Bundling/runtime config
├── build.rs               # Build-time hooks
└── icons/                 # App icons
```

## Where To Look

| Task                 | Location                                          |
| -------------------- | ------------------------------------------------- |
| Tauri initialization | `apps/hbd-ui/src-tauri/src/lib.rs`                |
| Desktop entrypoint   | `apps/hbd-ui/src-tauri/src/main.rs`               |
| App config/bundling  | `apps/hbd-ui/src-tauri/tauri.conf.json`           |
| Frontend behavior    | `apps/hbd-ui/src/*` (see `apps/hbd-ui/README.md`) |
