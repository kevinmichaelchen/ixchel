# HBD-UI TAURI AGENTS

**Parent:** See `../../AGENTS.md` for workspace context and `../README.md` for
frontend details.

## Overview

Rust Tauri shell for the hbd UI. The frontend lives in `hbd-ui/src/` and uses
Svelte; this crate wires the desktop app entrypoint.

## Structure

```
hbd-ui/src-tauri/
├── src/
│   ├── main.rs            # Tauri entrypoint
│   └── lib.rs             # Tauri builder setup
├── tauri.conf.json        # Bundling/runtime config
├── build.rs               # Build-time hooks
└── icons/                 # App icons
```

## Where To Look

| Task                 | Location                                |
| -------------------- | --------------------------------------- |
| Tauri initialization | `hbd-ui/src-tauri/src/lib.rs`           |
| Desktop entrypoint   | `hbd-ui/src-tauri/src/main.rs`          |
| App config/bundling  | `hbd-ui/src-tauri/tauri.conf.json`      |
| Frontend behavior    | `hbd-ui/src/*` (see `hbd-ui/README.md`) |
