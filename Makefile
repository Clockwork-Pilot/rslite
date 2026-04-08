# Makefile for C2Rust SQLite Shell
# Main goals: c-tests, rust-tests
# Usage: make [DEBUG=1] [target]

.PHONY: help all clean clean-c-tests \
  c-tcl-tests crust-tcl-tests \
  verify-c-link verify-rust-link \
  test

# Configuration
SQLITE_SRC ?= /sqlite
PROJ := $(shell cd $(dir $(MAKEFILE_LIST)) && pwd)
NPROC := $(shell nproc)
DEBUG ?= 0
WITH_WARNINGS ?= 0

# Debug/Release selection
ifeq ($(DEBUG),1)
  MODE := debug
  CFLAGS := -g -O0
  RUST_LIB := $(PROJ)/target/debug/libsqlite_noamalgam.so
  RUST_SHELL := $(PROJ)/c2rust/target/debug/sqlite3
  RUST_TEST := $(PROJ)/c2rust/target/debug/rustfixture
else
  MODE := release
  CFLAGS := -O2
  RUST_LIB := $(PROJ)/target/release/libsqlite_noamalgam.so
  RUST_SHELL := $(PROJ)/c2rust/target/release/sqlite3
  RUST_TEST := $(PROJ)/c2rust/target/release/rustfixture
endif

# ============ Rust Source Tracking ============
RUST_SHELL_SOURCES := $(shell find $(PROJ)/c2rust/crust-sqlite-shell/src -name "*.rs" 2>/dev/null) \
                      $(shell find $(PROJ)/c2rust/crust-sqlite-shell -name "build.rs" -o -name "Cargo.toml" 2>/dev/null) \
                      $(PROJ)/Cargo.toml $(PROJ)/Cargo.lock

RUST_TEST_SOURCES := $(shell find $(PROJ)/c2rust/crust-tclsqlite/src -name "*.rs" 2>/dev/null) \
                     $(shell find $(PROJ)/c2rust/crust-tclsqlite -name "build.rs" -o -name "Cargo.toml" 2>/dev/null) \
                     $(PROJ)/Cargo.toml $(PROJ)/Cargo.lock

# ============ Rust Library Builds ============

$(PROJ)/target/debug/libsqlite_noamalgam.so:
	@echo "→ Building Rust library (debug)..."
	@cargo +nightly build -p crust-core --manifest-path $(PROJ)/Cargo.toml $(if $(filter 1,$(WITH_WARNINGS)),, --quiet)

$(PROJ)/target/release/libsqlite_noamalgam.so:
	@echo "→ Building Rust library (release)..."
	@cargo +nightly build --release -p crust-core --manifest-path $(PROJ)/Cargo.toml $(if $(filter 1,$(WITH_WARNINGS)),, --quiet)

# ============ C Build Targets ============

$(SQLITE_SRC)/sqlite3-c-debug: $(PROJ)/target/debug/libsqlite_noamalgam.so
	@echo "→ Building C shell (debug) linked against Rust library..."
	@cd $(SQLITE_SRC) && ./configure CFLAGS="-g -O0" \
		--fts3 --fts4 --fts5 --rtree --session --geopoly \
		--memsys3 --memsys5 --update-limit --dbpage --dbstat \
		--column-metadata $(if $(filter 1,$(WITH_WARNINGS)),, > /dev/null 2>&1)
	@cd $(SQLITE_SRC) && $(MAKE) shell.c sqlite3.h sqlite3ext.h $(if $(filter 1,$(WITH_WARNINGS)),, > /dev/null 2>&1)
	@cd $(SQLITE_SRC) && cc -g -O0 -o sqlite3 shell.c \
		-I. -I./src -I./ext/rtree -I./ext/icu -I./ext/fts3 -I./ext/session -I./ext/misc \
		-I/usr/include -DHAVE_READLINE=1 -DSQLITE_HAVE_ZLIB=1 \
		-L$(PROJ)/target/debug -Wl,-rpath,$(PROJ)/target/debug \
		-lsqlite_noamalgam -lreadline -lncurses -lm -lz $(if $(filter 1,$(WITH_WARNINGS)),, 2>&1)

$(SQLITE_SRC)/sqlite3-c-release: $(PROJ)/target/release/libsqlite_noamalgam.so
	@echo "→ Building C shell (release) linked against Rust library..."
	@cd $(SQLITE_SRC) && ./configure CFLAGS="-O2" \
		--fts3 --fts4 --fts5 --rtree --session --geopoly \
		--memsys3 --memsys5 --update-limit --dbpage --dbstat \
		--column-metadata $(if $(filter 1,$(WITH_WARNINGS)),, > /dev/null 2>&1)
	@cd $(SQLITE_SRC) && $(MAKE) shell.c sqlite3.h sqlite3ext.h $(if $(filter 1,$(WITH_WARNINGS)),, > /dev/null 2>&1)
	@cd $(SQLITE_SRC) && cc -O2 -o sqlite3 shell.c \
		-I. -I./src -I./ext/rtree -I./ext/icu -I./ext/fts3 -I./ext/session -I./ext/misc \
		-I/usr/include -DHAVE_READLINE=1 -DSQLITE_HAVE_ZLIB=1 \
		-L$(PROJ)/target/release -Wl,-rpath,$(PROJ)/target/release \
		-lsqlite_noamalgam -lreadline -lncurses -lm -lz $(if $(filter 1,$(WITH_WARNINGS)),, 2>&1)

# ============ Rust Shell & Test Builds ============

$(PROJ)/c2rust/target/debug/sqlite3 $(PROJ)/c2rust/target/debug/rustfixture: $(RUST_SHELL_SOURCES) $(RUST_TEST_SOURCES)
	@echo "→ Building Rust binaries (debug)..."
	@cargo +nightly build -p crust-sqlite-shell -p crust-tclsqlite --features crust-tclsqlite/test --manifest-path $(PROJ)/c2rust/Cargo.toml $(if $(filter 1,$(WITH_WARNINGS)),, --quiet)

$(PROJ)/c2rust/target/release/sqlite3 $(PROJ)/c2rust/target/release/rustfixture: $(RUST_SHELL_SOURCES) $(RUST_TEST_SOURCES)
	@echo "→ Building Rust binaries (release)..."
	@cargo +nightly build --release -p crust-sqlite-shell -p crust-tclsqlite --features crust-tclsqlite/test --manifest-path $(PROJ)/c2rust/Cargo.toml $(if $(filter 1,$(WITH_WARNINGS)),, --quiet)

# ============ Verification ============

verify-c-link:
	@if ! ldd $(SQLITE_SRC)/sqlite3 | grep -q libsqlite_noamalgam; then \
		echo "✗ ERROR: C shell is NOT linked to libsqlite_noamalgam"; \
		ldd $(SQLITE_SRC)/sqlite3; \
		exit 1; \
	fi
	@echo "✓ C shell linked to libsqlite_noamalgam"

verify-rust-link:
	@if ! ldd $(RUST_TEST) | grep -q libsqlite_noamalgam; then \
		echo "✗ ERROR: Rust test fixture is NOT linked to libsqlite_noamalgam"; \
		ldd $(RUST_TEST); \
		exit 1; \
	fi
	@echo "✓ Rust test fixture linked to libsqlite_noamalgam"

# ============ Main Test Targets ============

c-tcl-tests:
	@if [ ! -f $(SQLITE_SRC)/sqlite3 ] || [ $(RUST_LIB) -nt $(SQLITE_SRC)/sqlite3 ] || ! ldd $(SQLITE_SRC)/sqlite3 2>/dev/null | grep -q libsqlite_noamalgam; then \
		$(MAKE) $(if $(filter 1,$(DEBUG)),$(SQLITE_SRC)/sqlite3-c-debug,$(SQLITE_SRC)/sqlite3-c-release); \
	else \
		echo "→ C shell already built ($(MODE))"; \
	fi
	@echo "→ Running C TCL tests ($(MODE))"
	@echo "  Shell: $(SQLITE_SRC)/sqlite3"
	@echo "  Linked libraries:"
	@ldd $(SQLITE_SRC)/sqlite3 | grep -E "libsqlite|libc.so"
	@if ! ldd $(SQLITE_SRC)/sqlite3 | grep -q libsqlite_noamalgam; then \
		echo "✗ ERROR: C shell is NOT linked to libsqlite_noamalgam"; \
		exit 1; \
	fi
	$(MAKE) -C $(SQLITE_SRC) testrunner
	@echo "✓ C TCL tests ($(MODE)) passed"

crust-tcl-tests: $(RUST_SHELL) $(RUST_TEST)
	@echo "→ Running Rust TCL tests ($(MODE))"
	@echo "  Shell: $(RUST_SHELL)"
	@export PATH="$(dir $(RUST_TEST)):$(dir $(RUST_SHELL)):$$PATH" && \
		export LD_LIBRARY_PATH="$(dir $(RUST_LIB)):$$LD_LIBRARY_PATH" && \
		cd $(SQLITE_SRC) && "$(RUST_TEST)" test/testrunner.tcl --jobs $(NPROC)
	@echo "✓ Rust TCL tests ($(MODE)) passed"

# ============ Master Target ============

test: clean-c-tests
	@echo "→ Building & testing all..."
	@$(MAKE) DEBUG=0 c-tcl-tests > /dev/null
	@$(MAKE) DEBUG=0 crust-tcl-tests > /dev/null
	@echo "✓ All tests passed"

all: test

# ============ Cleanup ============

clean-c-tests:
	@echo "Cleaning C test artifacts from /sqlite..."
	cd $(SQLITE_SRC) && $(MAKE) distclean 2>/dev/null || true
	rm -f $(SQLITE_SRC)/sqlite3 $(SQLITE_SRC)/rustfixture
	@echo "✓ C tests cleaned"

clean:
	@echo "Cleaning all build artifacts..."
	cargo clean --manifest-path $(PROJ)/Cargo.toml
	cargo clean --manifest-path $(PROJ)/c2rust/Cargo.toml
	cd $(SQLITE_SRC) && $(MAKE) distclean 2>/dev/null || true
	@echo "✓ All artifacts cleaned"

# ============ Help ============

help:
	@echo "╔════════════════════════════════════════╗"
	@echo "║ C2Rust SQLite Shell Build & Test      ║"
	@echo "╚════════════════════════════════════════╝"
	@echo ""
	@echo "USAGE:"
	@echo "  make [DEBUG=1] [WITH_WARNINGS=1] [target]"
	@echo ""
	@echo "MAIN TARGETS:"
	@echo "  c-tcl-tests             Build & run C TCL tests (release by default)"
	@echo "  crust-tcl-tests         Build & run Rust TCL tests (release by default)"
	@echo "  test                    Build & run all tests (release)"
	@echo ""
	@echo "OPTIONS:"
	@echo "  DEBUG=1                 Use debug mode instead of release"
	@echo "  WITH_WARNINGS=1         Show build warnings (suppressed by default)"
	@echo ""
	@echo "EXAMPLES:"
	@echo "  make c-tcl-tests        Build & test C (release, quiet)"
	@echo "  make DEBUG=1 c-tcl-tests    Build & test C (debug, quiet)"
	@echo "  make WITH_WARNINGS=1 c-tcl-tests  Build & test C (release, show warnings)"
	@echo "  make crust-tcl-tests    Build & test Rust (release)"
	@echo "  make DEBUG=1 crust-tcl-tests Build & test Rust (debug)"
	@echo "  make test               Build & test all (release)"
	@echo ""
	@echo "CLEANUP:"
	@echo "  make clean              Clean all artifacts"
	@echo "  make clean-c-tests      Clean only C test artifacts"
	@echo ""
