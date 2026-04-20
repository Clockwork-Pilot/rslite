# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: build_all](#feature-build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: clippy_checks](#feature-clippy_checks)
      - [clippy_checks](#clippy_checks)
      - [no_forbidden_allows_robust](#no_forbidden_allows_robust)
    - [Feature: fts5_no_libc_global](#feature-fts5_no_libc_global)
      - [fts5_no_libc_calls](#fts5_no_libc_calls)
    - [Feature: lib_exports](#feature-lib_exports)
      - [cdylib_symbols](#cdylib_symbols)
      - [no_rust_variadic_exports](#no_rust_variadic_exports)
      - [rslite_expected_exports](#rslite_expected_exports)
    - [Feature: pub_visibility](#feature-pub_visibility)
      - [rslite_uses_crust_core](#rslite_uses_crust_core)
    - [Feature: toolchain_version](#feature-toolchain_version)
      - [stable_toolchain_or_missing](#stable_toolchain_or_missing)
    - [Feature: workflow_constraints](#feature-workflow_constraints)
      - [workflow_spec](#workflow_spec)

## Features

### Feature: build_all
**Check that our rust codebase is healthy**

#### constraint_build_all
**Description:** Ensure our rust codebase is healthy

### Feature: clippy_checks
**Verify no clippy warnings in rust codebase**

**Goals:**
- Run cargo clippy on all packages
- Ensure all clippy lints pass with no warnings

#### clippy_checks
**Description:** Run cargo clippy and ensure no warnings reported

#### no_forbidden_allows_robust
**Description:** Structural: Robust whitespace-tolerant detection of forbidden clippy allows. Detects any formatting of allow() with forbidden lints, ignoring whitespace between allow and (.

### Feature: fts5_no_libc_global
**Ensure fts5.rs contains no libc:: calls (only external C FFI allowed)**

**Goals:**
- fts5.rs must not contain libc:: direct calls
- All C interop must go through safe wrappers or external C FFI only

#### fts5_no_libc_calls
**Description:** fts5.rs must not contain libc:: calls - use safe wrappers or external C FFI instead

### Feature: lib_exports
**All C FFI symbols correctly exported in rlib binaries and cdylib**

**Goals:**
- Verify all 10 exported C FFI symbols are present as global symbols in shell/rustfixture binaries and in the cdylib dynamic symbol table
- Each must have a C source file in c_code/
- Consolidates libs_appendf, libs_snprintf, libs_vsnprintf, libs_vmprintf

#### cdylib_symbols
**Description:** Behavioral: all 10 symbols must appear in dynamic symbol table (T) of the cdylib

#### no_rust_variadic_exports
**Description:** Negative: none of the 10 exported symbols must be Rust variadic extern fns — they live in C only

#### rslite_expected_exports
**Description:** Behavioral: all symbols from expected_exports.txt must appear in cdylib dynamic symbol table

### Feature: pub_visibility
**rslite crate must use crust-core (sqlite_noamalgam) directly, not rslite-raw**

**Goals:**
- Verify rslite crate uses crust-core (sqlite_noamalgam) for FFI
- Ensure no references to rslite-raw in Cargo.toml or source files

#### rslite_uses_crust_core
**Description:** Structural: rslite crate must not reference rslite-raw in its Cargo.toml or source files — it should use crust-core (sqlite_noamalgam) directly

### Feature: toolchain_version
**Enforce Rust toolchain versions. Stable for main code, nightly for shell and tests.**

**Goals:**
- rust-toolchain.toml must use stable channel

#### stable_toolchain_or_missing
**Description:** rust-toolchain.toml must either be missing or use stable channel

### Feature: workflow_constraints
**Workflow specs exist and constraint checker runs on PRs**

**Goals:**
- Workflow spec must exist at .github/workflows/spec.k.json
- A CI workflow must run check_spec_constraints.py against the workflow spec on pull_request

#### workflow_spec
**Description:** Workflow checks