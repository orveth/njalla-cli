# Development Guide

This document outlines the phased development approach for njalla-cli.

## Philosophy

- **Incremental**: Each phase is independently testable and deployable
- **Idiomatic**: Follow Rust and Nix conventions
- **Verifiable**: Every phase has clear acceptance criteria
- **AI-friendly**: Clear structure for future maintenance by humans or AI

---

## Phase 1: Project Scaffolding ✓

**Goal**: Buildable project with CLI skeleton

**Tasks**:
- [x] Initialize Cargo project
- [x] Set up Nix flake with devShell
- [x] Create CLI structure with clap
- [x] Define error types
- [x] Define API types (no implementation)

---

## Phase 2: API Client ✓

**Goal**: Working API client with proper error handling

**Tasks**:
- [x] Implement `NjallaClient` struct
- [x] Implement `Request` method (JSON-RPC style)
- [x] Add proper error handling for API responses
- [x] Add request timeout and retry logic
- [x] Write unit tests with mocked responses
- [x] Add config file support

---

## Phase 3: Domain Commands ✓

**Goal**: Implement domain listing and search

**Tasks**:
- [x] Implement `domains` command (list-domains API)
- [x] Implement `search` command (find-domains API)
- [x] Implement `status` command (get-domain API)
- [x] Add JSON output formatting
- [x] Write integration tests

---

## Phase 4: Registration Flow ✓

**Goal**: Implement domain registration with polling

**Tasks**:
- [x] Implement `register` command
- [x] Add confirmation prompt (skip with --confirm)
- [x] Implement task polling (check-task API)
- [x] Add --wait flag for synchronous registration

---

## Phase 5: Validation & DNS ✓

**Goal**: Complete feature set

**Tasks**:
- [x] Implement `validate` command
- [x] Add DNS record listing (--dns flag on status)
- [x] Add structured validation output

---

## Phase 6: Wallet Commands ✓

**Goal**: Wallet management

**Tasks**:
- [x] Implement `wallet balance` command
- [x] Implement `wallet add-payment` command
- [x] Implement `wallet get-payment` command
- [x] Implement `wallet transactions` command

---

## Phase 7: Polish & Release

**Goal**: Production-ready release

**Tasks**:
- [ ] Add shell completions (bash, zsh, fish)
- [ ] Add man page generation
- [ ] Set up CI/CD (GitHub Actions)
- [ ] Tag v0.1.0 release

---

## Testing Strategy

### Unit Tests
- Located in `src/*.rs` as `#[cfg(test)]` modules
- Mock external dependencies
- Test error handling paths

### Integration Tests
- Located in `tests/`
- Use real API with test token (optional, skip in CI)
- Test full command flows

### Running Tests

```bash
# Unit tests only
cargo test --lib

# All tests (needs NJALLA_API_TOKEN for integration)
cargo test

# Skip integration tests in CI
cargo test --lib
```

---

## Code Style

### Rust
- Use `rustfmt` defaults
- Use `clippy` with `-D warnings`
- Use manual `Display` and `Error` impls for error types
- Use `?` for error propagation
- Document public APIs with `///`

### Nix
- Follow nixpkgs conventions
- Use `flake-utils` for multi-platform
- Pin nixpkgs to a specific commit

---

## File Structure

```
njalla-cli/
├── Cargo.toml
├── flake.nix
├── config.example.toml
├── README.md
├── docs/
│   ├── DEVELOPMENT.md
│   ├── API.md
│   └── ARCHITECTURE.md
├── src/
│   ├── main.rs          # CLI entrypoint
│   ├── client.rs        # API client
│   ├── config.rs        # Config file loading
│   ├── error.rs         # Error types
│   ├── types.rs         # API types
│   ├── output.rs        # Output formatting
│   └── commands/
│       ├── mod.rs
│       ├── domains.rs
│       ├── search.rs
│       ├── register.rs
│       ├── status.rs
│       ├── validate.rs
│       └── wallet.rs
└── tests/
```

---

## Current Phase

**Active**: Phase 7 - Polish & Release

**Completed**: Phases 1-6 (core functionality implemented)
