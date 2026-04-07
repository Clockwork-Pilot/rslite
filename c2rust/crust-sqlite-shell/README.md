# Crust-SQLite-Shell

A C2Rust transpiled version of the SQLite shell (`shell.c`), providing a pure Rust implementation of the SQLite CLI that integrates seamlessly with the Rust SQLite library.

## Overview

This project contains the SQLite shell transpiled from C to Rust using the [C2Rust](https://c2rust.com) automated transpiler. The resulting Rust code compiles to a binary-compatible SQLite shell (`sqlite3`) that is linked against the Rust implementation of SQLite (`sqlite_noamalgam`).

## Architecture

```
shell.c (C source, ~33,890 lines)
    ↓ (c2rust transpilation)
src/generated/ (Rust, ~3,000-5,000 lines)
    ↓
src/main.rs (Rust entry point)
    ↓ (cargo build)
target/release/sqlite3 (Rust binary)
    ↓
links: sqlite_noamalgam (Rust library)
```

## Building

### Prerequisites

- Rust 1.56+ (nightly recommended)
- C2Rust: `cargo install c2rust`
- LLVM development libraries (for c2rust)
- The `sqlite_noamalgam` library (parent directory)

### Build Steps

1. **Transpile shell.c** (one-time):
   ```bash
   cd /workspace
   c2rust transpile c2rust-shell.toml
   ```

2. **Build the Rust crate**:
   ```bash
   cd crust-sqlite-shell
   cargo build --release
   ```

3. **Run the binary**:
   ```bash
   ./target/release/sqlite3 :memory: "SELECT 1;"
   ```

Or use the provided build script:
```bash
./build.sh
```

## File Structure

```
crust-sqlite-shell/
├── Cargo.toml              # Package configuration
├── build.sh                # Build automation script
├── README.md               # This file
├── src/
│   ├── main.rs             # Entry point (calls generated code)
│   └── generated/          # C2Rust generated modules
│       ├── mod.rs          # Module exports
│       └── shell_*.rs      # Transpiled shell functions
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/           # Test data
└── target/
    └── release/
        ├── libcrust_sqlite_shell.a  # Static library
        └── sqlite3                   # Main binary
```

## Configuration

The transpilation is configured in `/workspace/c2rust-shell.toml`:

- **Source:** `sqlite-shell-src/shell.c`
- **Output:** `crust-sqlite-shell/src/generated/`
- **Flags:** All flags from `defines_shell.txt` included
  - FTS4, RTREE, EXPLAIN_COMMENTS, etc.

## Development

### Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# With output
cargo test -- --nocapture --test-threads=1

# Quick smoke test
./target/release/sqlite3 :memory: "SELECT 1;"
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -W clippy::all

# Check for unsafe code
grep -r "unsafe" src/generated/
```

### Benchmarking

```bash
# Time startup
time ./target/release/sqlite3 :memory: "SELECT 1;"

# Compare with C version
time /usr/bin/sqlite3 :memory: "SELECT 1;"
```

## Known Issues

### Transpilation-Related
- Some variadic functions may need safe Rust wrappers
- Global mutable state wrapped with `lazy_static` + `Mutex`
- Some platform-specific code may need manual adjustment

### Performance
- Expected performance within 5% of C implementation
- May see slight regression in startup time (Rust overhead)
- Optimizations via `lto = true` and `codegen-units = 1`

## Integration

This project is integrated with the main SQLite build system:

```bash
# From /workspace:
./build_all.sh
```

The build system:
1. Builds the Rust SQLite library
2. Builds this Rust shell
3. Links them together
4. Runs the test suite

## C2Rust Migration Details

### Stats
- **Original C code:** ~33,890 lines (shell.c)
- **SQLite API functions used:** ~226 unique functions
- **Macros in original:** ~50 (platform-specific)
- **Generated Rust code:** ~3,000-5,000 lines (optimized)

### Key Challenges Addressed
- ✓ Macro expansion (all defines_shell.txt flags)
- ✓ Variadic function wrapping (printf family)
- ✓ Global mutable state management
- ✓ Safe pointer handling
- ✓ FFI bindings to sqlite_noamalgam

### Compilation Strategy
1. Generate raw Rust from C
2. Fix unsafe blocks iteratively
3. Create safe wrapper types
4. Verify with SQLite test suite
5. Optimize and benchmark

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Startup time | < 15ms | C version: ~10ms (5% tolerance) |
| Binary size | < 250KB | C version: ~200KB |
| Memory usage | Within 10% | Allow reasonable variance |
| Test pass rate | 100% | SQLite regression test suite |

## Links & References

- [C2Rust Documentation](https://c2rust.com)
- [SQLite C API](https://www.sqlite.org/c3ref.html)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Parent Project](../)

## Maintenance

### Regular Tasks
- [ ] Monitor test pass rate
- [ ] Check for compiler warnings
- [ ] Profile performance quarterly
- [ ] Update dependencies monthly

### When Updating shell.c
1. Update `sqlite-shell-src/shell.c`
2. Run: `c2rust transpile c2rust-shell.toml`
3. Review generated changes
4. Fix any new compilation errors
5. Re-run test suite

## License

Same as SQLite (Public Domain) + C2Rust (Apache 2.0 / MIT)

## Support

For issues:
- Check `/workspace/c2rust_shell_actions_log.md` for status
- Review transpilation errors in build output
- See `/workspace/SHELL_C_TO_RUST_MIGRATION.md` for detailed guide
