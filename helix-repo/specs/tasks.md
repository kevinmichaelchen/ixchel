# Tasks

Implementation roadmap for `helix-repo`.

## Phase 1: Core Foundation

**Goal:** Minimal viable clone functionality

### Tasks

- [ ] **1.1** Project setup
  - [ ] Create Cargo.toml with dependencies
  - [ ] Add to workspace Cargo.toml
  - [ ] Set up basic module structure
  - [ ] Configure clippy and rustfmt

- [ ] **1.2** URL parsing module (`domain/url.rs`)
  - [ ] Implement `RepoUrl` struct
  - [ ] Parse HTTPS URLs
  - [ ] Parse SSH URLs (`git@host:path`)
  - [ ] Parse schemeless URLs
  - [ ] Strip `.git` suffix
  - [ ] Extract domain/owner/repo
  - [ ] Port test cases from git-grab

- [ ] **1.3** Path calculation (`path.rs`)
  - [ ] Implement `relative_path()` from RepoUrl
  - [ ] Handle edge cases (special chars, long paths)
  - [ ] Unit tests for path generation

- [ ] **1.4** Error types (`error.rs`)
  - [ ] Define `Error` enum with variants
  - [ ] Implement `exit_code()` method
  - [ ] Add Display/Error traits

---

## Phase 2: Git Operations

**Goal:** Shell out to git for cloning

### Tasks

- [ ] **2.1** GitClient trait (`ports/git.rs`)
  - [ ] Define `GitClient` trait
  - [ ] Define `CloneOptions` struct

- [ ] **2.2** GitCliClient adapter (`adapters/git_cli.rs`)
  - [ ] Implement `clone()` - shell out to git
  - [ ] Implement `is_repository()` - check .git exists
  - [ ] Implement `remote_url()` - get origin URL
  - [ ] Handle git exit codes
  - [ ] Stream output for progress

- [ ] **2.3** Clone options
  - [ ] Shallow clone (`--depth 1`)
  - [ ] Branch specification (`--branch`)
  - [ ] Submodule handling (`--recurse-submodules`)

---

## Phase 3: Repository Store

**Goal:** Discover and manage cloned repositories

### Tasks

- [ ] **3.1** RepositoryStore trait (`ports/store.rs`)
  - [ ] Define trait methods
  - [ ] Define `Repository` struct

- [ ] **3.2** FileSystemStore adapter (`adapters/fs_store.rs`)
  - [ ] Implement `list()` - scan directories
  - [ ] Implement `find()` - pattern matching
  - [ ] Implement `exists()` - path check
  - [ ] Implement `remove()` - delete directory
  - [ ] Clean up empty parent directories

- [ ] **3.3** Pattern matching
  - [ ] Match by `owner/repo`
  - [ ] Match by `repo` only
  - [ ] Match by `domain/owner/repo`
  - [ ] Fuzzy matching (optional)

---

## Phase 4: Repository Manager

**Goal:** Unified API for clone/find/list/remove

### Tasks

- [ ] **4.1** RepositoryManager (`manager.rs`)
  - [ ] Implement `clone()` - parse URL, check exists, clone
  - [ ] Implement `find()` - delegate to store
  - [ ] Implement `list()` - delegate to store
  - [ ] Implement `remove()` - delegate to store
  - [ ] Implement `from_config()` - load config and create

- [ ] **4.2** Dry-run support
  - [ ] Add dry-run flag to operations
  - [ ] Print what would happen without executing

---

## Phase 5: Configuration

**Goal:** Load settings from config files and environment

### Tasks

- [ ] **5.1** Config loading (`config.rs`)
  - [ ] Define `Config` struct
  - [ ] Load from `~/.config/helix/helix-repo.toml`
  - [ ] Support `HELIX_REPO_ROOT` env var
  - [ ] Default to `~/.cache/helix/repos`
  - [ ] Integrate with helix-config (if available)

---

## Phase 6: CLI

**Goal:** User-facing command-line interface

### Tasks

- [ ] **6.1** CLI structure (`cli/mod.rs`)
  - [ ] Define `Cli` struct with clap
  - [ ] Define `Commands` enum
  - [ ] Global `--json` flag

- [ ] **6.2** Clone command (`cli/clone.rs`)
  - [ ] Parse URL argument
  - [ ] Handle `--shallow`, `--branch` flags
  - [ ] Handle `--dry-run` flag
  - [ ] Output path on success

- [ ] **6.3** List command (`cli/list.rs`)
  - [ ] List all repositories
  - [ ] Handle `--filter` flag
  - [ ] JSON output format

- [ ] **6.4** Info command (`cli/info.rs`)
  - [ ] Show repository details
  - [ ] Handle not found error
  - [ ] JSON output format

- [ ] **6.5** Remove command (`cli/remove.rs`)
  - [ ] Delete repository
  - [ ] Handle `--dry-run` flag
  - [ ] Confirm before delete (optional)

- [ ] **6.6** Root command (`cli/root.rs`)
  - [ ] Print current root path

- [ ] **6.7** Main entry point (`main.rs`)
  - [ ] Parse args and dispatch to commands
  - [ ] Handle errors with exit codes

---

## Phase 7: Testing

**Goal:** Comprehensive test coverage

### Tasks

- [ ] **7.1** Unit tests
  - [ ] URL parsing (all formats)
  - [ ] Path calculation
  - [ ] Pattern matching
  - [ ] Config loading

- [ ] **7.2** Integration tests
  - [ ] Mock GitClient for manager tests
  - [ ] Temp directory for store tests

- [ ] **7.3** Documentation
  - [ ] Rustdoc for public API
  - [ ] Update README with examples

---

## Phase 8: Polish

**Goal:** Production-ready quality

### Tasks

- [ ] **8.1** Error messages
  - [ ] Clear, actionable error text
  - [ ] Suggest fixes where possible

- [ ] **8.2** Progress output
  - [ ] Show clone progress (pipe git output)
  - [ ] Spinner for long operations

- [ ] **8.3** Performance
  - [ ] Lazy directory scanning
  - [ ] Parallel listing (if needed)

---

## Future Phases

### Phase 9: Update Support (v2)
- [ ] `helix-repo update <name>` - pull latest
- [ ] `helix-repo update --all` - update all repos
- [ ] Track last update time

### Phase 10: VCS Abstraction (v2)
- [ ] Abstract `VcsClient` trait
- [ ] Mercurial support
- [ ] Per-URL VCS configuration

### Phase 11: Multiple Roots (v2)
- [ ] Support multiple root directories
- [ ] Primary vs secondary roots
- [ ] Search all roots for existing repos

---

## Dependencies

| Phase | Depends On |
|-------|------------|
| Phase 2 | Phase 1 |
| Phase 3 | Phase 1 |
| Phase 4 | Phase 2, Phase 3 |
| Phase 5 | Phase 1 |
| Phase 6 | Phase 4, Phase 5 |
| Phase 7 | Phase 6 |
| Phase 8 | Phase 7 |
