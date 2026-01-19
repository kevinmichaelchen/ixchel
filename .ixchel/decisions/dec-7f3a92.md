---
id: dec-7f3a92
type: decision
title: 'ADR-005: Git Hooks Tooling Migration'
status: accepted
date: 2026-01-19
created_at: 2026-01-19T00:00:00Z
updated_at: 2026-01-19T00:00:00Z
created_by: Kevin Chen
tags:
  - dx
  - git-hooks
  - tooling
  - ci
---

# ADR-005: Git Hooks Tooling Migration

**Status:** Accepted
**Date:** 2026-01-19
**Deciders:** Kevin Chen
**Tags:** dx, git-hooks, tooling, ci

## Context and Problem Statement

We currently use [Husky](https://github.com/typicode/husky) for Git hooks management. While Husky is the de facto standard in the JavaScript ecosystem, we're evaluating modern Rust-based alternatives that may offer better performance, tighter linter integration, and reduced maintenance overhead for our polyglot Rust monorepo.

Two leading candidates have emerged:

1. **[hk](https://github.com/jdx/hk)** (583 stars) - Git hooks and project linting tool by jdx (mise author)
2. **[prek](https://github.com/j178/prek)** (3.7k stars) - Rust reimplementation of pre-commit framework

## Decision Drivers

1. **Performance** - Fast hook execution, especially for large changesets
2. **Linter integration** - Native support for formatters/linters used in the repo
3. **Configuration ergonomics** - Simple, maintainable config format
4. **Ecosystem fit** - Alignment with Rust toolchain and monorepo structure
5. **Adoption & maintenance** - Active development, community adoption
6. **Migration effort** - Complexity of switching from Husky

## Research Findings

### Current: Husky

| Aspect        | Details                                            |
| ------------- | -------------------------------------------------- |
| **Size**      | ~2 kB gzipped, no dependencies                     |
| **Startup**   | ~1 ms (native Git core.hooksPath)                  |
| **Config**    | Shell scripts in `.husky/` directory               |
| **Linting**   | Generic - users invoke linters manually in scripts |
| **Ecosystem** | JavaScript/npm standard                            |

**Pros:**

- Extremely lightweight and fast startup
- Cross-platform (macOS, Linux, Windows)
- Simple mental model (just shell scripts)
- Supports all 13 client-side Git hooks

**Cons:**

- No built-in linter abstraction or parallelism
- Shell script maintenance burden
- No automatic file-filtering or staging logic
- JS-centric; feels foreign in Rust repos

---

### Option A: hk (jdx/hk)

| Aspect              | Details                                          |
| ------------------- | ------------------------------------------------ |
| **Language**        | Rust (55.6%), Shell (33.5%), Pkl (10.6%)         |
| **Config**          | `hk.pkl` (Pkl configuration language)            |
| **Performance**     | ~52 ms no-change hooks, ~95 ms staged-file hooks |
| **Git integration** | Direct libgit2 linking (no subprocess spawning)  |

**Key Features:**

- Tight built-in linter definitions (Prettier, Black, Ruff, etc.)
- Advanced parallelism with read/write file locks (prevents race conditions)
- Automatic stashing of unstaged changes before fix steps
- Both `check` and `fix` commands per step
- Per-linter file-glob configuration
- Local override via `hk.local.pkl`

**Benchmarks (MacBook Pro M3):**

| Scenario     | hk    | Lefthook |
| ------------ | ----- | -------- |
| No changes   | 52 ms | 70 ms    |
| Staged files | 95 ms | 155 ms   |

**Pros:**

- Purpose-built for linting workflows
- Race-condition-safe parallelism
- libgit2 integration eliminates git subprocess overhead
- From jdx (author of mise) - active maintainer
- Pkl config is expressive and type-safe

**Cons:**

- Smaller community (583 stars)
- Pkl is a new config language to learn
- Less ecosystem momentum than pre-commit family
- Tightly coupled to specific linter abstractions

---

### Option B: prek (j178/prek)

| Aspect           | Details                                        |
| ---------------- | ---------------------------------------------- |
| **Language**     | Rust (single binary)                           |
| **Config**       | `.pre-commit-config.yaml` (drop-in compatible) |
| **Performance**  | Up to 5× faster than Python pre-commit         |
| **Dependencies** | Zero runtime dependencies                      |

**Key Features:**

- Drop-in replacement for pre-commit
- Full `.pre-commit-config.yaml` compatibility
- Built-in monorepo/workspace support (`prek workspace`)
- Native Rust implementations of common hooks
- Integrated toolchain management (Python via uv, Node.js, Go, Ruby)
- `repo: builtin` for offline, zero-setup hooks
- Auto-update with cooldown for supply chain safety

**Notable Adopters:** CPython, Apache Airflow, FastAPI, PDM, Ruff, Home Assistant

**Installation Options:** cargo-binstall, Homebrew, pip/uv, npm, Nix, standalone script

**Pros:**

- Large community (3.7k stars, major project adoption)
- Zero migration friction if already using pre-commit
- Workspace/monorepo support out of the box
- Familiar YAML config (no new language to learn)
- Excellent toolchain management built-in
- Active development with frequent releases

**Cons:**

- pre-commit model may be overkill for simple hooks
- Hook ecosystem requires network/cache for external repos
- Less tight linter integration than hk (generic runner)

---

## Comparison Matrix

| Criteria               | Husky         | hk                   | prek                       |
| ---------------------- | ------------- | -------------------- | -------------------------- |
| **Performance**        | ~1 ms startup | 52-95 ms             | ~5× faster than pre-commit |
| **Parallelism**        | Manual        | Built-in with locks  | Built-in by priority       |
| **Linter integration** | None          | Excellent (built-in) | Good (via hooks)           |
| **Config format**      | Shell scripts | Pkl                  | YAML                       |
| **Learning curve**     | Low           | Medium (Pkl)         | Low (if know pre-commit)   |
| **Monorepo support**   | Basic         | Unknown              | Excellent                  |
| **Community**          | Massive (JS)  | Growing (583★)       | Strong (3.7k★)             |
| **Migration effort**   | N/A           | High (new config)    | Low-Medium                 |
| **Rust ecosystem fit** | Poor          | Excellent            | Good                       |

## Considered Options

### Option 1: Stay with Husky

Keep current setup. Minimal effort but foregoes performance and integration benefits.

### Option 2: Migrate to hk

Best for: Teams wanting tight linter integration and maximum performance with explicit control over each linter's behavior.

**Migration steps:**

1. Install hk (`cargo binstall hk`)
2. Create `hk.pkl` with linter definitions
3. Run `hk install` to set up Git hooks
4. Remove `.husky/` directory

### Option 3: Migrate to prek

Best for: Teams already familiar with pre-commit or wanting a well-adopted, zero-dependency solution with monorepo support.

**Migration steps:**

1. Install prek (`cargo binstall prek`)
2. Create `.pre-commit-config.yaml` (or migrate existing)
3. Run `prek install`
4. Remove `.husky/` directory

### Option 4: Hybrid (prek for hooks, hk for project linting)

Use prek for Git hooks and hk's `hk lint` for ad-hoc project-wide linting.

## Recommendation

**Leaning toward Option 3 (prek)** for the following reasons:

1. **Community validation** - Adopted by CPython, FastAPI, Ruff, and other major projects
2. **Zero config migration** - If we adopt pre-commit YAML, config is portable
3. **Monorepo-native** - Built-in workspace support aligns with our structure
4. **Lower learning curve** - YAML vs Pkl; pre-commit is a known quantity
5. **Toolchain management** - Built-in Python/Node management via uv

However, **hk deserves consideration** if:

- We want tighter control over linter fix/check modes
- Performance benchmarks matter more than ecosystem adoption
- We're comfortable with Pkl configuration

## Open Questions

1. Do we have existing `.pre-commit-config.yaml` to migrate, or starting fresh?
2. What linters/formatters do we need to support? (rustfmt, clippy, prettier, etc.)
3. Is monorepo workspace support a hard requirement?
4. How important is the Pkl type-safety vs YAML familiarity?

## Decision

**Accepted** - Migrate to prek (Option 3).

Rationale:

- Large community adoption (CPython, FastAPI, Ruff)
- Zero-dependency single binary
- Familiar YAML config (no Pkl learning curve)
- Monorepo/workspace support aligns with our structure
- Drop-in compatible with pre-commit ecosystem

## References

- [hk documentation](https://hk.jdx.dev/)
- [hk vs Lefthook benchmark](https://t.holmium.no/dia/hk-vs-lefthook)
- [prek documentation](https://prek.j178.dev/)
- [prek GitHub](https://github.com/j178/prek)
- [Husky GitHub](https://github.com/typicode/husky)
- [pre-commit framework](https://pre-commit.com/)
