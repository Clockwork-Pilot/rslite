# Makefile for C2Rust SQLite Shell
# Main goals: c-tests, rust-tests
# Usage: make [DEBUG=1] [target]

.PHONY: help all clean clean-c-tests \
  ensure-c-shell \
  c-quick-tests c-tcl-tests c-tests c-fuzz-tests c-prerelease-tests \
  crust-tcl-tests \
  ensure-binaries \
  patch-mk unpatch-mk \
  test

# Configuration
SQLITE_SRC ?= /sqlite
export RUSTUP_HOME ?= $(HOME)/.cargo/rustup
PROJ := $(shell cd $(dir $(MAKEFILE_LIST)) && pwd)
NPROC := $(shell nproc)
DEBUG ?= 0
VERBOSE ?= 0
ORIGINAL ?= 0
FEATURES ?=test,fts4,update_delete_limit

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

# Linking flags for Rust library (passed to SQLite configure/make when not using ORIGINAL)
RUST_LDFLAGS = -L$(dir $(RUST_LIB)) -Wl,-rpath,$(dir $(RUST_LIB))
RUST_LIBS = -lsqlite_noamalgam -lm -lz

# Make flags for SQLite tests - inject Rust library linking if not using ORIGINAL
# Monkey-patch /sqlite/main.mk at make-variable level: TESTFIXTURE_SRC1= strips sqlite3.c from
# testfixture's sources so the linker must resolve sqlite3 symbols from LDFLAGS.libsqlite3
# (i.e. libsqlite_noamalgam) rather than embedding the amalgamation directly.
MAKE_TEST_FLAGS = $(if $(filter 0,$(ORIGINAL)),LDFLAGS.math="-lm" LDFLAGS.libsqlite3="$(RUST_LDFLAGS) $(RUST_LIBS)" TESTFIXTURE_SRC1=,)

# Fuzz-specific flags: when ORIGINAL=0, MAKE_TEST_FLAGS plus a CFLAGS override that prepends a
# generated stub dir to the include path.  The stub dir contains an empty sqlite3.c, so
# sessionfuzz/optfuzz's '#include "sqlite3.c"' resolves to that empty file instead of the real
# amalgamation.  Declarations are injected via -include sqlite3.h / -include stdlib.h.
# The stub dir lives under target/ (gitignored) and is created by build-fuzz-tests.
# When ORIGINAL=1, no flags needed — fuzz binaries embed sqlite3.c directly as usual.
FUZZ_STUB_DIR := $(PROJ)/target/fuzz-stubs
ifeq ($(ORIGINAL),1)
MAKE_FUZZ_FLAGS =
else
MAKE_FUZZ_FLAGS = $(MAKE_TEST_FLAGS) CFLAGS="$(CFLAGS) -I$(FUZZ_STUB_DIR) -include sqlite3.h -include stdlib.h"
endif

# Implementation type string for display
IMPL_TYPE = $(if $(filter 1,$(ORIGINAL)),original C,Rust-linked)

# ============ Test Binary Verification Lists ============
# Define binary names (without path), used with target-specific VERIFY_LINKAGE assignments

# ============ Rust Source Tracking ============
RUST_LIB_SOURCES := $(shell find $(PROJ)/crates/crust-core -name "*.rs" 2>/dev/null) \
                    $(shell find $(PROJ)/crates/crust-core -name "Cargo.toml" 2>/dev/null) \
                    $(shell find $(PROJ)/c_code -name "*.c" 2>/dev/null) \
                    $(PROJ)/Cargo.toml $(PROJ)/Cargo.lock $(PROJ)/Makefile

RUST_SHELL_SOURCES := $(shell find $(PROJ)/c2rust/crust-sqlite-shell/src -name "*.rs" 2>/dev/null) \
                      $(shell find $(PROJ)/c2rust/crust-sqlite-shell -name "build.rs" -o -name "Cargo.toml" 2>/dev/null) \
                      $(PROJ)/Cargo.toml $(PROJ)/Cargo.lock

RUST_TEST_SOURCES := $(shell find $(PROJ)/c2rust/crust-tclsqlite/src -name "*.rs" 2>/dev/null) \
                     $(shell find $(PROJ)/c2rust/crust-tclsqlite -name "build.rs" -o -name "Cargo.toml" 2>/dev/null) \
                     $(PROJ)/Cargo.toml $(PROJ)/Cargo.lock

# ============ Rust Library Builds ============

$(PROJ)/target/debug/libsqlite_noamalgam.so: $(RUST_LIB_SOURCES)
	@echo "→ Building Rust library (debug)..."
	@cargo build --manifest-path $(PROJ)/Cargo.toml \
		$(if $(FEATURES),--features $(FEATURES)) \
		$(if $(filter 1,$(VERBOSE)),, --quiet)

$(PROJ)/target/release/libsqlite_noamalgam.so: $(RUST_LIB_SOURCES)
	@echo "→ Building Rust library (release)..."
	@cargo build --release --manifest-path $(PROJ)/Cargo.toml \
		$(if $(FEATURES),--features $(FEATURES)) \
		$(if $(filter 1,$(VERBOSE)),, --quiet)

# ============ C Build Targets ============

$(SQLITE_SRC)/sqlite3-orig:
	@echo "→ Building original C shell from $(SQLITE_SRC)..."
	@cd $(SQLITE_SRC) && ./configure CFLAGS="$(CFLAGS)" \
		--fts3 --fts4 --fts5 --rtree --session --geopoly \
		--update-limit --dbpage --dbstat \
		$(if $(filter 1,$(VERBOSE)),, > /dev/null 2>&1)
	@cd $(SQLITE_SRC) && $(MAKE) sqlite3 $(if $(filter 1,$(VERBOSE)),, > /dev/null 2>&1)
	@echo "✓ Original C shell built at $(SQLITE_SRC)/sqlite3"

$(SQLITE_SRC)/sqlite3-c: $(RUST_LIB)
	@echo "→ Building C shell ($(MODE)) linked against Rust library..."
	@cd $(SQLITE_SRC) && ./configure CFLAGS="$(CFLAGS)" LDFLAGS="$(RUST_LDFLAGS)" LIBS="$(RUST_LIBS)" \
		--fts3 --fts4 --fts5 --rtree --session --geopoly \
		--update-limit --dbpage --dbstat \
		$(if $(filter 1,$(VERBOSE)),, > /dev/null 2>&1)
	@cd $(SQLITE_SRC) && $(MAKE) shell.c sqlite3.h sqlite3ext.h $(if $(filter 1,$(VERBOSE)),, > /dev/null 2>&1)
	@cd $(SQLITE_SRC) && cc $(CFLAGS) -o sqlite3 shell.c \
		-I. -I./src -I./ext/rtree -I./ext/icu -I./ext/fts3 -I./ext/session -I./ext/misc \
		-I/usr/include -DHAVE_READLINE=1 -DSQLITE_HAVE_ZLIB=1 -DSQLITE_ENABLE_DBPAGE_VTAB \
		-L$(dir $(RUST_LIB)) -Wl,-rpath,$(dir $(RUST_LIB)) \
		-lsqlite_noamalgam -lreadline -lncurses -lm -lz $(if $(filter 1,$(VERBOSE)),, > /dev/null 2>&1)
	@echo "✓ C shell installed at $(SQLITE_SRC)/sqlite3"

# ============ Rust Shell & Test Builds ============

$(PROJ)/c2rust/target/debug/sqlite3: $(RUST_SHELL_SOURCES)
	@echo "→ Building Rust shell (debug)..."
	@cargo +nightly build -p crust-sqlite-shell --manifest-path $(PROJ)/c2rust/Cargo.toml $(if $(filter 1,$(VERBOSE)),, --quiet)

$(PROJ)/c2rust/target/debug/rustfixture: $(RUST_TEST_SOURCES)
	@echo "→ Building Rust test fixture (debug)..."
	@cargo +nightly build -p crust-tclsqlite --features crust-tclsqlite/test --manifest-path $(PROJ)/c2rust/Cargo.toml $(if $(filter 1,$(VERBOSE)),, --quiet)

$(PROJ)/c2rust/target/release/sqlite3: $(RUST_SHELL_SOURCES)
	@echo "→ Building Rust shell (release)..."
	@cargo +nightly build --release -p crust-sqlite-shell --manifest-path $(PROJ)/c2rust/Cargo.toml $(if $(filter 1,$(VERBOSE)),, --quiet)

$(PROJ)/c2rust/target/release/rustfixture: $(RUST_TEST_SOURCES)
	@echo "→ Building Rust test fixture (release)..."
	@cargo +nightly build --release -p crust-tclsqlite --features crust-tclsqlite/test --manifest-path $(PROJ)/c2rust/Cargo.toml $(if $(filter 1,$(VERBOSE)),, --quiet)

# ============ Verification ============

# Verify all binaries are linked to Rust library (fails if any not linked or missing)
# This ensures all test executables use the Rust-translated SQLite, not embedded C
verify-linkage:
	@if [ $(ORIGINAL) -eq 0 ] && [ -n "$(VERIFY_LINKAGE)" ]; then \
		echo "→ Verifying C binaries linked to Rust library..."; \
		for binary in $(VERIFY_LINKAGE); do \
			if [ ! -f $$binary ]; then \
				echo "✗ ASSERTION FAILED: Binary not found at $$binary"; \
				exit 1; \
			fi; \
			if ! ldd $$binary 2>/dev/null | grep -q libsqlite_noamalgam; then \
				echo "✗ ASSERTION FAILED: $$binary is NOT linked to libsqlite_noamalgam"; \
				ldd $$binary 2>/dev/null; \
				exit 1; \
			fi; \
			echo "  ✓ $$binary linked to libsqlite_noamalgam"; \
		done; \
	fi

# Verify Rust binaries are properly linked to rlib dependencies
# Uses nm to check for Rust crate symbols (sqlite_noamalgam)
verify-rust-linkage:
	@if [ -n "$(VERIFY_RUST_LINKAGE)" ]; then \
		echo "→ Verifying Rust binaries linked to rlib..."; \
		for binary in $(VERIFY_RUST_LINKAGE); do \
			if [ ! -f $$binary ]; then \
				echo "✗ ASSERTION FAILED: Binary not found at $$binary"; \
				exit 1; \
			fi; \
			if ! nm $$binary 2>/dev/null | grep -q "sqlite_noamalgam"; then \
				echo "✗ ASSERTION FAILED: $$binary is NOT linked to Rust rlib"; \
				nm $$binary 2>/dev/null | head -20; \
				exit 1; \
			fi; \
			echo "  ✓ $$binary linked to Rust rlib"; \
		done; \
	fi

# ============ Main Test Targets ============

# Helper to ensure C shell is built
ensure-c-shell:
	@if [ $(ORIGINAL) -ne 1 ]; then \
		if [ ! -f $(SQLITE_SRC)/sqlite3 ] || [ $(RUST_LIB) -nt $(SQLITE_SRC)/sqlite3 ] || ! ldd $(SQLITE_SRC)/sqlite3 2>/dev/null | grep -q libsqlite_noamalgam; then \
			echo "→ Building C shell ($(MODE)) linked against Rust library..."; \
			$(MAKE) $(SQLITE_SRC)/sqlite3-c; \
		else \
			echo "→ C shell already built ($(MODE))"; \
			echo "  Shell: $(SQLITE_SRC)/sqlite3"; \
		fi; \
	else \
		if [ ! -f $(SQLITE_SRC)/sqlite3 ] || [ ! -f $(SQLITE_SRC)/Makefile ]; then \
			echo "→ Building original C shell ($(MODE))..."; \
			$(MAKE) $(SQLITE_SRC)/sqlite3-orig; \
		else \
			echo "→ Original C shell already built at $(SQLITE_SRC)/sqlite3"; \
		fi; \
	fi

# Helper to ensure Rust shell is copied to /sqlite
ensure-rust-shell: $(RUST_SHELL)
	@echo "→ Installing Rust shell to $(SQLITE_SRC)/sqlite3..."
	cp $(RUST_SHELL) $(SQLITE_SRC)/sqlite3
	@echo "✓ Rust shell installed at $(SQLITE_SRC)/sqlite3"

# ============ SQLite Build Targets ============

build-quicktest:
	$(MAKE) -C $(SQLITE_SRC) $(MAKE_TEST_FLAGS) testfixture

build-tcl-tests:
	$(MAKE) -C $(SQLITE_SRC) $(MAKE_TEST_FLAGS) testfixture

build-all-tests:
	$(MAKE) -C $(SQLITE_SRC) $(MAKE_TEST_FLAGS) testfixture

ifeq ($(ORIGINAL),1)
build-fuzz-tests:
	rm -f $(SQLITE_SRC)/fuzzcheck $(SQLITE_SRC)/sessionfuzz
	$(MAKE) -C $(SQLITE_SRC) $(MAKE_FUZZ_FLAGS) fuzzcheck sessionfuzz
else
build-fuzz-tests: patch-mk
	mkdir -p $(FUZZ_STUB_DIR) && touch $(FUZZ_STUB_DIR)/sqlite3.c
	rm -f $(SQLITE_SRC)/fuzzcheck $(SQLITE_SRC)/sessionfuzz
	$(MAKE) -C $(SQLITE_SRC) $(MAKE_FUZZ_FLAGS) fuzzcheck sessionfuzz
endif

build-prerelease-tests:
	$(MAKE) -C $(SQLITE_SRC) $(MAKE_TEST_FLAGS) testfixture

# ============ C Test Targets (build -> verify -> run inline) ============

c-quick-tests: VERIFY_LINKAGE = $(SQLITE_SRC)/sqlite3 $(SQLITE_SRC)/testfixture
c-quick-tests: ensure-c-shell build-quicktest verify-linkage
	@echo "→ Running C quick tests ($(IMPL_TYPE), $(MODE))..."
	@echo "  Shell: $(SQLITE_SRC)/sqlite3"
	@echo "  Test Fixture: $(SQLITE_SRC)/testfixture"
	cd $(SQLITE_SRC) && $(SQLITE_SRC)/testfixture test/extraquick.test
	@echo "✓ C quick tests ($(MODE)) passed"

c-tcl-tests: VERIFY_LINKAGE = $(SQLITE_SRC)/sqlite3 $(SQLITE_SRC)/testfixture
c-tcl-tests: ensure-c-shell build-tcl-tests verify-linkage
	@echo "→ Running C TCL tests ($(IMPL_TYPE), $(MODE))..."
	@echo "  Shell: $(SQLITE_SRC)/sqlite3"
	@echo "  Test Fixture: $(SQLITE_SRC)/testfixture"
	cd $(SQLITE_SRC) && $(SQLITE_SRC)/testfixture test/testrunner.tcl --jobs $(NPROC)
	@echo "✓ C TCL tests ($(MODE)) passed"

c-tests: VERIFY_LINKAGE = $(SQLITE_SRC)/sqlite3 $(SQLITE_SRC)/testfixture
c-tests: ensure-c-shell build-all-tests verify-linkage
	@echo "→ Running C all tests ($(IMPL_TYPE), $(MODE))..."
	@echo "  Shell: $(SQLITE_SRC)/sqlite3"
	@echo "  Test Fixture: $(SQLITE_SRC)/testfixture"
	cd $(SQLITE_SRC) && $(SQLITE_SRC)/testfixture test/all.tcl
	@echo "✓ C all tests ($(MODE)) passed"

ifeq ($(ORIGINAL),1)
c-fuzz-tests: VERIFY_LINKAGE =
else
# Verify ALL fuzz test binaries link to Rust library (will fail if build-fuzz-tests doesn't link properly)
c-fuzz-tests: VERIFY_LINKAGE = $(SQLITE_SRC)/sqlite3 $(SQLITE_SRC)/fuzzcheck $(SQLITE_SRC)/sessionfuzz
endif
c-fuzz-tests: ensure-c-shell build-fuzz-tests verify-linkage
	@echo "→ Running C fuzz tests ($(IMPL_TYPE), $(MODE))..."
	@echo "  Shell: $(SQLITE_SRC)/sqlite3"
	@echo "  Fuzz Binaries: $(SQLITE_SRC)/fuzzcheck $(SQLITE_SRC)/sessionfuzz"
	cd $(SQLITE_SRC) && $(SQLITE_SRC)/fuzzcheck test/fuzzdata*.db && $(SQLITE_SRC)/sessionfuzz run test/sessionfuzz-data*.db
	@echo "✓ C fuzz tests ($(MODE)) passed"

c-prerelease-tests: VERIFY_LINKAGE = $(addprefix $(SQLITE_SRC)/,$(VERIFY_PRERELEASE))
c-prerelease-tests: ensure-c-shell build-prerelease-tests verify-linkage
	@echo "→ Running C prerelease tests ($(IMPL_TYPE), $(MODE))..."
	@echo "  Shell: $(SQLITE_SRC)/sqlite3"
	@echo "  Test Fixture: $(SQLITE_SRC)/testfixture"
	cd $(SQLITE_SRC) && $(SQLITE_SRC)/testfixture test/releasetest.tcl
	@echo "✓ C prerelease tests ($(MODE)) passed"

crust-tcl-tests: VERIFY_RUST_LINKAGE = $(RUST_SHELL) $(RUST_TEST)
crust-tcl-tests: $(RUST_SHELL) $(RUST_TEST) ensure-rust-shell verify-rust-linkage
	@echo "→ Running Rust TCL tests ($(MODE))"
	@echo "  Shell: $(SQLITE_SRC)/sqlite3"
	@echo "  Test Fixture: $(RUST_TEST)"
	cd $(SQLITE_SRC) && "$(RUST_TEST)" test/testrunner.tcl --jobs $(NPROC)
	@echo "✓ Rust TCL tests ($(MODE)) passed"

# ============ Master Target ============

test: c-tcl-tests
	@echo "✓ Tcl tests passed"

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

# ============ SQLite main.mk Monkey Patch ============
# Inline forward/reverse patch for /sqlite/main.mk.
# The same fix is already applied at make-variable level via MAKE_TEST_FLAGS (TESTFIXTURE_SRC1=),
# so these targets are only needed if you want to persist the change directly in the file.
#
# Problem: TESTFIXTURE_SRC1 = sqlite3.c causes testfixture to embed the amalgamation,
# so the linker satisfies all sqlite3 symbols from it and never links libsqlite_noamalgam.
# Fix: clear TESTFIXTURE_SRC1 so LDFLAGS.libsqlite3 (= -lsqlite_noamalgam) must be used.

define MAIN_MK_PATCH
--- a/main.mk
+++ b/main.mk
@@ -1802,3 +1802,3 @@
 TESTFIXTURE_SRC0 = $$(TESTSRC2) $$(libsqlite3.LIB)
-TESTFIXTURE_SRC1 = sqlite3.c
+TESTFIXTURE_SRC1 =
 TESTFIXTURE_SRC = $$(TESTSRC) $$(TOP)/src/tclsqlite.c
@@ -2277,4 +2277,4 @@
-fuzzcheck$$(T.exe):	$$(FUZZCHECK_SRC) sqlite3.c sqlite3.h $$(FUZZCHECK_DEP)
-	$$(T.link) -o $$@ $$(FUZZCHECK_OPT) $$(FUZZCHECK_SRC) sqlite3.c $$(LDFLAGS.libsqlite3)
+fuzzcheck$$(T.exe):	$$(FUZZCHECK_SRC) sqlite3.h $$(FUZZCHECK_DEP)
+	$$(T.link) -o $$@ $$(FUZZCHECK_OPT) $$(FUZZCHECK_SRC) $$(LDFLAGS.libsqlite3)
 fuzzy: fuzzcheck$$(T.exe)
 xbin: fuzzcheck$$(T.exe)
endef
export MAIN_MK_PATCH

patch-mk:
	@printf '%s\n' "$$MAIN_MK_PATCH" | patch -N -r - $(SQLITE_SRC)/main.mk && \
		echo "✓ Patched $(SQLITE_SRC)/main.mk (TESTFIXTURE_SRC1 cleared)" || \
		echo "→ $(SQLITE_SRC)/main.mk already patched or differs — skipping"

unpatch-mk:
	@printf '%s\n' "$$MAIN_MK_PATCH" | patch -N -R -r - $(SQLITE_SRC)/main.mk && \
		echo "✓ Reverted $(SQLITE_SRC)/main.mk (TESTFIXTURE_SRC1 = sqlite3.c restored)" || \
		echo "→ $(SQLITE_SRC)/main.mk not in patched state — skipping"

# ============ Help ============

help:
	@echo "╔════════════════════════════════════════╗"
	@echo "║ C2Rust SQLite Shell Build & Test      ║"
	@echo "╚════════════════════════════════════════╝"
	@echo ""
	@echo "USAGE:"
	@echo "  make [DEBUG=1] [VERBOSE=1] [ORIGINAL=1] [target]"
	@echo ""
	@echo "C TEST TARGETS (linked to Rust library by default):"
	@echo "  c-quick-tests           Quick sanity checks (seconds)"
	@echo "  c-tcl-tests             Full TCL test suite (parallel, ~40s)"
	@echo "  c-tests                 Most/all TCL tests"
	@echo "  c-prerelease-tests      Pre-release tests"
	@echo "  c-fuzz-tests            Fuzz testing (random inputs)"
	@echo ""
	@echo "RUST TEST TARGETS:"
	@echo "  crust-tcl-tests         Build & run Rust TCL tests"
	@echo ""
	@echo "MASTER TARGETS:"
	@echo "  test                    Build & run all tests (release)"
	@echo ""
	@echo "OPTIONS:"
	@echo "  DEBUG=1                 Use debug mode instead of release"
	@echo "  VERBOSE=1               Show full build output & warnings (quiet by default)"
	@echo "  ORIGINAL=1              Run C tests with original C sqlite3 (no Rust library)"
	@echo "  FEATURES=feat1,feat2    Cargo features to enable in the Rust library"
	@echo ""
	@echo "EXAMPLES:"
	@echo "  make c-quick-tests      Quick test C (quiet)"
	@echo "  make VERBOSE=1 c-tcl-tests  Full C tests with build output"
	@echo "  make DEBUG=1 c-fuzz-tests Fuzz test C (debug)"
	@echo "  make crust-tcl-tests    Test Rust (quiet)"
	@echo "  make test               Build & test all"
	@echo ""
	@echo "CLEANUP:"
	@echo "  make clean              Clean all artifacts"
	@echo "  make clean-c-tests      Clean only C test artifacts"
	@echo ""
