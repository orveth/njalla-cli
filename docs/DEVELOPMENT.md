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

**Verification**:
```bash
nix develop              # Enter dev shell
cargo build              # Compiles without errors
cargo run -- --help      # Shows help text
cargo test               # All tests pass (trivial for now)
nix build                # Produces working binary
```

**Acceptance criteria**:
- `njalla --help` shows all planned commands
- `njalla domains` returns "not implemented" error
- All types compile and are well-documented

---

## Phase 2: API Client

**Goal**: Working API client with proper error handling

**Tasks**:
- [ ] Implement `NjallaClient` struct
- [ ] Implement `Request` method (JSON-RPC style)
- [ ] Add proper error handling for API responses
- [ ] Add request timeout and retry logic
- [ ] Write unit tests with mocked responses

**Verification**:
```bash
cargo test client        # Client tests pass
NJALLA_API_TOKEN=xxx cargo run -- domains  # Live API test
```

**Acceptance criteria**:
- Client handles auth errors gracefully
- Client handles network errors with retries
- Client handles malformed responses
- Unit tests cover happy path + error cases

---

## Phase 3: Domain Commands

**Goal**: Implement domain listing and search

**Tasks**:
- [ ] Implement `domains` command (list-domains API)
- [ ] Implement `search` command (find-domains API)
- [ ] Implement `status` command (get-domain API)
- [ ] Add table and JSON output formatting
- [ ] Write integration tests

**Verification**:
```bash
njalla domains           # Lists domains (table)
njalla domains -o json   # Lists domains (JSON)
njalla search test       # Shows available domains
njalla status example.com # Shows domain details
```

**Acceptance criteria**:
- Commands work with real API
- JSON output is valid and parseable
- Table output is human-readable
- Empty results handled gracefully

---

## Phase 4: Registration Flow

**Goal**: Implement domain registration with polling

**Tasks**:
- [ ] Implement `register` command
- [ ] Add confirmation prompt (skip with --confirm)
- [ ] Implement task polling (check-task API)
- [ ] Add --wait flag for synchronous registration
- [ ] Write integration tests (with mock for registration)

**Verification**:
```bash
njalla register test.com              # Interactive confirmation
njalla register test.com --confirm    # Skip confirmation
njalla register test.com --wait       # Wait for completion
```

**Acceptance criteria**:
- Registration initiates correctly
- Polling works with configurable timeout
- User sees clear progress feedback
- Handles insufficient balance error

---

## Phase 5: Validation & DNS

**Goal**: Complete feature set

**Tasks**:
- [ ] Implement `validate` command
- [ ] Add DNS record listing (--dns flag on status)
- [ ] Add structured validation output
- [ ] Write comprehensive tests

**Verification**:
```bash
njalla validate example.com           # Run all checks
njalla status example.com --dns       # Show DNS records
```

**Acceptance criteria**:
- Validation checks: exists, active, expiry, DNS accessible
- DNS records displayed correctly
- JSON output includes all validation details

---

## Phase 6: Polish & Release

**Goal**: Production-ready release

**Tasks**:
- [ ] Add shell completions (bash, zsh, fish)
- [ ] Add man page generation
- [ ] Set up CI/CD (GitHub Actions)
- [ ] Add integration tests in CI
- [ ] Write user documentation
- [ ] Tag v0.1.0 release

**Verification**:
```bash
njalla completions bash > /etc/bash_completion.d/njalla
man njalla
```

**Acceptance criteria**:
- CI passes on all commits
- Release binaries available
- Documentation complete
- Shell completions work

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
- Prefer `thiserror` for error types
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
├── Cargo.toml           # Package manifest
├── Cargo.lock           # Locked dependencies
├── flake.nix            # Nix flake
├── flake.lock           # Locked Nix inputs
├── README.md            # User documentation
├── docs/
│   ├── DEVELOPMENT.md   # This file
│   ├── API.md           # Njalla API reference
│   └── ARCHITECTURE.md  # Code architecture
├── src/
│   ├── main.rs          # CLI entrypoint
│   ├── lib.rs           # Library exports
│   ├── client.rs        # API client
│   ├── error.rs         # Error types
│   ├── types.rs         # API types
│   ├── output.rs        # Output formatting
│   └── commands/
│       ├── mod.rs       # Command exports
│       ├── domains.rs   # domains command
│       ├── search.rs    # search command
│       ├── register.rs  # register command
│       ├── status.rs    # status command
│       └── validate.rs  # validate command
└── tests/
    └── integration.rs   # Integration tests
```

---

## Current Phase

**Active**: Phase 1 - Project Scaffolding

**Next milestone**: Complete Phase 1 verification checklist
